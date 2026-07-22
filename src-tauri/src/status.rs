use crate::agent::Agent;
use crate::error::AppError;
use serde::Serialize;
use std::path::PathBuf;
use std::process::{Command, Stdio};

const CLAUDE_LATEST_URL: &str = "https://downloads.claude.ai/claude-code-releases/stable";
// api.github.com은 미인증 60회/시 제한이 있어 불안정 → github.com의 latest 리다이렉트
// 최종 URL(.../tag/rust-vX.Y.Z)에서 버전을 뽑는다.
const CODEX_LATEST_URL: &str = "https://github.com/openai/codex/releases/latest";

/// 홈베이스의 "내 에이전트" 패널용 — 설치·버전·로그인 상태(로컬, 빠름).
#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AgentStatus {
    pub installed: bool,
    /// 설치된 버전 문자열(원문, 예: "2.1.216 (Claude Code)")
    pub version: Option<String>,
    pub logged_in: bool,
    pub path: Option<String>,
}

#[tauri::command]
pub async fn agent_status(agent: String) -> Result<AgentStatus, AppError> {
    let agent = Agent::from_id(&agent)?;
    tauri::async_runtime::spawn_blocking(move || {
        let Some(path) = crate::detect::agent_bin(agent) else {
            return AgentStatus {
                installed: false,
                version: None,
                logged_in: false,
                path: None,
            };
        };
        let version = crate::detect::agent_version(&path);
        let logged_in = crate::login::is_logged_in(agent);
        AgentStatus {
            installed: true,
            version,
            logged_in,
            path: Some(path.display().to_string()),
        }
    })
    .await
    .map_err(|e| AppError::generic(e.to_string()))
}

/// 에이전트의 최신 배포 버전(네트워크). 실패·오프라인이면 None을 돌려 홈이 조용히 넘어가게 한다.
#[tauri::command]
pub async fn latest_agent_version(agent: String) -> Result<Option<String>, AppError> {
    let agent = Agent::from_id(&agent)?;
    tauri::async_runtime::spawn_blocking(move || Ok(fetch_latest(agent)))
        .await
        .map_err(|e| AppError::generic(e.to_string()))?
}

fn fetch_latest(agent: Agent) -> Option<String> {
    match agent {
        Agent::ClaudeCode => {
            // 버전 문자열만 반환하는 엔드포인트
            let out = curl(&[CLAUDE_LATEST_URL])?;
            let v = out.trim().to_string();
            semver_of(&v)?;
            Some(v)
        }
        Agent::Codex => {
            // latest 리다이렉트의 최종 URL(.../tag/rust-v0.145.0)에서 버전을 뽑는다
            let final_url = curl_effective_url(CODEX_LATEST_URL)?;
            semver_of(&final_url).map(|s| s.to_string())
        }
    }
}

fn curl(extra_args: &[&str]) -> Option<String> {
    let out = curl_cmd().args(extra_args).output().ok()?;
    if !out.status.success() {
        return None;
    }
    Some(String::from_utf8_lossy(&out.stdout).to_string())
}

/// 리다이렉트를 따라간 최종 URL만 반환한다(본문은 버림).
fn curl_effective_url(url: &str) -> Option<String> {
    let out = curl_cmd()
        .args(["-o", null_device(), "-w", "%{url_effective}", url])
        .output()
        .ok()?;
    if !out.status.success() {
        return None;
    }
    Some(String::from_utf8_lossy(&out.stdout).trim().to_string())
}

fn curl_cmd() -> Command {
    let mut cmd = Command::new(curl_path());
    cmd.args(["-fsSL", "--max-time", "10"]).stdin(Stdio::null());
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        cmd.creation_flags(0x0800_0000);
    }
    cmd
}

fn null_device() -> &'static str {
    if cfg!(windows) { "NUL" } else { "/dev/null" }
}

fn curl_path() -> PathBuf {
    if cfg!(windows) {
        let root = std::env::var_os("SystemRoot")
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from("C:\\Windows"));
        root.join("System32").join("curl.exe")
    } else {
        PathBuf::from("/usr/bin/curl")
    }
}

/// 문자열에서 첫 `x.y.z`를 뽑는다(정규식 없이).
fn semver_of(text: &str) -> Option<&str> {
    let bytes = text.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i].is_ascii_digit() {
            let start = i;
            let mut dots = 0;
            while i < bytes.len() && (bytes[i].is_ascii_digit() || bytes[i] == b'.') {
                if bytes[i] == b'.' {
                    dots += 1;
                }
                i += 1;
            }
            if dots >= 2 {
                return Some(&text[start..i]);
            }
        } else {
            i += 1;
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::semver_of;

    #[test]
    fn extract_semver() {
        assert_eq!(semver_of("2.1.216 (Claude Code)"), Some("2.1.216"));
        assert_eq!(semver_of("codex-cli 0.145.0"), Some("0.145.0"));
        assert_eq!(semver_of("rust-v0.145.0"), Some("0.145.0"));
        assert_eq!(semver_of("no version here"), None);
    }
}

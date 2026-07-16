use serde::{Deserialize, Serialize};
use std::io::{BufRead, BufReader, Write};
use std::process::{Child, Command, Stdio};
use std::sync::Mutex;
use tauri::ipc::Channel;
use tauri::State;

/// `claude auth status --json` 출력 (비밀값 없음 — 상태 메타데이터만)
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LoginStatus {
    pub logged_in: bool,
    #[serde(default)]
    pub auth_method: Option<String>,
    #[serde(default)]
    pub email: Option<String>,
    #[serde(default)]
    pub subscription_type: Option<String>,
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum LoginEvent {
    /// 브라우저에서 열어야 할 로그인 URL
    Url { url: String },
    Log { line: String },
    /// 로그인 프로세스 종료 (성공 여부는 auth status 재확인으로 판정)
    Exit { success: bool },
}

#[derive(Default)]
pub struct LoginSession(pub Mutex<Option<Child>>);

#[tauri::command]
pub async fn login_status() -> Result<LoginStatus, String> {
    tauri::async_runtime::spawn_blocking(|| {
        let bin = crate::detect::claude_bin()
            .ok_or_else(|| "클로드 코드가 아직 설치되어 있지 않아요.".to_string())?;
        let out = Command::new(&bin)
            .args(["auth", "status", "--json"])
            .stdin(Stdio::null())
            .output()
            .map_err(|e| format!("로그인 상태를 확인하지 못했어요: {e}"))?;
        let text = String::from_utf8_lossy(&out.stdout);
        serde_json::from_str(text.trim())
            .map_err(|_| "로그인 상태 응답을 이해하지 못했어요.".to_string())
    })
    .await
    .map_err(|e| e.to_string())?
}

/// `claude auth login`을 백그라운드로 시작한다.
/// CLI가 브라우저를 직접 열고, 대비용 URL도 이벤트로 전달한다.
/// 이후 사용자가 브라우저에서 받은 코드를 submit_login_code로 넘기면 완료된다.
#[tauri::command]
pub fn start_login(
    session: State<'_, LoginSession>,
    on_event: Channel<LoginEvent>,
) -> Result<(), String> {
    let bin = crate::detect::claude_bin()
        .ok_or_else(|| "클로드 코드가 아직 설치되어 있지 않아요.".to_string())?;

    let mut guard = session.0.lock().unwrap();
    if let Some(mut old) = guard.take() {
        let _ = old.kill();
        let _ = old.wait();
    }

    let mut child = Command::new(&bin)
        .args(["auth", "login", "--claudeai"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("로그인을 시작하지 못했어요: {e}"))?;

    let stdout = child.stdout.take().expect("stdout is piped");
    let stderr = child.stderr.take().expect("stderr is piped");
    *guard = Some(child);
    drop(guard);

    for (reader, is_stdout) in [
        (Box::new(stdout) as Box<dyn std::io::Read + Send>, true),
        (Box::new(stderr), false),
    ] {
        let ch = on_event.clone();
        std::thread::spawn(move || {
            for line in BufReader::new(reader).lines().map_while(Result::ok) {
                if is_stdout {
                    if let Some(url) = extract_url(&line) {
                        let _ = ch.send(LoginEvent::Url { url });
                        continue;
                    }
                }
                if !line.trim().is_empty() {
                    let _ = ch.send(LoginEvent::Log { line });
                }
            }
        });
    }
    Ok(())
}

/// 브라우저 로그인 후 받은 확인 코드를 CLI에 전달하고 종료를 기다린다.
#[tauri::command]
pub async fn submit_login_code(
    session: State<'_, LoginSession>,
    on_event: Channel<LoginEvent>,
    code: String,
) -> Result<(), String> {
    let mut child = session
        .0
        .lock()
        .unwrap()
        .take()
        .ok_or_else(|| "진행 중인 로그인이 없어요. 처음부터 다시 시도해 주세요.".to_string())?;

    tauri::async_runtime::spawn_blocking(move || {
        if let Some(stdin) = child.stdin.as_mut() {
            stdin
                .write_all(format!("{}\n", code.trim()).as_bytes())
                .map_err(|e| format!("코드를 전달하지 못했어요: {e}"))?;
        }
        // 코드가 틀리면 CLI가 다시 물으며 종료되지 않으므로 대기 시간을 제한한다
        let success = wait_with_timeout(&mut child, std::time::Duration::from_secs(30));
        if !success {
            let _ = child.kill();
            let _ = child.wait();
        }
        let _ = on_event.send(LoginEvent::Exit { success });
        if success {
            Ok(())
        } else {
            Err("코드 확인에 실패했어요. 코드를 다시 복사해서 시도해 주세요.".to_string())
        }
    })
    .await
    .map_err(|e| e.to_string())?
}

#[tauri::command]
pub fn cancel_login(session: State<'_, LoginSession>) {
    if let Some(mut child) = session.0.lock().unwrap().take() {
        let _ = child.kill();
        let _ = child.wait();
    }
}

fn wait_with_timeout(child: &mut Child, timeout: std::time::Duration) -> bool {
    let start = std::time::Instant::now();
    loop {
        match child.try_wait() {
            Ok(Some(status)) => return status.success(),
            Ok(None) => {
                if start.elapsed() > timeout {
                    return false;
                }
                std::thread::sleep(std::time::Duration::from_millis(200));
            }
            Err(_) => return false,
        }
    }
}

/// 터미널 하이퍼링크 이스케이프(OSC 8)가 섞인 줄에서 로그인 URL만 뽑아낸다
fn extract_url(line: &str) -> Option<String> {
    let start = line.find("https://")?;
    let rest = &line[start..];
    let end = rest
        .find(|c: char| c.is_whitespace() || c == '\x1b' || c == '\x07')
        .unwrap_or(rest.len());
    Some(rest[..end].to_string())
}

#[cfg(test)]
mod tests {
    #[test]
    fn extract_url_from_osc8_line() {
        let line = "If the browser didn't open, visit: \x1b]8;;https://claude.com/cai/oauth/authorize?code=true&state=abc\x1b\\https://claude.com/cai/oauth/authorize?code=true&state=abc\x1b]8;;\x1b\\";
        let url = super::extract_url(line).unwrap();
        assert_eq!(
            url,
            "https://claude.com/cai/oauth/authorize?code=true&state=abc"
        );
    }

    #[test]
    fn parse_status_json() {
        let json = r#"{"loggedIn":true,"authMethod":"claude.ai","apiProvider":"firstParty","email":"a@b.c","orgId":"x","orgName":"y","subscriptionType":"pro"}"#;
        let s: super::LoginStatus = serde_json::from_str(json).unwrap();
        assert!(s.logged_in);
        assert_eq!(s.subscription_type.as_deref(), Some("pro"));
    }

    /// 실제 머신의 로그인 상태 조회 (읽기 전용). 실행: cargo test -- --ignored
    #[test]
    #[ignore = "claude 설치 필요"]
    fn login_status_on_this_machine() {
        let bin = crate::detect::claude_bin().expect("claude installed");
        let out = std::process::Command::new(bin)
            .args(["auth", "status", "--json"])
            .output()
            .unwrap();
        let s: super::LoginStatus =
            serde_json::from_str(String::from_utf8_lossy(&out.stdout).trim()).unwrap();
        println!("loggedIn={} type={:?}", s.logged_in, s.subscription_type);
    }
}

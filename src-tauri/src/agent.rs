use std::path::{Path, PathBuf};

/// 지원 에이전트. 설치·감지·로그인·첫 대화 방식의 차이를 이 레시피가 흡수한다
/// (docs/architecture.md §4 — 새 에이전트 추가 시 여기만 확장).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Agent {
    ClaudeCode,
    Codex,
}

impl Agent {
    pub fn from_id(id: &str) -> Result<Agent, String> {
        match id {
            "claude-code" => Ok(Agent::ClaudeCode),
            "codex" => Ok(Agent::Codex),
            _ => Err(format!("모르는 에이전트예요: {id}")),
        }
    }

    pub fn display_name(self) -> &'static str {
        match self {
            Agent::ClaudeCode => "클로드 코드",
            Agent::Codex => "코덱스",
        }
    }

    pub fn bin_name(self) -> &'static str {
        match self {
            Agent::ClaudeCode => "claude",
            Agent::Codex => "codex",
        }
    }

    /// 알려진 설치 위치 후보 디렉터리 (셸 PATH 탐색 이전의 우선 후보)
    pub fn candidate_dirs(self, home: &Path) -> Vec<PathBuf> {
        let mut dirs = vec![home.join(".local/bin")];
        if !cfg!(windows) {
            dirs.push(PathBuf::from("/opt/homebrew/bin"));
            dirs.push(PathBuf::from("/usr/local/bin"));
            match self {
                Agent::ClaudeCode => {
                    dirs.push(home.join(".claude/local"));
                    dirs.push(home.join(".npm-global/bin"));
                }
                Agent::Codex => {
                    dirs.push(home.join(".npm-global/bin"));
                }
            }
        }
        dirs
    }

    /// 구독 로그인 시작 인자. `use_api_billing`은 클로드의 콘솔(API 과금) 로그인 분기
    pub fn login_args(self, use_api_billing: bool) -> Vec<&'static str> {
        match self {
            Agent::ClaudeCode => {
                if use_api_billing {
                    vec!["auth", "login", "--console"]
                } else {
                    vec!["auth", "login", "--claudeai"]
                }
            }
            Agent::Codex => vec!["login"],
        }
    }

    pub fn status_args(self) -> Vec<&'static str> {
        match self {
            Agent::ClaudeCode => vec!["auth", "status", "--json"],
            Agent::Codex => vec!["login", "status"],
        }
    }

    /// 첫 대화(비대화형 1회 응답) 인자
    pub fn chat_args(self, prompt: &str) -> Vec<String> {
        match self {
            Agent::ClaudeCode => vec!["-p".into(), prompt.into()],
            // codex exec는 git 저장소 밖에서 확인을 요구하므로 건너뛴다
            Agent::Codex => vec![
                "exec".into(),
                "--skip-git-repo-check".into(),
                prompt.into(),
            ],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_id_roundtrip() {
        assert_eq!(Agent::from_id("claude-code").unwrap(), Agent::ClaudeCode);
        assert_eq!(Agent::from_id("codex").unwrap(), Agent::Codex);
        assert!(Agent::from_id("gemini").is_err());
    }
}

use serde::Serialize;
use std::path::PathBuf;
use std::process::{Command, Stdio};

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProjectInfo {
    pub path: String,
    /// false면 이미 있던 폴더를 그대로 재사용
    pub created: bool,
}

/// 초보자 안전 프리셋: 프로젝트의 에이전트에게 사용자가 초보자임을 알린다.
/// 권한 프리셋 정교화는 M2에서 (docs/architecture.md §6).
const FIRST_PROJECT_CLAUDE_MD: &str = "# 내 첫 프로젝트

이 폴더의 주인은 코딩을 처음 해 보는 사용자예요. 함께 일할 때:

- 쉬운 한국어로, 전문용어 없이 설명해 주세요.
- 파일을 지우거나 이 폴더 밖의 것을 바꾸기 전에는 반드시 먼저 물어봐 주세요.
- 작업을 마치면 무엇을 했는지 쉬운 말로 한 줄 정리해 주세요.
";

#[tauri::command]
pub async fn create_first_project(name: Option<String>) -> Result<ProjectInfo, String> {
    tauri::async_runtime::spawn_blocking(move || create(name))
        .await
        .map_err(|e| e.to_string())?
}

fn create(name: Option<String>) -> Result<ProjectInfo, String> {
    let name = sanitize_name(name.as_deref().unwrap_or("내-첫-프로젝트"));
    if name.is_empty() {
        return Err("폴더 이름에 쓸 수 있는 글자가 없어요. 다른 이름을 지어 주세요.".into());
    }
    let dir = documents_dir().join(&name);
    let created = !dir.exists();
    std::fs::create_dir_all(&dir)
        .map_err(|e| format!("폴더를 만들지 못했어요: {e}"))?;

    let claude_md = dir.join("CLAUDE.md");
    if !claude_md.exists() {
        std::fs::write(&claude_md, FIRST_PROJECT_CLAUDE_MD)
            .map_err(|e| format!("안내 파일을 만들지 못했어요: {e}"))?;
    }

    Ok(ProjectInfo {
        path: dir.display().to_string(),
        created,
    })
}

/// 프로젝트 폴더에서 비대화형으로 첫 인사를 주고받는다 (M0 검증: 신뢰 프롬프트 없이 동작).
#[tauri::command]
pub async fn run_first_chat(project_path: String) -> Result<String, String> {
    tauri::async_runtime::spawn_blocking(move || {
        let dir = PathBuf::from(&project_path);
        if !dir.is_dir() {
            return Err("프로젝트 폴더를 찾을 수 없어요. 이전 단계로 돌아가 주세요.".into());
        }
        let bin = crate::detect::claude_bin()
            .ok_or_else(|| "클로드 코드가 아직 설치되어 있지 않아요.".to_string())?;
        let out = Command::new(&bin)
            .args([
                "-p",
                "코딩 도우미를 처음 만나는 사용자에게 두 문장 이내의 짧고 따뜻한 한국어 환영 인사를 해 주세요.",
            ])
            .current_dir(&dir)
            .stdin(Stdio::null())
            .output()
            .map_err(|e| format!("클로드를 실행하지 못했어요: {e}"))?;
        let reply = String::from_utf8_lossy(&out.stdout).trim().to_string();
        if out.status.success() && !reply.is_empty() {
            Ok(reply)
        } else {
            let err = String::from_utf8_lossy(&out.stderr).trim().to_string();
            Err(if err.is_empty() {
                "클로드의 대답을 받지 못했어요. 다시 시도해 주세요.".into()
            } else {
                format!("클로드의 대답을 받지 못했어요: {err}")
            })
        }
    })
    .await
    .map_err(|e| e.to_string())?
}

fn documents_dir() -> PathBuf {
    crate::detect::home_dir().join("Documents")
}

/// 경로 구분자·제어문자를 제거하고 앞뒤 공백과 점을 정리한다
fn sanitize_name(raw: &str) -> String {
    let cleaned: String = raw
        .chars()
        .filter(|c| !matches!(c, '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|') && !c.is_control())
        .collect();
    cleaned.trim().trim_matches('.').trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanitize_strips_path_separators() {
        assert_eq!(sanitize_name("../../../etc"), "etc");
        assert_eq!(sanitize_name("내 프로젝트"), "내 프로젝트");
        assert_eq!(sanitize_name("a/b\\c:d"), "abcd");
        assert_eq!(sanitize_name("///"), "");
    }

    #[test]
    fn create_project_in_documents() {
        let name = format!("agent-starter-테스트-{}", std::process::id());
        let info = create(Some(name.clone())).unwrap();
        let dir = std::path::PathBuf::from(&info.path);
        assert!(info.created);
        assert!(dir.is_dir());
        assert!(dir.join("CLAUDE.md").is_file());
        // 두 번째 호출은 재사용으로 판정돼야 함
        let again = create(Some(name)).unwrap();
        assert!(!again.created);
        std::fs::remove_dir_all(&dir).unwrap();
    }

    /// 실제 첫 대화 실행 (사용량 소모). 실행: cargo test -- --ignored --nocapture
    #[test]
    #[ignore = "claude 로그인 + 사용량 소모"]
    fn first_chat_on_this_machine() {
        let home = std::env::temp_dir().join(format!("agent-starter-chat-{}", std::process::id()));
        std::fs::create_dir_all(&home).unwrap();
        let reply = tauri::async_runtime::block_on(super::run_first_chat(
            home.display().to_string(),
        ))
        .unwrap();
        println!("응답: {reply}");
        assert!(!reply.is_empty());
        std::fs::remove_dir_all(&home).ok();
    }
}

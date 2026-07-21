use serde::Serialize;
use std::path::{Path, PathBuf};

/// 초보자에게 추천하는 코드 편집기. 설치 여부를 감지하고, 설치돼 있으면
/// 프로젝트 폴더를 그 편집기로 바로 열어준다 ("편집기에서 폴더 열기"를 대신).
#[derive(Clone, Copy)]
enum Editor {
    Cursor,
    VsCode,
}

impl Editor {
    const ALL: [Editor; 2] = [Editor::Cursor, Editor::VsCode];

    fn id(self) -> &'static str {
        match self {
            Editor::Cursor => "cursor",
            Editor::VsCode => "vscode",
        }
    }

    fn name(self) -> &'static str {
        match self {
            Editor::Cursor => "커서(Cursor)",
            Editor::VsCode => "VS Code",
        }
    }

    fn url(self) -> &'static str {
        match self {
            Editor::Cursor => "https://cursor.com",
            Editor::VsCode => "https://code.visualstudio.com",
        }
    }

    fn from_id(id: &str) -> Option<Editor> {
        Editor::ALL.into_iter().find(|e| e.id() == id)
    }

    #[cfg(target_os = "macos")]
    fn mac_app_name(self) -> &'static str {
        match self {
            Editor::Cursor => "Cursor",
            Editor::VsCode => "Visual Studio Code",
        }
    }

    /// 설치 여부를 판단할 후보 경로들
    fn install_paths(self, home: &Path) -> Vec<PathBuf> {
        #[cfg(target_os = "macos")]
        {
            let app = format!("{}.app", self.mac_app_name());
            vec![
                PathBuf::from("/Applications").join(&app),
                home.join("Applications").join(&app),
            ]
        }
        #[cfg(windows)]
        {
            let local = std::env::var_os("LOCALAPPDATA")
                .map(PathBuf::from)
                .unwrap_or_default();
            let pf = std::env::var_os("ProgramFiles")
                .map(PathBuf::from)
                .unwrap_or_default();
            let _ = home;
            match self {
                Editor::Cursor => vec![
                    local.join(r"Programs\cursor\Cursor.exe"),
                    home.join(r"AppData\Local\Programs\cursor\Cursor.exe"),
                ],
                Editor::VsCode => vec![
                    local.join(r"Programs\Microsoft VS Code\Code.exe"),
                    pf.join(r"Microsoft VS Code\Code.exe"),
                ],
            }
        }
        #[cfg(not(any(target_os = "macos", windows)))]
        {
            let _ = home;
            Vec::new()
        }
    }

    fn installed_path(self, home: &Path) -> Option<PathBuf> {
        self.install_paths(home).into_iter().find(|p| p.exists())
    }
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EditorInfo {
    pub id: String,
    pub name: String,
    pub url: String,
    pub installed: bool,
}

#[tauri::command]
pub async fn detect_editors() -> Vec<EditorInfo> {
    tauri::async_runtime::spawn_blocking(|| {
        let home = crate::detect::home_dir();
        Editor::ALL
            .into_iter()
            .map(|e| EditorInfo {
                id: e.id().into(),
                name: e.name().into(),
                url: e.url().into(),
                installed: e.installed_path(&home).is_some(),
            })
            .collect()
    })
    .await
    .unwrap_or_default()
}

/// 설치된 편집기로 프로젝트 폴더를 연다.
#[tauri::command]
pub async fn open_in_editor(editor: String, path: String) -> Result<(), String> {
    let editor = Editor::from_id(&editor).ok_or("알 수 없는 편집기예요.")?;
    tauri::async_runtime::spawn_blocking(move || open(editor, &path))
        .await
        .map_err(|e| e.to_string())?
}

#[cfg(target_os = "macos")]
fn open(editor: Editor, path: &str) -> Result<(), String> {
    let status = std::process::Command::new("/usr/bin/open")
        .args(["-a", editor.mac_app_name()])
        .arg(path)
        .status()
        .map_err(|e| format!("편집기를 열지 못했어요: {e}"))?;
    if status.success() {
        Ok(())
    } else {
        Err("편집기를 여는 데 실패했어요. 편집기가 제대로 설치되어 있는지 확인해 주세요.".into())
    }
}

#[cfg(windows)]
fn open(editor: Editor, path: &str) -> Result<(), String> {
    let home = crate::detect::home_dir();
    let exe = editor
        .installed_path(&home)
        .ok_or("편집기를 찾지 못했어요. 먼저 설치해 주세요.")?;
    crate::detect::command(&exe)
        .arg(path)
        .spawn()
        .map_err(|e| format!("편집기를 열지 못했어요: {e}"))?;
    Ok(())
}

#[cfg(not(any(target_os = "macos", windows)))]
fn open(_editor: Editor, _path: &str) -> Result<(), String> {
    Err("이 운영체제에서는 편집기 열기를 지원하지 않아요.".into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn editor_ids_roundtrip() {
        for e in Editor::ALL {
            assert!(Editor::from_id(e.id()).is_some());
            assert!(!e.name().is_empty());
            assert!(e.url().starts_with("https://"));
        }
        assert!(Editor::from_id("emacs").is_none());
    }

    /// 실기기에 설치된 편집기를 감지하는지 확인. 실행: cargo test -- --ignored --nocapture
    #[test]
    #[ignore = "실기기 편집기 설치 상황에 의존"]
    fn detect_editors_on_this_machine() {
        let home = crate::detect::home_dir();
        for e in Editor::ALL {
            println!("{}: {:?}", e.name(), e.installed_path(&home));
        }
    }
}

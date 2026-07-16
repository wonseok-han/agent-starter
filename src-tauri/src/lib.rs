mod detect;
mod install;
mod login;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(login::LoginSession::default())
        .invoke_handler(tauri::generate_handler![
            detect::detect_environment,
            install::install_claude_code,
            login::login_status,
            login::start_login,
            login::submit_login_code,
            login::cancel_login
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

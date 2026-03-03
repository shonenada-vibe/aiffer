pub mod commands;
pub mod git;
pub mod models;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            commands::greet,
            commands::get_diff,
            commands::get_file_status,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

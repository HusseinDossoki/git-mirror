#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

// Module tree
pub mod git_operations;
pub mod azure_devops;
pub mod tauri_apis;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            tauri_apis::apis::mirror_repository,
            tauri_apis::apis::get_azure_devops_projects,
            tauri_apis::apis::sync_azure_devops_project
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

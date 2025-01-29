use std::{
    process::Command,
    sync::{Arc, Mutex},
};

use api::ollama::get_api_tags;
use tauri::{Manager, State};

#[derive(Clone)]
struct AppState {
    ollama_process: Arc<Mutex<std::process::Child>>,
}

mod api;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str, state: State<'_, AppState>) -> String {
    let process_id = state
        .ollama_process
        .lock()
        .expect("failed to lock mutex")
        .id();
    let api_tags = get_api_tags();
    format!(
        "Hello, {}! You've been greeted from Rust! Process ID: {} AND {:?}",
        name, process_id, api_tags
    )
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let ollama_process = Arc::new(Mutex::new(
        Command::new("ollama")
            .arg("serve")
            .spawn()
            .expect("failed to execute process"),
    ));

    let state = AppState {
        ollama_process: Arc::clone(&ollama_process),
    };

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .setup(|app| {
            app.manage(state);
            Ok(())
        })
        .on_window_event(move |window, window_event| {
            if let tauri::WindowEvent::CloseRequested { .. } = window_event {
                let app_handle = window.app_handle();
                let state: State<AppState> = app_handle.state();
                stop_ollama(state);
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn stop_ollama(state: State<'_, AppState>) {
    let mut process = state.ollama_process.lock().expect("failed to lock mutex");
    process.kill().expect("failed to kill process");
}

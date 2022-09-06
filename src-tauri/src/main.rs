#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::time::Duration;

#[tauri::command]
fn eval(window: tauri::Window, contents: String) {
    std::thread::spawn(move || {
        std::thread::sleep(Duration::from_millis(1000));
        window.eval(contents.as_str()).unwrap();
    });
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            eval
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}



fn main() {
     // Configure color scheme for log level
     let colors = fern::colors::ColoredLevelConfig::new()
     .info(fern::colors::Color::Green)
     .warn(fern::colors::Color::Yellow)
     .error(fern::colors::Color::Red)
     .debug(fern::colors::Color::White)
     .trace(fern::colors::Color::BrightBlack);
    
    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::default().format(move|out, message, record| {
            // Custom format: exclude the target information
            out.finish(format_args!(
                "[{}][{}] {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                colors.color(record.level()),
                message
            ))
        }).build()) // <-- this line here
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

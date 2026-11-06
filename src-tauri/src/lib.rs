// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

use tauri::{Manager, PhysicalPosition, Position};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();

            // Di Tauri v2, kita bisa mengatur posisi dan sifat jendela dengan aman di sini
            if let Ok(Some(monitor)) = window.current_monitor() {
                let screen_size = monitor.size();
                let window_size = window.outer_size().unwrap_or_default();

                let x = (screen_size.width as i32 - window_size.width as i32) / 2;

                // Set y ke 0 agar menempel di batas paling atas layar
                let y = 0;

                window
                    .set_position(Position::Physical(PhysicalPosition { x, y }))
                    .ok();
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

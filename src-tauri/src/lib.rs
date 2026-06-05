use std::ffi::CString;
use std::os::raw::c_char;

extern "C" {
    fn core_launch_path(path: *const c_char) -> i32;
    fn core_reveal_path(path: *const c_char) -> i32;
}

#[tauri::command]
fn launch_link(path: &str) -> Result<(), String> {
    println!("Rust: Launching path: {}", path);

    let c_path = CString::new(path).map_err(|e| e.to_string())?;

    let result = unsafe { core_launch_path(c_path.as_ptr()) };

    if result == 0 {
        Ok(())
    } else {
        Err(format!("C++ Core Error (Launch): Code {}", result))
    }
}

#[tauri::command]
fn reveal_link(path: &str) -> Result<(), String> {
    println!("Rust: Revealing path: {}", path);

    let c_path = CString::new(path).map_err(|e| e.to_string())?;

    let result = unsafe { core_reveal_path(c_path.as_ptr()) };

    if result == 0 {
        Ok(())
    } else {
        Err(format!("C++ Core Error (Reveal): Code {}", result))
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_notification::init())
        .invoke_handler(tauri::generate_handler![launch_link, reveal_link])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

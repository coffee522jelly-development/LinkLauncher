use tauri_plugin_opener::OpenerExt;

#[tauri::command]
fn launch_link(app: tauri::AppHandle, path: &str) -> Result<(), String> {
    println!("Rust (Plugin): Launching path: {}", path);

    if path.starts_with("http") {
        app.opener().open_url(path, None::<&str>).map_err(|e| e.to_string())?;
    } else {
        app.opener().open_path(path, None::<&str>).map_err(|e| e.to_string())?;
    }

    Ok(())
}

#[tauri::command]
fn reveal_link(app: tauri::AppHandle, path: &str) -> Result<(), String> {
    println!("Rust (Plugin): Revealing path: {}", path);

    app.opener().reveal_item_in_dir(path).map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
fn open_terminal(path: &str) -> Result<(), String> {
    println!("Rust: Opening terminal at: {}", path);

    let target_path = std::path::Path::new(path);

    // Determine the directory to open
    let target_dir = if target_path.exists() {
        if target_path.is_file() {
            target_path.parent().unwrap_or(target_path)
        } else {
            target_path
        }
    } else {
        // If the path doesn't exist locally (e.g. invalid path or network drive not mapped),
        // we try to extract the directory part if it looks like a path
        if path.contains('\\') || path.contains('/') {
            target_path.parent().unwrap_or(target_path)
        } else {
            target_path
        }
    };

    println!("Rust: target_dir set to: {}", target_dir.display());

    // On Windows, use start cmd to open a new terminal window at the specific directory
    #[cfg(target_os = "windows")]
    {
        // We use /d to ensure drive change if necessary, and use the directory as current_dir
        std::process::Command::new("cmd")
            .args(["/c", "start", "cmd", "/k", &format!("cd /d \"{}\"", target_dir.display())])
            .current_dir(target_dir)
            .spawn()
            .map_err(|e| format!("Failed to spawn cmd: {}", e))?;
    }

    // Basic support for other OS just in case (e.g. Linux with gnome-terminal)
    #[cfg(not(target_os = "windows"))]
    {
         std::process::Command::new("sh")
            .arg("-c")
            .arg(format!("cd \"{}\" && x-terminal-emulator", target_dir.display()))
            .current_dir(target_dir)
            .spawn()
            .map_err(|e| format!("Failed to spawn terminal: {}", e))?;
    }

    Ok(())
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
        .invoke_handler(tauri::generate_handler![launch_link, reveal_link, open_terminal])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

use tauri_plugin_opener::OpenerExt;

/// リンクを適切なアプリケーションで開くコマンド
/// URLの場合はブラウザ、ローカルパスの場合は規定のアプリで開きます。
#[tauri::command]
fn launch_link(app: tauri::AppHandle, path: &str) -> Result<(), String> {
    println!("Rust (Plugin): Launching path: {}", path);

    if path.starts_with("http") {
        // URLを開く
        app.opener().open_url(path, None::<&str>).map_err(|e| e.to_string())?;
    } else {
        // ローカルパスを開く
        app.opener().open_path(path, None::<&str>).map_err(|e| e.to_string())?;
    }

    Ok(())
}

/// ファイルの保存場所（フォルダ）をエクスプローラーで開くコマンド
/// 指定されたパスがファイルの場合は、そのファイルを選択した状態で開きます。
#[tauri::command]
fn reveal_link(app: tauri::AppHandle, path: &str) -> Result<(), String> {
    println!("Rust (Plugin): Revealing path: {}", path);

    // システムの標準機能を使用してフォルダを表示し、アイテムを選択状態にする
    app.opener().reveal_item_in_dir(path).map_err(|e| e.to_string())?;

    Ok(())
}

/// 指定されたパスでターミナル（コマンドプロンプト等）を開くコマンド
/// 指定されたパスがファイルの場合は、その親ディレクトリで開きます。
#[tauri::command]
fn open_terminal(path: &str) -> Result<(), String> {
    println!("Rust: Opening terminal at: {}", path);

    let target_path = std::path::Path::new(path);

    // 開く対象のディレクトリを決定する
    let target_dir = if target_path.exists() {
        if target_path.is_file() {
            // ファイルの場合は親フォルダを取得
            target_path.parent().unwrap_or(target_path)
        } else {
            // フォルダの場合はそのまま使用
            target_path
        }
    } else {
        // パスが現在存在しない場合（ネットワークドライブ等）、区切り文字からディレクトリ部分を推定
        if path.contains('\\') || path.contains('/') {
            target_path.parent().unwrap_or(target_path)
        } else {
            target_path
        }
    };

    println!("Rust: target_dir set to: {}", target_dir.display());

    // Windows環境の場合：cmd.exeを起動し、指定ディレクトリへ移動する
    #[cfg(target_os = "windows")]
    {
        // /d オプションでドライブ変更にも対応し、カレントディレクトリも設定する
        std::process::Command::new("cmd")
            .args(["/c", "start", "cmd", "/k", &format!("cd /d \"{}\"", target_dir.display())])
            .current_dir(target_dir)
            .spawn()
            .map_err(|e| format!("Failed to spawn cmd: {}", e))?;
    }

    // Windows以外の環境（Linuxなど）：x-terminal-emulator等での起動を試みる
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
        // 各種プラグインの初期化
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_window_state::Builder::default().build())
        // フロントエンドから呼び出すコマンドの登録
        .invoke_handler(tauri::generate_handler![launch_link, reveal_link, open_terminal])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

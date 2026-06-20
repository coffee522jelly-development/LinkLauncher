use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Manager, Runtime,
};
use tauri_plugin_opener::OpenerExt;

// ============================================================================
// Tauriコマンド定義: フロントエンド(Svelte)から呼び出されるRust関数
// ============================================================================

/// リンクを適切なアプリケーションで開く
///
/// * `path` - 開きたいURLまたはファイルパス
///
/// URLの場合はシステムの既定ブラウザ、ローカルパスの場合はOSに関連付けられたアプリで開きます。
#[tauri::command]
fn launch_link(app: tauri::AppHandle, path: &str) -> Result<(), String> {
    println!("Rust (Plugin): Launching path: {}", path);

    if path.starts_with("http") {
        // HTTP/HTTPSプロトコルの場合はブラウザでURLを開く
        app.opener().open_url(path, None::<&str>).map_err(|e| e.to_string())?;
    } else {
        // それ以外（ファイルパス）の場合はシステム標準のハンドラで開く
        app.opener().open_path(path, None::<&str>).map_err(|e| e.to_string())?;
    }

    Ok(())
}

/// ファイルの保存場所をエクスプローラーで表示する
///
/// * `path` - 対象となるファイルのフルパス
///
/// 指定されたパスがファイルの場合、そのファイルが保存されているフォルダを開き、
/// 該当ファイルをハイライト（選択）した状態でエクスプローラーを表示します。
#[tauri::command]
fn reveal_link(app: tauri::AppHandle, path: &str) -> Result<(), String> {
    println!("Rust (Plugin): Revealing path: {}", path);

    // Tauri v2 の opener プラグインを使用して、アイテムを選択した状態でディレクトリを表示
    app.opener().reveal_item_in_dir(path).map_err(|e| e.to_string())?;

    Ok(())
}

/// 指定されたパスでターミナル（コマンドプロンプト）を開く
///
/// * `path` - 対象のディレクトリ、またはファイルが含まれるフォルダ
///
/// 指定されたディレクトリに移動(`cd`)した状態で新しいターミナルウィンドウを起動します。
#[tauri::command]
fn open_terminal(path: &str) -> Result<(), String> {
    println!("Rust: Opening terminal at: {}", path);

    let target_path = std::path::Path::new(path);

    // 開く対象となるディレクトリの判定ロジック
    let target_dir = if target_path.exists() {
        if target_path.is_file() {
            // ファイルが指定された場合は、その親（フォルダ）を取得
            target_path.parent().unwrap_or(target_path)
        } else {
            // 既にディレクトリの場合はそのまま使用
            target_path
        }
    } else {
        // パスが物理的に存在しない場合（ネットワーク未接続など）、パス文字列からディレクトリ部を推論
        if path.contains('\\') || path.contains('/') {
            target_path.parent().unwrap_or(target_path)
        } else {
            target_path
        }
    };

    println!("Rust: target_dir set to: {}", target_dir.display());

    // Windows環境向けのターミナル起動処理
    #[cfg(target_os = "windows")]
    {
        // cmd.exe を /k 引数で起動し、/d オプション付き cd でドライブを跨ぐ移動を確実に行う
        std::process::Command::new("cmd")
            .args(["/c", "start", "cmd", "/k", &format!("cd /d \"{}\"", target_dir.display())])
            .current_dir(target_dir) // プロセスのカレントディレクトリも念のため設定
            .spawn()
            .map_err(|e| format!("Failed to spawn cmd: {}", e))?;
    }

    // Windows以外の環境（Linux等）でのフォールバック処理
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

// ============================================================================
// アプリケーションのエントリポイント
// ============================================================================

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // --- システムトレイの設定 ---
        .setup(|app| {
            // トレイメニューの作成
            let quit_i = MenuItem::with_id(app, "quit", "終了", true, None::<&str>)?;
            let show_i = MenuItem::with_id(app, "show", "メイン画面を表示", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show_i, &quit_i])?;

            // トレイアイコンの構築
            let _tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "quit" => {
                        app.exit(0);
                    }
                    "show" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                })
                .build(app)?;

            Ok(())
        })
        // --- Tauri v2 プラグインのセットアップ ---
        .plugin(tauri_plugin_opener::init())            // ファイルやURLを開くためのプラグイン
        .plugin(tauri_plugin_shell::init())             // シェルコマンド実行用（ターミナル起動で使用）
        .plugin(tauri_plugin_fs::init())                // ファイルシステム操作用（CSV入出力で使用）
        .plugin(tauri_plugin_clipboard_manager::init()) // クリップボード操作用
        .plugin(tauri_plugin_store::Builder::default().build()) // 設定やデータの永続化(JSON)
        .plugin(tauri_plugin_dialog::init())            // ファイル選択ダイアログ用
        .plugin(tauri_plugin_notification::init())      // OS通知用
        .plugin(tauri_plugin_window_state::Builder::default().build()) // ウィンドウサイズ・位置の保存/復元

        // --- フロントエンド向けコマンドの登録 ---
        .invoke_handler(tauri::generate_handler![
            launch_link,
            reveal_link,
            open_terminal
        ])

        // --- アプリケーションの実行 ---
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|_app_handle, event| match event {
            // ウィンドウの「閉じる」ボタンが押された時の挙動をカスタマイズ
            tauri::RunEvent::WindowEvent { label, event: tauri::WindowEvent::CloseRequested { api, .. }, .. } => {
                if label == "main" {
                    // アプリを終了せず、ウィンドウを隠すだけにする（常駐状態の維持）
                    api.prevent_close();
                    if let Some(window) = _app_handle.get_webview_window("main") {
                        let _ = window.hide();
                    }
                }
            }
            _ => {}
        });
}

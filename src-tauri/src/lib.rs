use serde::Deserialize;
use tauri::{
    menu::{Menu, MenuItem, PredefinedMenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Manager,
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
#[derive(Debug, Deserialize)]
struct LinkItem {
    name: String,
    path: String,
}

#[derive(Debug, Deserialize)]
struct LinkData {
    links: Vec<LinkItem>,
}

/// システムトレイのメニューを更新する
#[tauri::command]
fn refresh_tray_menu(app: tauri::AppHandle) -> Result<(), String> {
    let app_clone = app.clone();

    // 別スレッドでファイル読み込みとメニュー更新を行う
    std::thread::spawn(move || {
        let store_path = app_clone
            .path()
            .app_data_dir()
            .unwrap_or_default()
            .join("links.json");

        let mut tray_links = Vec::new();

        if let Ok(content) = std::fs::read_to_string(store_path) {
            if let Ok(data) = serde_json::from_str::<LinkData>(&content) {
                // 最新または重要な5件を抽出（ここでは末尾の5件）
                tray_links = data.links.into_iter().rev().take(5).collect();
            }
        }

        let handle = app_clone.clone();

        // メインスレッドでメニューを再構築
        let _ = app_clone.run_on_main_thread(move || {
            if let Some(tray) = handle.tray_by_id("main_tray") {
                let quit_i = MenuItem::with_id(&handle, "quit", "終了", true, None::<&str>).unwrap();
                let show_i = MenuItem::with_id(&handle, "show", "メイン画面を表示", true, None::<&str>).unwrap();

                let mut menu_items: Vec<Box<dyn tauri::menu::IsMenuItem<tauri::Wry>>> = Vec::new();
                menu_items.push(Box::new(show_i));
                menu_items.push(Box::new(PredefinedMenuItem::separator(&handle).unwrap()));

                // 動的リンクの追加
                for link in tray_links {
                    let item = MenuItem::with_id(
                        &handle,
                        format!("link:{}", link.path), // IDにパスを含める
                        format!("🚀 {}", link.name),
                        true,
                        None::<&str>
                    ).unwrap();
                    menu_items.push(Box::new(item));
                }

                menu_items.push(Box::new(PredefinedMenuItem::separator(&handle).unwrap()));
                menu_items.push(Box::new(quit_i));

                let menu = Menu::with_items(&handle, &menu_items.iter().map(|b| b.as_ref()).collect::<Vec<_>>()).unwrap();
                let _ = tray.set_menu(Some(menu));
            }
        });
    });

    Ok(())
}

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
            // トレイメニューの作成（初期状態）
            let quit_i = MenuItem::with_id(app, "quit", "終了", true, None::<&str>)?;
            let show_i = MenuItem::with_id(app, "show", "メイン画面を表示", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show_i, &quit_i])?;

            // トレイアイコンの構築
            let _tray = TrayIconBuilder::with_id("main_tray")
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .on_menu_event(|app, event| {
                    let id = event.id.as_ref();
                    if id == "quit" {
                        app.exit(0);
                    } else if id == "show" {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    } else if id.starts_with("link:") {
                        let path = &id[5..]; // "link:" プレフィックスを削除
                        let _ = launch_link(app.clone(), path);
                    }
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
            open_terminal,
            refresh_tray_menu
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

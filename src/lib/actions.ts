import { writeText } from '@tauri-apps/plugin-clipboard-manager';
import { openUrl, openPath as tauriOpenPath, revealItemInDir } from '@tauri-apps/plugin-opener';
import { isPermissionGranted, requestPermission, sendNotification } from '@tauri-apps/plugin-notification';
import { invoke } from '@tauri-apps/api/core';

async function notify(title: string, body: string) {
  try {
    let permission = await isPermissionGranted();
    if (!permission) {
      const permissionStatus = await requestPermission();
      permission = permissionStatus === 'granted';
    }
    if (permission) {
      sendNotification({ title, body });
    }
  } catch (err) {
    console.error('Notification error:', err);
  }
}

export async function copyToClipboard(text: string) {
  try {
    await writeText(text);
    await notify('コピー成功', 'パスをクリップボードにコピーしました。');
  } catch (err) {
    console.error('Failed to copy to clipboard:', err);
    await notify('エラー', 'コピーに失敗しました。権限を確認してください。');
  }
}

export async function openPath(path: string) {
  try {
    // Calling our new C++ Core via Rust Bridge
    await invoke('launch_link', { path });
  } catch (err) {
    console.error('Failed to open path via Core Engine:', err);
    // Fallback to Tauri plugin if C++ Core fails for any reason
    try {
      if (path.startsWith('http')) {
        await openUrl(path);
      } else {
        await tauriOpenPath(path);
      }
    } catch (fallbackErr) {
      console.error('Fallback also failed:', fallbackErr);
      await notify('エラー', `パスを開けませんでした: ${fallbackErr}`);
    }
  }
}

export async function revealInExplorer(path: string) {
  try {
    if (path.startsWith('http')) {
      await openUrl(path);
      return;
    }

    // Using our new C++ Core for better file selection support
    await invoke('reveal_link', { path });
  } catch (err) {
    console.error('Failed to reveal in explorer via Core Engine:', err);
    // Fallback: try Tauri v2 native reveal
    try {
      await revealItemInDir(path);
    } catch (fallbackErr) {
      console.error('Fallback reveal failed:', fallbackErr);
      await notify('エラー', `フォルダを開けませんでした: ${fallbackErr}`);
    }
  }
}

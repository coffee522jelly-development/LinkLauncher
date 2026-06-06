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
    // Using Rust command which utilizes the official tauri-plugin-opener
    await invoke('launch_link', { path });
  } catch (err) {
    console.error('Failed to open path:', err);
    await notify('エラー', `パスを開けませんでした: ${err}`);
  }
}

export async function revealInExplorer(path: string) {
  try {
    if (path.startsWith('http')) {
      await openUrl(path);
      return;
    }

    // Using Rust command which utilizes the official tauri-plugin-opener
    await invoke('reveal_link', { path });
  } catch (err) {
    console.error('Failed to reveal in explorer:', err);
    await notify('エラー', `フォルダを開けませんでした: ${err}`);
  }
}

export async function openTerminal(path: string) {
  try {
    if (path.startsWith('http')) return;

    await invoke('open_terminal', { path });
  } catch (err) {
    console.error('Failed to open terminal:', err);
    await notify('エラー', `コマンドプロンプトを開けませんでした: ${err}`);
  }
}

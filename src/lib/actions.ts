import { writeText } from '@tauri-apps/plugin-clipboard-manager';
import { openUrl, openPath as tauriOpenPath, revealItemInDir } from '@tauri-apps/plugin-opener';
import { isPermissionGranted, requestPermission, sendNotification } from '@tauri-apps/plugin-notification';

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
    if (path.startsWith('http')) {
      await openUrl(path);
    } else {
      await tauriOpenPath(path);
    }
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

    // revealItemInDir is the recommended way in Tauri v2 to open the folder and highlight the item.
    await revealItemInDir(path);
  } catch (err) {
    console.error('Failed to reveal in explorer:', err);
    // Fallback: try opening parent dir with openPath if revealItemInDir fails
    try {
      const isFile = /\.[a-z0-9]+$/i.test(path);
      if (isFile) {
        const lastSlash = Math.max(path.lastIndexOf('/'), path.lastIndexOf('\\'));
        if (lastSlash !== -1) {
          const dir = path.substring(0, lastSlash);
          await tauriOpenPath(dir);
          return;
        }
      }
      await tauriOpenPath(path);
    } catch (innerErr) {
      await notify('エラー', `フォルダを開けませんでした: ${innerErr}`);
    }
  }
}

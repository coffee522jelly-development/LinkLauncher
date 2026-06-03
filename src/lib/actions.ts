import { writeText } from '@tauri-apps/plugin-clipboard-manager';
import { openUrl, openPath as tauriOpenPath } from '@tauri-apps/plugin-opener';

export async function copyToClipboard(text: string) {
  try {
    await writeText(text);
  } catch (err) {
    console.error('Failed to copy to clipboard:', err);
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
  }
}

export async function revealInExplorer(path: string) {
  try {
    if (path.startsWith('http')) {
      await openUrl(path);
      return;
    }

    // "Open in Explorer" should ideally open the containing folder for files.
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
  } catch (err) {
    console.error('Failed to reveal in explorer:', err);
  }
}

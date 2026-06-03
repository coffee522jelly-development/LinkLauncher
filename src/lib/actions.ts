import { writeText } from '@tauri-apps/plugin-clipboard-manager';
import { open } from '@tauri-apps/plugin-shell';

export async function copyToClipboard(text: string) {
  try {
    await writeText(text);
  } catch (err) {
    console.error('Failed to copy to clipboard:', err);
  }
}

export async function openPath(path: string) {
  try {
    await open(path);
  } catch (err) {
    console.error('Failed to open path:', err);
  }
}

export async function revealInExplorer(path: string) {
  try {
    // In many Tauri configurations, shell.open for a directory path opens it in the file explorer.
    // If it's a file path, we can't easily "select" it without OS-specific commands.
    // For now, we'll open it. If it's a folder, it opens explorer.
    // If it's a file, it might open the file or we can try to open its parent.
    await open(path);
  } catch (err) {
    console.error('Failed to reveal in explorer:', err);
  }
}

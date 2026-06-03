import { writable } from 'svelte/store';
import { load } from '@tauri-apps/plugin-store';

export type Theme = 'zinc' | 'blue' | 'rose' | 'green';

interface Settings {
  theme: Theme;
}

const STORE_PATH = 'settings.json';

function createSettingsStore() {
  const { subscribe, set, update } = writable<Settings>({ theme: 'zinc' });

  return {
    subscribe,
    load: async () => {
      const store = await load(STORE_PATH);
      const saved = await store.get<Settings>('settings');
      if (saved) {
        set(saved);
        applyTheme(saved.theme);
      }
    },
    setTheme: async (theme: Theme) => {
      const store = await load(STORE_PATH);
      update((s) => {
        const updated = { ...s, theme };
        store.set('settings', updated).then(() => store.save());
        applyTheme(theme);
        return updated;
      });
    },
  };
}

function applyTheme(theme: Theme) {
  if (typeof document !== 'undefined') {
    document.documentElement.dataset.theme = theme;
  }
}

export const settingsStore = createSettingsStore();

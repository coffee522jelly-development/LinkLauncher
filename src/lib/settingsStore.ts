import { writable } from 'svelte/store';
import { load } from '@tauri-apps/plugin-store';

export type Theme = 'zinc' | 'blue' | 'rose' | 'green' | 'orange' | 'slate';
export type ViewMode = 'table' | 'grid';

interface Settings {
  theme: Theme;
  viewMode: ViewMode;
}

const STORE_PATH = 'settings.json';

function createSettingsStore() {
  const { subscribe, set, update } = writable<Settings>({
    theme: 'zinc',
    viewMode: 'table'
  });

  return {
    subscribe,
    load: async () => {
      try {
        const store = await load(STORE_PATH);
        const saved = await store.get<Settings>('settings');
        if (saved) {
          set(saved);
          applyTheme(saved.theme);
        }
      } catch (err) {
        console.error('Failed to load settings:', err);
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
    setViewMode: async (viewMode: ViewMode) => {
      try {
        const store = await load(STORE_PATH);
        update((s) => {
          const updated = { ...s, viewMode };
          store.set('settings', updated).then(() => store.save());
          return updated;
        });
      } catch (err) {
        console.error('Failed to save viewMode:', err);
        // Fallback for browser verification
        update((s) => ({ ...s, viewMode }));
      }
    }
  };
}

function applyTheme(theme: Theme) {
  if (typeof document !== 'undefined') {
    document.documentElement.dataset.theme = theme;
  }
}

export const settingsStore = createSettingsStore();

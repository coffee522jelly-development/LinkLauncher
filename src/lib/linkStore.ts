import { writable } from 'svelte/store';
import { load } from '@tauri-apps/plugin-store';

export interface Link {
  id: string;
  name: string;
  path: string;
  category: string;
}

const STORE_PATH = 'links.json';

function createLinkStore() {
  const { subscribe, set, update } = writable<Link[]>([]);

  return {
    subscribe,
    load: async () => {
      try {
        const store = await load(STORE_PATH);
        const savedLinks = await store.get<Link[]>('links');
        if (savedLinks) {
          set(savedLinks);
        }
      } catch (err) {
        console.error('Failed to load links:', err);
      }
    },
    add: async (name: string, path: string, category: string = '') => {
      const newLink: Link = {
        id: typeof crypto.randomUUID === 'function' ? crypto.randomUUID() : Math.random().toString(36).substring(2),
        name,
        path,
        category,
      };
      try {
        const store = await load(STORE_PATH);
        update((links) => {
          const updated = [...links, newLink];
          store.set('links', updated).then(() => store.save());
          return updated;
        });
      } catch (err) {
        console.error('Failed to add link:', err);
      }
    },
    remove: async (id: string) => {
      try {
        const store = await load(STORE_PATH);
        update((links) => {
          const updated = links.filter((link) => link.id !== id);
          store.set('links', updated).then(() => store.save());
          return updated;
        });
      } catch (err) {
        console.error('Failed to remove link:', err);
      }
    },
  };
}

export const linkStore = createLinkStore();

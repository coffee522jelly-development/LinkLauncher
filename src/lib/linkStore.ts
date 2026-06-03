import { writable } from 'svelte/store';
import { load } from '@tauri-apps/plugin-store';

export interface Link {
  id: string;
  name: string;
  path: string;
}

const STORE_PATH = 'links.json';

function createLinkStore() {
  const { subscribe, set, update } = writable<Link[]>([]);

  return {
    subscribe,
    load: async () => {
      const store = await load(STORE_PATH);
      const savedLinks = await store.get<Link[]>('links');
      if (savedLinks) {
        set(savedLinks);
      }
    },
    add: async (name: string, path: string) => {
      const newLink: Link = {
        id: crypto.randomUUID(),
        name,
        path,
      };
      const store = await load(STORE_PATH);
      update((links) => {
        const updated = [...links, newLink];
        store.set('links', updated).then(() => store.save());
        return updated;
      });
    },
    remove: async (id: string) => {
      const store = await load(STORE_PATH);
      update((links) => {
        const updated = links.filter((link) => link.id !== id);
        store.set('links', updated).then(() => store.save());
        return updated;
      });
    },
  };
}

export const linkStore = createLinkStore();

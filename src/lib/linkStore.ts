import { writable } from 'svelte/store';
import { LazyStore } from '@tauri-apps/plugin-store';

export interface Link {
  id: string;
  name: string;
  path: string;
}

const STORE_PATH = 'links.json';
const store = new LazyStore(STORE_PATH);

function createLinkStore() {
  const { subscribe, set, update } = writable<Link[]>([]);

  return {
    subscribe,
    load: async () => {
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
      update((links) => {
        const updated = [...links, newLink];
        store.set('links', updated).then(() => store.save());
        return updated;
      });
    },
    remove: async (id: string) => {
      update((links) => {
        const updated = links.filter((link) => link.id !== id);
        store.set('links', updated).then(() => store.save());
        return updated;
      });
    },
  };
}

export const linkStore = createLinkStore();

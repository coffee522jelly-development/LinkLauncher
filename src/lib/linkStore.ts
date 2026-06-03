import { writable, get } from 'svelte/store';
import { load } from '@tauri-apps/plugin-store';
import { save, open } from '@tauri-apps/plugin-dialog';
import { writeTextFile, readTextFile } from '@tauri-apps/plugin-fs';

export interface Link {
  id: string;
  name: string;
  path: string;
  category: string;
}

const STORE_PATH = 'links.json';

function createLinkStore() {
  const { subscribe, set, update } = writable<Link[]>([]);

  const persist = async (links: Link[]) => {
    try {
      const store = await load(STORE_PATH);
      await store.set('links', links);
      await store.save();
    } catch (err) {
      console.error('Failed to persist links:', err);
    }
  };

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
        id: crypto.randomUUID(),
        name,
        path,
        category,
      };
      update((links) => {
        const updated = [...links, newLink];
        persist(updated);
        return updated;
      });
    },
    remove: async (id: string) => {
      update((links) => {
        const updated = links.filter((link) => link.id !== id);
        persist(updated);
        return updated;
      });
    },
    exportToCSV: async () => {
      try {
        const links = get({ subscribe });
        if (links.length === 0) return;

        const filePath = await save({
          filters: [{ name: 'CSV', extensions: ['csv'] }],
          defaultPath: 'links_backup.csv'
        });

        if (filePath) {
          const header = '名称,カテゴリー,パス\n';
          const content = links.map(l =>
            `"${l.name.replace(/"/g, '""')}","${l.category.replace(/"/g, '""')}","${l.path.replace(/"/g, '""')}"`
          ).join('\n');
          await writeTextFile(filePath, header + content);
        }
      } catch (err) {
        console.error('Failed to export CSV:', err);
      }
    },
    importFromCSV: async () => {
      try {
        const selected = await open({
          filters: [{ name: 'CSV', extensions: ['csv'] }],
          multiple: false
        });

        if (selected && typeof selected === 'string') {
          const content = await readTextFile(selected);
          const lines = content.split('\n').filter(line => line.trim() !== '');
          const dataLines = lines.slice(1);

          const importedLinks: Link[] = [];
          for (const line of dataLines) {
            const parts = line.match(/(".*?"|[^",\s]+)(?=\s*,|\s*$)/g);
            if (parts && parts.length >= 2) {
              const name = parts[0].replace(/^"|"$/g, '').replace(/""/g, '"');
              const category = parts[1].replace(/^"|"$/g, '').replace(/""/g, '"');
              const path = (parts[2] || '').replace(/^"|"$/g, '').replace(/""/g, '"');
              importedLinks.push({
                id: crypto.randomUUID(),
                name,
                category,
                path
              });
            }
          }

          if (importedLinks.length > 0) {
            update(current => {
              const updated = [...current, ...importedLinks];
              persist(updated);
              return updated;
            });
          }
        }
      } catch (err) {
        console.error('Failed to import CSV:', err);
      }
    }
  };
}

export const linkStore = createLinkStore();

import { writable, get } from 'svelte/store';
import { load } from '@tauri-apps/plugin-store';
import { save, open } from '@tauri-apps/plugin-dialog';
import { writeTextFile, readTextFile } from '@tauri-apps/plugin-fs';
import { refreshTray } from './actions';

export interface Link {
  id: string;
  name: string;
  path: string;
  category: string;
  isPinned?: boolean;
}

const STORE_PATH = 'links.json';

function parseCSVLine(line: string): string[] {
  const parts = [];
  let current = '';
  let inQuotes = false;

  for (let i = 0; i < line.length; i++) {
    const char = line[i];
    if (char === '"') {
      if (inQuotes && line[i+1] === '"') {
        current += '"';
        i++;
      } else {
        inQuotes = !inQuotes;
      }
    } else if (char === ',' && !inQuotes) {
      parts.push(current);
      current = '';
    } else {
      current += char;
    }
  }
  parts.push(current);
  return parts;
}

function createLinkStore() {
  const { subscribe, set, update } = writable<Link[]>([]);

  const persist = async (links: Link[]) => {
    try {
      const store = await load(STORE_PATH);
      await store.set('links', links);
      await store.save();
      // Update tray menu whenever links change
      refreshTray();
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
      let finalCategory = category;
      if (!finalCategory) {
        finalCategory = path.startsWith('http') ? 'Web' : 'Local';
      }

      const newLink: Link = {
        id: crypto.randomUUID(),
        name,
        path,
        category: finalCategory,
        isPinned: false,
      };
      update((links) => {
        const updated = [...links, newLink];
        persist(updated);
        return updated;
      });
    },

    togglePin: async (id: string) => {
      update((links) => {
        const updated = links.map(l => l.id === id ? { ...l, isPinned: !l.isPinned } : l);
        persist(updated);
        return updated;
      });
    },

  reorder: async (fromId: string, toId: string) => {
    const store = await load(STORE_PATH);
    update((links) => {
      const updated = [...links];
      const fromIdx = updated.findIndex(l => l.id === fromId);
      const toIdx = updated.findIndex(l => l.id === toId);

      if (fromIdx !== -1 && toIdx !== -1) {
        const [removed] = updated.splice(fromIdx, 1);
        updated.splice(toIdx, 0, removed);
        store.set('links', updated).then(() => store.save());
      }
      return updated;
    });
  },
    update: async (id: string, partial: Partial<Omit<Link, 'id'>>) => {
      update((links) => {
        const updated = links.map(l => l.id === id ? { ...l, ...partial } : l);
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
          const header = '名称,カテゴリー,パス,ピン留め\n';
          const content = links.map(l =>
            `"${l.name.replace(/"/g, '""')}","${l.category.replace(/"/g, '""')}","${l.path.replace(/"/g, '""')}","${l.isPinned ? '1' : '0'}"`
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
          const lines = content.split(/\r?\n/).filter(line => line.trim() !== '');
          const dataLines = lines.slice(1);

          const importedLinks: Link[] = [];
          for (const line of dataLines) {
            const parts = parseCSVLine(line);
            if (parts && parts.length >= 2) {
              importedLinks.push({
                id: crypto.randomUUID(),
                name: parts[0],
                category: parts[1],
                path: parts[2] || '',
                isPinned: parts[3] === '1'
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

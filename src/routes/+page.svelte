<script lang="ts">
  import { onMount } from 'svelte';
  import { linkStore } from '$lib/linkStore';
  import { settingsStore, type Theme } from '$lib/settingsStore';
  import { copyToClipboard, openPath, revealInExplorer } from '$lib/actions';
  import Button from '$lib/components/Button.svelte';
  import Input from '$lib/components/Input.svelte';
  import { Search, Plus, Trash2, Copy, FolderOpen, ExternalLink, Play, Settings, ChevronUp, ChevronDown } from 'lucide-svelte';

  let newName = $state('');
  let newPath = $state('');
  let newCategory = $state('');
  let searchQuery = $state('');
  let sortKey = $state<'name' | 'category'>('name');
  let sortOrder = $state<'asc' | 'desc'>('asc');
  let showSettings = $state(false);

  onMount(() => {
    linkStore.load();
    settingsStore.load();
  });

  const filteredLinks = $derived(
    $linkStore.filter((link) =>
      link.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
      link.path.toLowerCase().includes(searchQuery.toLowerCase()) ||
      link.category.toLowerCase().includes(searchQuery.toLowerCase())
    ).sort((a, b) => {
      const valA = a[sortKey].toLowerCase();
      const valB = b[sortKey].toLowerCase();
      if (valA < valB) return sortOrder === 'asc' ? -1 : 1;
      if (valA > valB) return sortOrder === 'asc' ? 1 : -1;
      return 0;
    })
  );

  async function addLink() {
    if (newName && newPath) {
      await linkStore.add(newName, newPath, newCategory);
      newName = '';
      newPath = '';
      newCategory = '';
    }
  }

  function toggleSort(key: 'name' | 'category') {
    if (sortKey === key) {
      sortOrder = sortOrder === 'asc' ? 'desc' : 'asc';
    } else {
      sortKey = key;
      sortOrder = 'asc';
    }
  }

  function isUrl(path: string) {
    return path.startsWith('http://') || path.startsWith('https://');
  }

  function getAppButtonLabel(path: string) {
    if (isUrl(path)) return 'ブラウザ';
    if (path.toLowerCase().endsWith('.xlsx')) return 'Excel';
    return '開く';
  }

  const themes: { name: string, value: Theme, color: string }[] = [
    { name: 'Zinc', value: 'zinc', color: 'bg-zinc-500' },
    { name: 'Blue', value: 'blue', color: 'bg-blue-500' },
    { name: 'Rose', value: 'rose', color: 'bg-rose-500' },
    { name: 'Green', value: 'green', color: 'bg-green-500' },
  ];
</script>

<main class="p-4 flex flex-col gap-4 h-screen max-w-full">
  <!-- Header -->
  <div class="flex gap-2 items-end">
    <div class="flex-[1.5] space-y-1">
      <label for="name" class="text-xs text-muted-foreground ml-1">名称</label>
      <Input id="name" bind:value={newName} placeholder="名称" class="h-8 text-sm" />
    </div>
    <div class="flex-[2] space-y-1">
      <label for="path" class="text-xs text-muted-foreground ml-1">URL / パス</label>
      <Input id="path" bind:value={newPath} placeholder="URL または パス" class="h-8 text-sm" />
    </div>
    <div class="flex-1 space-y-1">
      <label for="category" class="text-xs text-muted-foreground ml-1">カテゴリー</label>
      <Input id="category" bind:value={newCategory} placeholder="任意" class="h-8 text-sm" />
    </div>
    <Button onclick={addLink} size="sm" class="h-8">
      <Plus class="w-4 h-4 mr-1" />
      追加
    </Button>
    <div class="w-px h-8 bg-border mx-1"></div>
    <div class="flex-1 relative space-y-1">
      <label for="search" class="text-xs text-muted-foreground ml-1">検索</label>
      <div class="relative">
        <Search class="absolute left-2 top-1/2 -translate-y-1/2 w-3 h-3 text-muted-foreground" />
        <Input id="search" bind:value={searchQuery} placeholder="検索..." class="pl-7 h-8 text-sm" />
      </div>
    </div>
    <Button variant="ghost" size="icon" class="h-8 w-8" onclick={() => showSettings = !showSettings}>
      <Settings class="w-4 h-4" />
    </Button>
  </div>

  {#if showSettings}
    <div class="p-3 border rounded bg-muted/20 flex gap-4 items-center animate-in fade-in slide-in-from-top-1">
      <span class="text-xs font-medium">テーマ設定:</span>
      {#each themes as t}
        <button
          class="flex items-center gap-1.5 px-2 py-1 rounded border text-xs transition-colors {$settingsStore.theme === t.value ? 'bg-primary text-primary-foreground border-primary' : 'bg-background hover:bg-muted'}"
          onclick={() => settingsStore.setTheme(t.value)}
        >
          <div class="w-3 h-3 rounded-full {t.color}"></div>
          {t.name}
        </button>
      {/each}
    </div>
  {/if}

  <!-- Table View -->
  <div class="flex-1 border rounded bg-card overflow-hidden flex flex-col">
    <div class="overflow-auto flex-1">
      <table class="w-full text-xs text-left border-collapse">
        <thead class="bg-muted text-muted-foreground sticky top-0 z-10">
          <tr>
            <th class="px-3 py-2 border-b w-1/4 cursor-pointer hover:text-foreground transition-colors" onclick={() => toggleSort('name')}>
              <div class="flex items-center gap-1">
                名称
                {#if sortKey === 'name'}
                  {sortOrder === 'asc' ? '▲' : '▼'}
                {/if}
              </div>
            </th>
            <th class="px-3 py-2 border-b w-1/6 cursor-pointer hover:text-foreground transition-colors" onclick={() => toggleSort('category')}>
              <div class="flex items-center gap-1">
                カテゴリー
                {#if sortKey === 'category'}
                  {sortOrder === 'asc' ? '▲' : '▼'}
                {/if}
              </div>
            </th>
            <th class="px-3 py-2 border-b">パス</th>
            <th class="px-3 py-2 border-b text-right w-40">アクション</th>
          </tr>
        </thead>
        <tbody class="divide-y">
          {#each filteredLinks as link (link.id)}
            <tr class="hover:bg-muted/30 transition-colors">
              <td class="px-3 py-1.5 font-medium truncate">{link.name}</td>
              <td class="px-3 py-1.5 truncate">
                <span class="px-1.5 py-0.5 rounded-full bg-primary/10 text-primary text-[10px] font-semibold empty:hidden">
                  {link.category}
                </span>
              </td>
              <td class="px-3 py-1.5 text-muted-foreground truncate" title={link.path}>
                {link.path}
              </td>
              <td class="px-3 py-1.5 text-right space-x-1 whitespace-nowrap">
                <Button variant="ghost" size="icon" class="h-7 w-7" onclick={() => copyToClipboard(link.path)} title="コピー">
                  <Copy class="w-3.5 h-3.5" />
                </Button>

                {#if !isUrl(link.path)}
                  <Button variant="ghost" size="icon" class="h-7 w-7" onclick={() => revealInExplorer(link.path)} title="フォルダ">
                    <FolderOpen class="w-3.5 h-3.5" />
                  </Button>
                {/if}

                <Button variant="outline" size="sm" class="h-7 px-2 text-[10px]" onclick={() => openPath(link.path)}>
                  {#if isUrl(link.path)}
                    <ExternalLink class="w-3 h-3 mr-1" />
                  {:else}
                    <Play class="w-3 h-3 mr-1" />
                  {/if}
                  {getAppButtonLabel(link.path)}
                </Button>

                <Button variant="ghost" size="icon" class="h-7 w-7 text-destructive hover:bg-destructive/10" onclick={() => linkStore.remove(link.id)}>
                  <Trash2 class="w-3.5 h-3.5" />
                </Button>
              </td>
            </tr>
          {:else}
            <tr>
              <td colspan="4" class="px-3 py-10 text-center text-muted-foreground">
                リンクがありません
              </td>
            </tr>
          {/each}
        </tbody>
      </table>
    </div>
  </div>
</main>

<style>
  :global(body) {
    background-color: hsl(var(--background));
    color: hsl(var(--foreground));
    overflow: hidden;
  }
</style>

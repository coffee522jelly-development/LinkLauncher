<script lang="ts">
  import { onMount } from 'svelte';
  import { linkStore, type Link } from '$lib/linkStore';
  import { settingsStore, type Theme, type ViewMode } from '$lib/settingsStore';
  import { copyToClipboard, openPath, revealInExplorer, openTerminal, refreshTray } from '$lib/actions';
  import Button from '$lib/components/Button.svelte';
  import Input from '$lib/components/Input.svelte';
  import ContextMenu from '$lib/components/ContextMenu.svelte';
  import { Search, Plus, Trash2, Copy, Folder, FolderOpen, ExternalLink, Play, Settings, Download, Upload, Edit2, Check, X, LayoutList, LayoutGrid, ArrowUpDown, Terminal, Globe, File, Pin, PinOff, Star, Layers, ListChecks } from 'lucide-svelte';

  let newName = $state('');
  let newPath = $state('');
  let newCategory = $state('');
  let searchQuery = $state('');
  let sortKey = $state<'name' | 'category' | 'manual'>('manual');
  let sortOrder = $state<'asc' | 'desc'>('asc');
  let showSettings = $state(false);

  // Editing state
  let editingId = $state<string | null>(null);
  let editName = $state('');
  let editCategory = $state('');
  let editPath = $state('');

  // Group Modal state
  let showGroupModal = $state(false);
  let groupName = $state('');
  let groupCategory = $state('');
  let selectedLinkIds = $state<Set<string>>(new Set());

  // Context Menu state
  let contextMenu = $state<{ x: number, y: number, link: Link | null }>({ x: 0, y: 0, link: null });

  // Input references
  let searchInputRef = $state<HTMLInputElement>();

  onMount(() => {
    linkStore.load().then(() => {
      // Initialize tray with data
      setTimeout(() => refreshTray(), 1000);
    });
    settingsStore.load();

    const handleGlobalKeydown = (e: KeyboardEvent) => {
      if (e.key === 'Control' || e.key === 'Meta' || e.key === 'Shift' || e.key === 'Alt') return;

      // Ctrl + F or Cmd + F
      // Require strictly 'f' key or KeyF code, and prevent execution on shortcuts like Ctrl+C
      if ((e.ctrlKey || e.metaKey) && (e.code === 'KeyF' || e.key.toLowerCase() === 'f') && !e.shiftKey && !e.altKey && !e.isComposing) {
        // Double check it's definitely 'f' or 'KeyF' to avoid false positives in some browsers/IMEs
        if (e.key.toLowerCase() !== 'f' && e.code !== 'KeyF') return;

        e.preventDefault();
        searchInputRef?.focus();
        searchInputRef?.select();
      }
    };

    window.addEventListener('keydown', handleGlobalKeydown);
    return () => window.removeEventListener('keydown', handleGlobalKeydown);
  });

  const filteredLinks = $derived(
    $linkStore.filter((link) =>
      link.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
      link.path.toLowerCase().includes(searchQuery.toLowerCase()) ||
      link.category.toLowerCase().includes(searchQuery.toLowerCase())
    ).sort((a, b) => {
      // Pinned items always come first
      if (a.isPinned && !b.isPinned) return -1;
      if (!a.isPinned && b.isPinned) return 1;

      // If both are pinned or both are unpinned, apply selected sort
      if (sortKey === 'manual') return 0;
      const valA = (a[sortKey] || '').toLowerCase();
      const valB = (b[sortKey] || '').toLowerCase();
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

  async function addGroup() {
    if (groupName && selectedLinkIds.size > 0) {
      await linkStore.addGroup(groupName, groupCategory, Array.from(selectedLinkIds));
      groupName = '';
      groupCategory = '';
      selectedLinkIds.clear();
      showGroupModal = false;
    }
  }

  function toggleLinkSelection(id: string) {
    if (selectedLinkIds.has(id)) {
      selectedLinkIds.delete(id);
    } else {
      selectedLinkIds.add(id);
    }
    selectedLinkIds = new Set(selectedLinkIds); // trigger reactivity
  }

  function startEdit(link: Link) {
    editingId = link.id;
    editName = link.name;
    editCategory = link.category;
    editPath = link.path;
  }

  function cancelEdit() {
    editingId = null;
  }

  async function saveEdit() {
    if (editingId) {
      await linkStore.update(editingId, {
        name: editName,
        category: editCategory,
        path: editPath
      });
      editingId = null;
    }
  }

  function toggleSort(key: 'name' | 'category' | 'manual') {
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

  async function launchGroup(groupLink: Link) {
    if (!groupLink.linkIds) return;
    const linksToLaunch = $linkStore.filter(l => groupLink.linkIds!.includes(l.id) && !l.isGroup);

    // We launch them one by one.
    for (const link of linksToLaunch) {
      await openPath(link.path);
      // Small delay to prevent overwhelming the OS/Browser
      await new Promise(r => setTimeout(r, 200));
    }
  }

  function getAppButtonLabel(path: string) {
    if (isUrl(path)) return 'ブラウザ';
    if (path.toLowerCase().endsWith('.xlsx')) return 'Excel';
    if (path.includes('\\') || path.includes('/')) {
       const isFile = /\.[a-z0-9]+$/i.test(path);
       return isFile ? '開く' : 'フォルダ';
    }
    return '開く';
  }

  function handleContextMenu(e: MouseEvent, link: Link) {
    e.preventDefault();
    contextMenu = {
      x: e.clientX,
      y: e.clientY,
      link
    };
  }

  function closeContextMenu() {
    contextMenu = { x: 0, y: 0, link: null };
  }

  const themes: { name: string, value: Theme, color: string }[] = [
    { name: 'Zinc', value: 'zinc', color: 'bg-zinc-500' },
    { name: 'Blue', value: 'blue', color: 'bg-blue-500' },
    { name: 'Rose', value: 'rose', color: 'bg-rose-500' },
    { name: 'Green', value: 'green', color: 'bg-green-500' },
    { name: 'Orange', value: 'orange', color: 'bg-orange-500' },
    { name: 'Slate', value: 'slate', color: 'bg-slate-500' },
  ];
</script>

<main class="p-4 flex flex-col gap-4 h-screen max-w-full">
  <!-- Header -->
  <div class="flex flex-col gap-2">
    <!-- Row 1 -->
    <div class="flex gap-1.5 items-end">
      <!-- 表示切替ボタン -->
      <div class="flex gap-0.5 border rounded p-0.5 h-8 items-center bg-muted/20">
        <Button
          variant={$settingsStore.viewMode === 'table' ? 'secondary' : 'ghost'}
          size="icon" class="h-7 w-7"
          onclick={() => settingsStore.setViewMode('table')}
          title="リスト表示"
        >
          <LayoutList class="w-4 h-4" />
        </Button>
        <Button
          variant={$settingsStore.viewMode === 'grid' ? 'secondary' : 'ghost'}
          size="icon" class="h-7 w-7"
          onclick={() => settingsStore.setViewMode('grid')}
          title="ボタン表示"
        >
          <LayoutGrid class="w-4 h-4" />
        </Button>
      </div>

      <!-- 並び替えボタン -->
      <div class="flex flex-col space-y-1">
        <span class="text-[9px] text-muted-foreground ml-1">並び替え</span>
        <div class="flex border rounded h-8 overflow-hidden bg-background">
          <button
            class="px-2 text-[9px] hover:bg-muted border-r transition-colors {sortKey === 'manual' ? 'bg-primary/10 text-primary font-bold' : ''}"
            onclick={() => toggleSort('manual')}
          >
            カスタム
          </button>
          <button
            class="px-2 text-[9px] hover:bg-muted border-r transition-colors {sortKey === 'name' ? 'bg-primary/10 text-primary font-bold' : ''}"
            onclick={() => toggleSort('name')}
          >
            名前 {sortKey === 'name' ? (sortOrder === 'asc' ? '↑' : '↓') : ''}
          </button>
          <button
            class="px-2 text-[9px] hover:bg-muted transition-colors {sortKey === 'category' ? 'bg-primary/10 text-primary font-bold' : ''}"
            onclick={() => toggleSort('category')}
          >
            カテゴリ {sortKey === 'category' ? (sortOrder === 'asc' ? '↑' : '↓') : ''}
          </button>
        </div>
      </div>

      <!-- 設定ボタン -->
      <div class="flex flex-col space-y-1">
         <span class="text-[9px] text-muted-foreground ml-1">設定</span>
         <Button variant="ghost" size="icon" class="h-8 w-8 border" onclick={() => showSettings = !showSettings} title="設定">
           <Settings class="w-4 h-4" />
         </Button>
      </div>

      <div class="w-px h-8 bg-border mx-0.5"></div>

      <!-- 検索欄 -->
      <div class="flex-[1] relative space-y-1">
        <label for="search" class="text-[10px] text-muted-foreground ml-1">検索 (Ctrl+F)</label>
        <div class="relative">
          <Search class="absolute left-2 top-1/2 -translate-y-1/2 w-3 h-3 text-muted-foreground" />
          <Input id="search" bind:ref={searchInputRef} bind:value={searchQuery} placeholder="検索..." class="pl-7 h-8 text-xs" />
        </div>
      </div>

      <!-- 名称 -->
      <div class="flex-[1.5] space-y-1">
        <label for="name" class="text-[10px] text-muted-foreground ml-1">名称</label>
        <Input id="name" bind:value={newName} placeholder="名称" class="h-8 text-xs" />
      </div>

      <!-- カテゴリー -->
      <div class="flex-[1] space-y-1">
        <label for="category" class="text-[10px] text-muted-foreground ml-1">カテゴリー</label>
        <Input id="category" bind:value={newCategory} placeholder="任意" class="h-8 text-xs" />
      </div>
    </div>

    <!-- Row 2 -->
    <div class="flex gap-1.5 items-end">
      <!-- パス -->
      <div class="flex-1 space-y-1">
        <label for="path" class="text-[10px] text-muted-foreground ml-1">URL / パス</label>
        <Input id="path" bind:value={newPath} placeholder="URL または パス" class="h-8 text-xs" />
      </div>

      <!-- 追加ボタン -->
      <Button onclick={addLink} size="sm" class="h-8 px-5">
        <Plus class="w-4 h-4 mr-1" />
        追加
      </Button>
      <Button variant="secondary" onclick={() => showGroupModal = true} size="sm" class="h-8 px-3" title="グループ追加">
        <Layers class="w-4 h-4" />
      </Button>
    </div>
  </div>

  {#if showSettings}
    <div class="p-3 border rounded bg-muted/20 flex flex-wrap gap-6 items-center animate-in fade-in slide-in-from-top-1">
      <div class="flex items-center gap-3">
        <span class="text-xs font-medium">テーマ設定:</span>
        <div class="flex gap-2">
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
      </div>

      <div class="w-px h-6 bg-border"></div>

      <div class="flex items-center gap-3">
        <span class="text-xs font-medium">データ管理:</span>
        <div class="flex gap-2">
          <Button variant="outline" size="sm" class="h-7 text-[10px]" onclick={() => linkStore.exportToCSV()}>
            <Download class="w-3 h-3 mr-1" />
            CSVエクスポート
          </Button>
          <Button variant="outline" size="sm" class="h-7 text-[10px]" onclick={() => linkStore.importFromCSV()}>
            <Upload class="w-3 h-3 mr-1" />
            CSVインポート
          </Button>
        </div>
      </div>
    </div>
  {/if}

  <!-- Content View -->
  <div class="flex-1 border rounded bg-card overflow-hidden flex flex-col">
    <div class="overflow-auto flex-1">
      {#if $settingsStore.viewMode === 'table'}
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
              <th class="px-3 py-2 border-b text-right w-48">アクション</th>
            </tr>
          </thead>
          <tbody class="divide-y">
            {#each filteredLinks as link (link.id)}
              <tr
                class="hover:bg-muted/30 transition-colors {link.isPinned ? 'bg-primary/5' : ''}"
                oncontextmenu={(e) => handleContextMenu(e, link)}
              >
                {#if editingId === link.id}
                  <td class="px-2 py-1"><Input bind:value={editName} class="h-7 text-[10px] w-full" /></td>
                  <td class="px-2 py-1"><Input bind:value={editCategory} class="h-7 text-[10px] w-full" /></td>
                  <td class="px-2 py-1">
                    {#if !link.isGroup}
                      <Input bind:value={editPath} class="h-7 text-[10px] w-full" />
                    {/if}
                  </td>
                  <td class="px-3 py-1 text-right space-x-1">
                    <Button variant="outline" size="sm" class="h-7 w-7 p-0" onclick={saveEdit} title="保存">
                      <Check class="w-3.5 h-3.5 text-green-600" />
                    </Button>
                    <Button variant="outline" size="sm" class="h-7 w-7 p-0" onclick={cancelEdit} title="キャンセル">
                      <X class="w-3.5 h-3.5 text-destructive" />
                    </Button>
                  </td>
                {:else}
                  <td class="px-3 py-1.5 font-medium truncate">
                    <div class="flex items-center gap-1.5">
                      {#if link.isGroup}
                        <Layers class="w-3 h-3 text-primary" />
                      {:else if isUrl(link.path)}
                        <Globe class="w-3 h-3 text-blue-500" />
                      {:else}
                        <File class="w-3 h-3 text-zinc-500" />
                      {/if}
                      {link.name}
                    </div>
                  </td>
                  <td class="px-3 py-1.5 truncate">
                    <span class="px-1.5 py-0.5 rounded-full text-[10px] font-bold empty:hidden
                      {link.category === 'Web' ? 'bg-blue-100 text-blue-700 dark:bg-blue-900/30 dark:text-blue-300' :
                       link.category === 'Local' ? 'bg-zinc-100 text-zinc-700 dark:bg-zinc-800 dark:text-zinc-300' :
                       'bg-primary/10 text-primary'}">
                      {link.category}
                    </span>
                  </td>
                  <td class="px-3 py-1.5 text-muted-foreground truncate" title={link.isGroup ? `${link.linkIds?.length || 0} 個のリンク` : link.path}>
                    {#if link.isGroup}
                      <div class="flex items-center gap-1"><ListChecks class="w-3 h-3" /> {link.linkIds?.length || 0} 個のリンク</div>
                    {:else}
                      {link.path}
                    {/if}
                  </td>
                  <td class="px-3 py-1.5 text-right space-x-0.5 whitespace-nowrap">
                    <Button variant="ghost" size="icon" class="h-7 w-7 {link.isFavorite ? 'text-yellow-500' : 'text-muted-foreground'}" onclick={() => linkStore.toggleFavorite(link.id)} title={link.isFavorite ? "お気に入り解除" : "お気に入りに追加"}>
                      <Star class="w-3.5 h-3.5 {link.isFavorite ? 'fill-yellow-500' : ''}" />
                    </Button>
                    <Button variant="ghost" size="icon" class="h-7 w-7 {link.isPinned ? 'text-primary' : 'text-muted-foreground'}" onclick={() => linkStore.togglePin(link.id)} title={link.isPinned ? "ピン留め解除" : "ピン留め"}>
                      {#if link.isPinned}
                        <PinOff class="w-3.5 h-3.5" />
                      {:else}
                        <Pin class="w-3.5 h-3.5" />
                      {/if}
                    </Button>
                    <Button variant="ghost" size="icon" class="h-7 w-7" onclick={() => startEdit(link)} title="編集">
                      <Edit2 class="w-3.5 h-3.5" />
                    </Button>

                    {#if link.isGroup}
                      <Button variant="default" size="sm" class="h-7 px-3 text-[10px]" onclick={() => launchGroup(link)}>
                        <Layers class="w-3 h-3 mr-1" />
                        一括起動
                      </Button>
                    {:else}
                      <Button variant="ghost" size="icon" class="h-7 w-7" onclick={() => copyToClipboard(link.path)} title="コピー">
                        <Copy class="w-3.5 h-3.5" />
                      </Button>

                      {#if !isUrl(link.path)}
                        <Button variant="ghost" size="icon" class="h-7 w-7" onclick={() => openTerminal(link.path)} title="ターミナル">
                          <Terminal class="w-3.5 h-3.5" />
                        </Button>
                      {/if}

                      <Button variant="outline" size="sm" class="h-7 px-2 text-[10px]" onclick={() => openPath(link.path)}>
                        {#if isUrl(link.path)}
                          <ExternalLink class="w-3 h-3 mr-1" />
                        {:else if getAppButtonLabel(link.path) === 'フォルダ'}
                          <Folder class="w-3 h-3 mr-1" />
                        {:else}
                          <Play class="w-3 h-3 mr-1" />
                        {/if}
                        {getAppButtonLabel(link.path)}
                      </Button>

                      {#if !isUrl(link.path)}
                        <Button variant="outline" size="sm" class="h-7 px-2 text-[10px]" onclick={() => revealInExplorer(link.path)} title="フォルダ内を表示">
                          <FolderOpen class="w-3 h-3 mr-1" />
                          フォルダ内
                        </Button>
                      {/if}
                    {/if}

                    <Button variant="ghost" size="icon" class="h-7 w-7 text-destructive hover:bg-destructive/10" onclick={() => linkStore.remove(link.id)}>
                      <Trash2 class="w-3.5 h-3.5" />
                    </Button>
                  </td>
                {/if}
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
      {:else}
        <div class="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 gap-3 p-4">
          {#each filteredLinks as link, i (link.id)}
            <div
              role="listitem"
              oncontextmenu={(e) => handleContextMenu(e, link)}
              class="group relative border rounded-lg p-3 hover:border-primary/50 hover:shadow-md transition-all flex flex-col gap-2 cursor-grab active:cursor-grabbing
               {link.isPinned ? 'bg-primary/5 border-primary/30 ring-1 ring-primary/20' : 'bg-background'}"
              draggable={sortKey === 'manual' && searchQuery === ''}
              ondragstart={(e) => {
                e.dataTransfer?.setData('text/plain', link.id);
                (e.currentTarget as HTMLElement).classList.add('opacity-50');
              }}
              ondragend={(e) => {
                (e.currentTarget as HTMLElement).classList.remove('opacity-50');
              }}
              ondragover={(e) => {
                e.preventDefault();
                if (sortKey === 'manual') {
                  (e.currentTarget as HTMLElement).classList.add('border-primary');
                }
              }}
              ondragleave={(e) => {
                (e.currentTarget as HTMLElement).classList.remove('border-primary');
              }}
              ondrop={(e) => {
                e.preventDefault();
                (e.currentTarget as HTMLElement).classList.remove('border-primary');
                const fromId = e.dataTransfer?.getData('text/plain');
                if (fromId && fromId !== link.id) {
                  linkStore.reorder(fromId, link.id);
                }
              }}
            >
              <div class="flex justify-between items-start">
                <span class="text-[9px] px-1.5 py-0.5 rounded font-bold truncate empty:invisible
                  {link.category === 'Web' ? 'bg-blue-100 text-blue-700 dark:bg-blue-900/30 dark:text-blue-300' :
                   link.category === 'Local' ? 'bg-zinc-100 text-zinc-700 dark:bg-zinc-800 dark:text-zinc-300' :
                   'bg-primary/10 text-primary'}">
                  {link.category || 'なし'}
                </span>
                <div class="flex gap-2.5 transition-opacity">
                   <button class="p-1 hover:bg-muted rounded transition-colors hover:text-yellow-500 {link.isFavorite ? 'text-yellow-500 opacity-100' : 'opacity-0 group-hover:opacity-100'}" onclick={() => linkStore.toggleFavorite(link.id)} title={link.isFavorite ? "お気に入り解除" : "お気に入りに追加"}>
                     <Star class="w-3.5 h-3.5 {link.isFavorite ? 'fill-yellow-500' : ''}" />
                   </button>
                   <button class="p-1 hover:bg-muted rounded transition-colors hover:text-primary {link.isPinned ? 'text-primary opacity-100' : 'opacity-0 group-hover:opacity-100'}" onclick={() => linkStore.togglePin(link.id)} title={link.isPinned ? "ピン留め解除" : "ピン留め"}>
                     {#if link.isPinned}
                       <PinOff class="w-3.5 h-3.5" />
                     {:else}
                       <Pin class="w-3.5 h-3.5" />
                     {/if}
                   </button>
                   <button class="p-1 hover:bg-muted rounded transition-colors hover:text-primary opacity-0 group-hover:opacity-100" onclick={() => startEdit(link)} title="編集">
                     <Edit2 class="w-3.5 h-3.5" />
                   </button>
                   <button class="p-1 hover:bg-destructive/10 rounded transition-colors hover:text-destructive opacity-0 group-hover:opacity-100" onclick={() => linkStore.remove(link.id)} title="削除">
                     <Trash2 class="w-3.5 h-3.5" />
                   </button>
                </div>
              </div>

              <button
                class="flex-1 text-left py-1"
                onclick={() => link.isGroup ? launchGroup(link) : openPath(link.path)}
                title={link.isGroup ? `${link.linkIds?.length || 0} 個のリンク` : link.path}
              >
                <div class="flex items-center gap-1.5">
                  {#if link.isGroup}
                    <Layers class="w-3.5 h-3.5 text-primary shrink-0" />
                  {:else if isUrl(link.path)}
                    <Globe class="w-3.5 h-3.5 text-blue-500 shrink-0" />
                  {:else}
                    <File class="w-3.5 h-3.5 text-zinc-500 shrink-0" />
                  {/if}
                  <div class="font-bold text-sm leading-tight line-clamp-2">{link.name}</div>
                </div>
                {#if link.isGroup}
                   <div class="text-[10px] text-muted-foreground mt-1 text-left">{link.linkIds?.length || 0} 個のリンク</div>
                {/if}
              </button>

              <div class="flex items-center gap-1 pt-2 border-t mt-auto opacity-0 group-hover:opacity-100 transition-opacity flex-wrap">
                {#if link.isGroup}
                  <div class="flex-1"></div>
                  <Button variant="default" size="sm" class="h-7 px-3 text-[10px] w-full" onclick={() => launchGroup(link)}>
                    <Layers class="w-3 h-3 mr-1" />
                    一括起動
                  </Button>
                {:else}
                  <Button variant="ghost" size="icon" class="h-6 w-6" onclick={() => copyToClipboard(link.path)} title="コピー">
                    <Copy class="w-3 h-3" />
                  </Button>
                  {#if !isUrl(link.path)}
                    <Button variant="ghost" size="icon" class="h-6 w-6" onclick={() => openTerminal(link.path)} title="ターミナル">
                      <Terminal class="w-3 h-3" />
                    </Button>
                  {/if}
                  <div class="flex-1"></div>
                  <div class="flex gap-1.5 items-center">
                    <Button variant="outline" size="sm" class="h-7 px-2.5 text-[10px]" onclick={() => openPath(link.path)}>
                      {#if isUrl(link.path)}
                        <ExternalLink class="w-3 h-3 mr-1" />
                      {:else if getAppButtonLabel(link.path) === 'フォルダ'}
                        <Folder class="w-3 h-3 mr-1" />
                      {/if}
                      {getAppButtonLabel(link.path)}
                    </Button>
                    {#if !isUrl(link.path)}
                      <Button variant="outline" size="sm" class="h-7 px-2.5 text-[10px] bg-primary/5 border-primary/40 font-medium" onclick={() => revealInExplorer(link.path)} title="フォルダ内を表示">
                        <FolderOpen class="w-3 h-3 mr-1" />
                        フォルダ内
                      </Button>
                    {/if}
                  </div>
                {/if}
              </div>
            </div>
          {:else}
            <div class="col-span-full py-20 text-center text-muted-foreground">
              リンクがありません
            </div>
          {/each}
        </div>
      {/if}
    </div>
  </div>

  {#if contextMenu.link}
    <ContextMenu
      x={contextMenu.x}
      y={contextMenu.y}
      isPinned={!!contextMenu.link.isPinned}
      isFavorite={!!contextMenu.link.isFavorite}
      isUrl={isUrl(contextMenu.link.path)}
      isGroup={!!contextMenu.link.isGroup}
      label={contextMenu.link.isGroup ? '一括起動' : getAppButtonLabel(contextMenu.link.path)}
      onClose={closeContextMenu}
      onEdit={() => startEdit(contextMenu.link!)}
      onDelete={() => linkStore.remove(contextMenu.link!.id)}
      onOpen={() => contextMenu.link!.isGroup ? launchGroup(contextMenu.link!) : openPath(contextMenu.link!.path)}
      onCopy={() => copyToClipboard(contextMenu.link!.path)}
      onPin={() => linkStore.togglePin(contextMenu.link!.id)}
      onFavorite={() => linkStore.toggleFavorite(contextMenu.link!.id)}
      onReveal={() => revealInExplorer(contextMenu.link!.path)}
      onTerminal={() => openTerminal(contextMenu.link!.path)}
    />
  {/if}

  {#if showGroupModal}
    <div class="fixed inset-0 bg-background/80 backdrop-blur-sm z-50 flex items-center justify-center p-4">
      <div class="bg-card border rounded-lg shadow-lg w-full max-w-xl p-4 flex flex-col max-h-[90vh] animate-in fade-in zoom-in-95">
        <div class="flex items-center gap-2 mb-4 shrink-0">
          <Layers class="w-5 h-5 text-primary" />
          <h3 class="font-bold">グループの作成</h3>
        </div>

        <div class="space-y-3 mb-4 shrink-0">
          <div class="flex gap-3">
            <div class="flex-[2] space-y-1">
              <label for="group-name" class="text-xs text-muted-foreground">グループ名 <span class="text-destructive">*</span></label>
              <Input id="group-name" bind:value={groupName} class="h-8" placeholder="朝のルーティン 等" />
            </div>
            <div class="flex-[1] space-y-1">
              <label for="group-cat" class="text-xs text-muted-foreground">カテゴリー</label>
              <Input id="group-cat" bind:value={groupCategory} class="h-8" placeholder="任意" />
            </div>
          </div>
        </div>

        <div class="text-xs text-muted-foreground mb-2 shrink-0 flex justify-between items-end">
          <span>起動するリンクを選択:</span>
          <span class="font-medium {selectedLinkIds.size === 0 ? 'text-destructive' : 'text-primary'}">{selectedLinkIds.size} 件選択中</span>
        </div>

        <div class="flex-1 overflow-y-auto border rounded-md p-2 space-y-1 bg-muted/30">
          {#each $linkStore.filter(l => !l.isGroup) as link (link.id)}
            <label class="flex items-center gap-3 p-2 hover:bg-background rounded border border-transparent hover:border-border cursor-pointer transition-colors {selectedLinkIds.has(link.id) ? 'bg-background border-primary/50 shadow-sm' : ''}">
              <input
                type="checkbox"
                class="rounded border-input text-primary focus:ring-primary h-4 w-4"
                checked={selectedLinkIds.has(link.id)}
                onchange={() => toggleLinkSelection(link.id)}
              />
              <div class="flex flex-col overflow-hidden">
                <span class="text-sm font-medium truncate">{link.name}</span>
                <span class="text-[10px] text-muted-foreground truncate">{link.path}</span>
              </div>
            </label>
          {:else}
             <div class="p-4 text-center text-xs text-muted-foreground">登録されているリンクがありません</div>
          {/each}
        </div>

        <div class="flex justify-end gap-2 pt-4 shrink-0">
          <Button variant="outline" size="sm" onclick={() => showGroupModal = false}>キャンセル</Button>
          <Button size="sm" onclick={addGroup} disabled={!groupName || selectedLinkIds.size === 0}>作成</Button>
        </div>
      </div>
    </div>
  {/if}

  {#if editingId && $settingsStore.viewMode === 'grid'}
    <div class="fixed inset-0 bg-background/80 backdrop-blur-sm z-50 flex items-center justify-center p-4">
      <div class="bg-card border rounded-lg shadow-lg w-full max-w-md p-4 space-y-4 animate-in fade-in zoom-in-95">
        <h3 class="font-bold text-sm">リンクの編集</h3>
        <div class="space-y-3">
          <div class="space-y-1">
            <label for="edit-name" class="text-[10px] text-muted-foreground">名称</label>
            <Input id="edit-name" bind:value={editName} class="h-8 text-sm" />
          </div>
          <div class="space-y-1">
            <label for="edit-cat" class="text-[10px] text-muted-foreground">カテゴリー</label>
            <Input id="edit-cat" bind:value={editCategory} class="h-8 text-sm" />
          </div>
          {#if !($linkStore.find(l => l.id === editingId)?.isGroup)}
            <div class="space-y-1">
              <label for="edit-path" class="text-[10px] text-muted-foreground">URL / パス</label>
              <Input id="edit-path" bind:value={editPath} class="h-8 text-sm" />
            </div>
          {/if}
        </div>
        <div class="flex justify-end gap-2 pt-2">
          <Button variant="outline" size="sm" onclick={cancelEdit}>キャンセル</Button>
          <Button size="sm" onclick={saveEdit}>保存</Button>
        </div>
      </div>
    </div>
  {/if}
</main>

<style>
  :global(body) {
    background-color: hsl(var(--background));
    color: hsl(var(--foreground));
    overflow: hidden;
  }
</style>

<script lang="ts">
  import { onMount } from 'svelte';
  import { linkStore } from '$lib/linkStore';
  import { copyToClipboard, openPath, revealInExplorer } from '$lib/actions';
  import Button from '$lib/components/Button.svelte';
  import Input from '$lib/components/Input.svelte';
  import { Search, Plus, Trash2, Copy, FolderOpen, ExternalLink, Play } from 'lucide-svelte';

  let newName = $state('');
  let newPath = $state('');
  let searchQuery = $state('');

  onMount(() => {
    linkStore.load();
  });

  const filteredLinks = $derived(
    $linkStore.filter((link) =>
      link.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
      link.path.toLowerCase().includes(searchQuery.toLowerCase())
    )
  );

  async function addLink() {
    if (newName && newPath) {
      await linkStore.add(newName, newPath);
      newName = '';
      newPath = '';
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
</script>

<main class="p-4 flex flex-col gap-4 h-screen max-w-full">
  <!-- Search and Add Compact Header -->
  <div class="flex gap-2 items-end">
    <div class="flex-1 space-y-1">
      <label for="name" class="text-xs text-muted-foreground ml-1">名称</label>
      <Input id="name" bind:value={newName} placeholder="名称" class="h-8 text-sm" />
    </div>
    <div class="flex-[2] space-y-1">
      <label for="path" class="text-xs text-muted-foreground ml-1">URL / パス</label>
      <Input id="path" bind:value={newPath} placeholder="URL または ファイルパス" class="h-8 text-sm" />
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
  </div>

  <!-- Table View -->
  <div class="flex-1 border rounded bg-card overflow-hidden flex flex-col">
    <div class="overflow-auto flex-1">
      <table class="w-full text-xs text-left border-collapse">
        <thead class="bg-muted text-muted-foreground sticky top-0 z-10">
          <tr>
            <th class="px-3 py-2 border-b w-1/4">名称</th>
            <th class="px-3 py-2 border-b">パス</th>
            <th class="px-3 py-2 border-b text-right w-40">アクション</th>
          </tr>
        </thead>
        <tbody class="divide-y">
          {#each filteredLinks as link (link.id)}
            <tr class="hover:bg-muted/30 transition-colors">
              <td class="px-3 py-1.5 font-medium truncate">{link.name}</td>
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
              <td colspan="3" class="px-3 py-10 text-center text-muted-foreground">
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
  :global(:root) {
    --background: 0 0% 100%;
    --foreground: 240 10% 3.9%;
    --card: 0 0% 100%;
    --muted: 240 4.8% 95.9%;
    --muted-foreground: 240 3.8% 46.1%;
    --border: 240 5.9% 90%;
  }
  @media (prefers-color-scheme: dark) {
    :global(:root) {
      --background: 240 10% 3.9%;
      --foreground: 0 0% 98%;
      --card: 240 10% 3.9%;
      --muted: 240 3.7% 15.9%;
      --muted-foreground: 240 5% 64.9%;
      --border: 240 3.7% 15.9%;
    }
  }
</style>

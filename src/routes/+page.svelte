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
    if (isUrl(path)) return 'ブラウザで開く';
    if (path.toLowerCase().endsWith('.xlsx')) return 'Excelで開く';
    return 'アプリで開く';
  }
</script>

<main class="container mx-auto p-4 max-w-4xl">
  <h1 class="text-2xl font-bold mb-6 text-center">リンクランチャー</h1>

  <!-- Registration Form -->
  <div class="bg-card border rounded-lg p-4 mb-8 shadow-sm">
    <h2 class="text-lg font-semibold mb-4">新規登録</h2>
    <div class="grid grid-cols-1 md:grid-cols-2 gap-4 mb-4">
      <div>
        <label for="name" class="block text-sm font-medium mb-1">名称</label>
        <Input id="name" bind:value={newName} placeholder="例: 業務マニュアル" />
      </div>
      <div>
        <label for="path" class="block text-sm font-medium mb-1">URL または ファイルパス</label>
        <Input id="path" bind:value={newPath} placeholder="https://... または C:\..." />
      </div>
    </div>
    <Button onclick={addLink} class="w-full md:w-auto">
      <Plus class="w-4 h-4 mr-2" />
      登録
    </Button>
  </div>

  <!-- Search and List -->
  <div class="space-y-4">
    <div class="relative">
      <Search class="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-muted-foreground" />
      <Input
        bind:value={searchQuery}
        placeholder="名称で検索..."
        class="pl-10"
      />
    </div>

    <div class="border rounded-lg overflow-hidden bg-card">
      <table class="w-full text-sm text-left">
        <thead class="bg-muted text-muted-foreground font-medium border-b">
          <tr>
            <th class="px-4 py-3">名称</th>
            <th class="px-4 py-3">パス</th>
            <th class="px-4 py-3 text-right">アクション</th>
          </tr>
        </thead>
        <tbody class="divide-y">
          {#each filteredLinks as link (link.id)}
            <tr class="hover:bg-muted/50 transition-colors">
              <td class="px-4 py-3 font-medium">{link.name}</td>
              <td class="px-4 py-3 text-muted-foreground truncate max-w-[200px]" title={link.path}>
                {link.path}
              </td>
              <td class="px-4 py-3 text-right space-x-1 whitespace-nowrap">
                <Button variant="outline" size="icon" onclick={() => copyToClipboard(link.path)} title="パスをコピー">
                  <Copy class="w-4 h-4" />
                </Button>

                {#if !isUrl(link.path)}
                  <Button variant="outline" size="icon" onclick={() => revealInExplorer(link.path)} title="エクスプローラーで開く">
                    <FolderOpen class="w-4 h-4" />
                  </Button>
                {/if}

                <Button variant="outline" onclick={() => openPath(link.path)} title={getAppButtonLabel(link.path)}>
                  {#if isUrl(link.path)}
                    <ExternalLink class="w-4 h-4 mr-1" />
                    開く
                  {:else}
                    <Play class="w-4 h-4 mr-1" />
                    開く
                  {/if}
                </Button>

                <Button variant="ghost" size="icon" onclick={() => linkStore.remove(link.id)} class="text-destructive hover:text-destructive hover:bg-destructive/10">
                  <Trash2 class="w-4 h-4" />
                </Button>
              </td>
            </tr>
          {:else}
            <tr>
              <td colspan="3" class="px-4 py-8 text-center text-muted-foreground">
                登録されているリンクはありません
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
  }
  :global(:root) {
    --background: 0 0% 100%;
    --foreground: 240 10% 3.9%;
    --card: 0 0% 100%;
    --muted: 240 4.8% 95.9%;
    --muted-foreground: 240 3.8% 46.1%;
  }
  @media (prefers-color-scheme: dark) {
    :global(:root) {
      --background: 240 10% 3.9%;
      --foreground: 0 0% 98%;
      --card: 240 10% 3.9%;
      --muted: 240 3.7% 15.9%;
      --muted-foreground: 240 5% 64.9%;
    }
  }
</style>

<script lang="ts">
  import { Edit2, Trash2, Play, Copy, Pin, PinOff, ExternalLink, FolderOpen, Terminal, Star } from 'lucide-svelte';
  import { onMount } from 'svelte';

  interface Props {
    x: number;
    y: number;
    isPinned: boolean;
    isFavorite: boolean;
    isUrl: boolean;
    label: string;
    onClose: () => void;
    onEdit: () => void;
    onDelete: () => void;
    onOpen: () => void;
    onCopy: () => void;
    onPin: () => void;
    onFavorite: () => void;
    onReveal?: () => void;
    onTerminal?: () => void;
  }

  let {
    x, y, isPinned, isFavorite, isUrl, label,
    onClose, onEdit, onDelete, onOpen, onCopy, onPin, onFavorite, onReveal, onTerminal
  }: Props = $props();

  let menuElement: HTMLDivElement;

  onMount(() => {
    // Adjust position if menu goes off screen
    const rect = menuElement.getBoundingClientRect();
    const screenWidth = window.innerWidth;
    const screenHeight = window.innerHeight;

    if (x + rect.width > screenWidth) x = screenWidth - rect.width - 10;
    if (y + rect.height > screenHeight) y = screenHeight - rect.height - 10;
  });

  function handleAction(action: () => void) {
    action();
    onClose();
  }
</script>

<!-- svelte-ignore a11y_click_events_have_key_events -->
<!-- svelte-ignore a11y_no_static_element_interactions -->
<div
  class="fixed inset-0 z-[100]"
  onclick={onClose}
  oncontextmenu={(e) => { e.preventDefault(); onClose(); }}
></div>

<div
  bind:this={menuElement}
  class="fixed z-[101] w-48 bg-popover border rounded-lg shadow-xl py-1 animate-in fade-in zoom-in-95 duration-100"
  style="left: {x}px; top: {y}px;"
>
  <button
    class="w-full px-3 py-1.5 text-left text-xs flex items-center gap-2 hover:bg-muted transition-colors"
    onclick={() => handleAction(onEdit)}
  >
    <Edit2 class="w-3.5 h-3.5" />
    編集
  </button>

  <button
    class="w-full px-3 py-1.5 text-left text-xs flex items-center gap-2 hover:bg-muted transition-colors text-destructive"
    onclick={() => handleAction(onDelete)}
  >
    <Trash2 class="w-3.5 h-3.5" />
    削除
  </button>

  <div class="h-px bg-border my-1"></div>

  <button
    class="w-full px-3 py-1.5 text-left text-xs flex items-center gap-2 hover:bg-muted transition-colors font-medium"
    onclick={() => handleAction(onOpen)}
  >
    {#if isUrl}
      <ExternalLink class="w-3.5 h-3.5 text-blue-500" />
    {:else}
      <Play class="w-3.5 h-3.5 text-primary" />
    {/if}
    {label}
  </button>

  <button
    class="w-full px-3 py-1.5 text-left text-xs flex items-center gap-2 hover:bg-muted transition-colors"
    onclick={() => handleAction(onCopy)}
  >
    <Copy class="w-3.5 h-3.5" />
    パスをコピー
  </button>

  <div class="h-px bg-border my-1"></div>

  <button
    class="w-full px-3 py-1.5 text-left text-xs flex items-center gap-2 hover:bg-muted transition-colors"
    onclick={() => handleAction(onPin)}
  >
    {#if isPinned}
      <PinOff class="w-3.5 h-3.5 text-primary" />
      ピン留め解除
    {:else}
      <Pin class="w-3.5 h-3.5" />
      ピン留め
    {/if}
  </button>

  <button
    class="w-full px-3 py-1.5 text-left text-xs flex items-center gap-2 hover:bg-muted transition-colors"
    onclick={() => handleAction(onFavorite)}
  >
    <Star class="w-3.5 h-3.5 {isFavorite ? 'text-yellow-500 fill-yellow-500' : ''}" />
    {isFavorite ? 'お気に入り解除' : 'お気に入りに追加'}
  </button>

  {#if !isUrl}
    <div class="h-px bg-border my-1"></div>

    <button
      class="w-full px-3 py-1.5 text-left text-xs flex items-center gap-2 hover:bg-muted transition-colors"
      onclick={() => handleAction(onReveal!)}
    >
      <FolderOpen class="w-3.5 h-3.5" />
      フォルダを開く
    </button>

    <button
      class="w-full px-3 py-1.5 text-left text-xs flex items-center gap-2 hover:bg-muted transition-colors"
      onclick={() => handleAction(onTerminal!)}
    >
      <Terminal class="w-3.5 h-3.5" />
      ターミナルを開く
    </button>
  {/if}
</div>

<script lang="ts">
  import type { HTMLInputAttributes } from 'svelte/elements';
  import { cn } from '$lib/utils';

  let {
    class: className = '',
    value = $bindable(),
    ...rest
  }: {
    class?: string;
    value?: string;
  } & HTMLInputAttributes = $props();

  const baseStyles = 'flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50';
</script>

<input
  class={cn(baseStyles, className)}
  bind:value={value}
  {...rest}
/>

<style>
  :global(:root) {
    --muted-foreground: 240 3.8% 46.1%;
  }
  @media (prefers-color-scheme: dark) {
    :global(:root) {
      --muted-foreground: 240 5% 64.9%;
    }
  }
</style>

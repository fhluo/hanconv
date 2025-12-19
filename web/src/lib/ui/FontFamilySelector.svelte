<script lang="ts">
  import { Check } from "@lucide/svelte";

  type Font = "Sans Serif" | "Serif" | "Monospace";

  let fonts: Font[] = ["Sans Serif", "Serif", "Monospace"];

  let selected = $state<Font>("Sans Serif");
</script>

{#snippet fontOption(font: Font)}
  <button
    class={[
      "flex w-full px-2 py-1.5 text-xs rounded items-center select-none outline-none transition-colors",
      font === selected
        ? "text-gray-900 dark:text-gray-100 bg-gray-100 dark:bg-gray-800"
        : "text-gray-600 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-800",
    ]}
    onclick={() => (selected = font)}
  >
    <div class="flex items-center justify-center w-4 mr-1.5 shrink-0">
      {#if font === selected}
        <Check
          size={12}
          strokeWidth={2.5}
          class="text-blue-600 dark:text-blue-400"
        />
      {/if}
    </div>
    <span class="font-medium">{font}</span>
  </button>
{/snippet}

<div class="flex flex-col gap-1">
  <span class="text-[10px] font-medium text-gray-400 tracking-wider px-2">字体</span>
  {#each fonts as font}
    {@render fontOption(font)}
  {/each}
</div>

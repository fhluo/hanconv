<script lang="ts">
  import { Type } from "@lucide/svelte";
  import FontFamilySelector, { type Font } from "./FontFamilySelector.svelte";
  import FontSizeControl from "./FontSizeControl.svelte";

  interface Props {
    fontFamily?: Font;
    fontSize?: number;
  }

  let {
    fontFamily = $bindable("Sans Serif"),
    fontSize = $bindable(16),
  }: Props = $props();

  let isOpen = $state(false);
</script>

<div class="relative">
  <button
    onclick={() => {
      isOpen = !isOpen;
    }}
    class={[
      "p-1.5 rounded-md text-gray-500 dark:text-gray-400 outline-none",
      "transition-colors hover:bg-gray-100 dark:hover:bg-gray-800",
      isOpen ? "bg-gray-100 dark:bg-gray-800" : "",
    ]}
  >
    <Type size={14} />
  </button>

  {#if isOpen}
    <div
      class="fixed inset-0 z-40"
      onclick={() => (isOpen = false)}
      role="presentation"
    ></div>

    <div
      class={[
        "absolute flex flex-col gap-2 bottom-full right-0 mb-2 min-w-50 p-2 z-50",
        "shadow-lg rounded-lg bg-white dark:bg-gray-900 border border-gray-200 dark:border-gray-800",
        "animate-in fade-in zoom-in-95 slide-in-from-bottom-2 duration-200 ease-out origin-bottom-right",
      ]}
    >
      <FontFamilySelector bind:selected={fontFamily} />
      <div class="h-px my-1 bg-gray-100 dark:bg-gray-800"></div>
      <FontSizeControl bind:size={fontSize} />
    </div>
  {/if}
</div>

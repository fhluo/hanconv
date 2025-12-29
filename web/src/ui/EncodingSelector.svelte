<script lang="ts">
  import { Check, ChevronUp } from "@lucide/svelte";

  export type Encoding =
    | "Auto"
    | "UTF-8"
    | "GBK"
    | "GB2312"
    | "GB18030"
    | "Big5"
    | "Shift-JIS"
    | "UTF-16";

  const encodings: Encoding[] = [
    "Auto",
    "UTF-8",
    "GBK",
    "GB2312",
    "GB18030",
    "Big5",
    "Shift-JIS",
    "UTF-16",
  ];

  interface Props {
    selected?: Encoding;
  }

  let { selected = $bindable("Auto") }: Props = $props();

  let isOpen = $state(false);
</script>

{#snippet encodingOption(encoding: Encoding)}
  <button
    class={[
      "flex items-center w-full px-2 py-1 text-xs rounded select-none outline-none transition-colors",
      selected === encoding
        ? "text-gray-900 dark:text-gray-100 bg-gray-100 dark:bg-gray-800"
        : "text-gray-600 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-800",
    ]}
    onclick={() => {
      selected = encoding;
      isOpen = false;
    }}
  >
    <div class="w-4 flex items-center justify-center mr-1.5 shrink-0">
      {#if selected === encoding}
        <Check
          size={12}
          strokeWidth={2.5}
          class="text-blue-600 dark:text-blue-400"
        />
      {/if}
    </div>
    <span class="font-mono font-medium">
      {encoding}
    </span>
  </button>
{/snippet}

<div class="relative">
  <button
    class={[
      "flex items-center gap-1.5 px-2 py-1 rounded-md",
      "hover:bg-gray-100 dark:hover:bg-gray-800 transition-colors",
      "text-xs font-mono font-medium text-gray-600 dark:text-gray-400 outline-none",
    ]}
    onclick={() => (isOpen = !isOpen)}
  >
    <span>{selected}</span>
    <div class="transition-transform duration-200" class:rotate-180={isOpen}>
      <ChevronUp size={12} />
    </div>
  </button>

  {#if isOpen}
    <div
      class="fixed inset-0 z-30"
      onclick={() => (isOpen = false)}
      role="presentation"
    ></div>
    <div
      class={[
        "absolute bottom-full right-0 mb-2 min-w-31.25 p-1",
        "bg-white dark:bg-gray-900 rounded-lg shadow-lg",
        "flex flex-col border border-gray-200 dark:border-gray-800 z-50",
        "animate-in fade-in zoom-in-95 slide-in-from-bottom-2 duration-200 ease-out origin-bottom-right",
      ]}
    >
      {#each encodings as encoding}
        {@render encodingOption(encoding)}
      {/each}
    </div>
  {/if}
</div>

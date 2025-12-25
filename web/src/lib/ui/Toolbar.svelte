<script lang="ts">
  import {
    Check,
    ClipboardPaste,
    Copy,
    FolderOpen,
    Save,
    Trash2,
  } from "@lucide/svelte";

  let title = $state("");
  let readonly = $state(false);
  let isEmpty = $state(true);
  let copied = $state(false);
  let pasted = $state(false);
</script>

<div
  class={[
    "relative flex items-center justify-between px-4 py-3 select-none z-10",
    "border-b border-gray-100/50 dark:border-gray-800/50 transition-colors duration-300",
  ]}
  data-tauri-drag-region
>
  <div
    class={[
      "absolute -bottom-px left-0 right-0 h-px opacity-0 group-focus-within:opacity-100",
      "bg-gray-300/40 dark:bg-gray-500/40 transition-all duration-1000 ease-out",
    ]}
    style="mask-image: linear-gradient(to right, transparent, black, transparent);"
  ></div>

  <span
    class="text-[10px] font-bold text-gray-400 dark:text-gray-500 tracking-wide transition-colors"
    data-tauri-drag-region>{title}</span
  >

  <div class="flex gap-1">
    {#if !readonly}
      <button
        class={[
          "p-1.5 rounded-md transition-colors duration-500 ease-in-out",
          "text-gray-400 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700",
        ]}
      >
        <FolderOpen size={14} />
      </button>
    {/if}

    <button
      class={[
        "p-1.5 rounded-md  transition-colors duration-500 ease-in-out",
        "text-gray-400 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700",
        "disabled:opacity-30 disabled:hover:bg-transparent",
      ]}
      disabled={isEmpty}
    >
      <Save size={14} />
    </button>

    {#if !readonly && !isEmpty}
      <button
        class={[
          "p-1.5 rounded-md transition-colors duration-500 ease-in-out",
          "text-gray-400 dark:text-gray-300 hover:bg-red-50 dark:hover:bg-red-900/20",
          "hover:text-red-600 dark:hover:text-red-400",
        ]}
      >
        <Trash2 size={14} />
      </button>
    {/if}

    <button
      class={[
        "p-1.5 rounded-md transition-colors",
        copied
          ? "text-green-600 dark:text-green-400 bg-green-50 dark:bg-green-900/20"
          : "text-gray-400 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700",
      ]}
      disabled={isEmpty}
    >
      {#if copied}
        <Check size={14} />
      {:else}
        <Copy size={14} />
      {/if}
    </button>

    <button
      class={[
        "p-1.5 rounded-md transition-colors",
        pasted
          ? "text-green-600 dark:text-green-400 bg-green-50 dark:bg-green-900/20"
          : "text-gray-400 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700",
      ]}
    >
      {#if pasted}
        <Check size={14} />
      {:else}
        <ClipboardPaste size={14} />
      {/if}
    </button>
  </div>
</div>

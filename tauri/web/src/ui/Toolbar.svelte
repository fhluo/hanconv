<script lang="ts">
  import {
    Check,
    Clipboard,
    Copy,
    FolderOpen,
    Save,
    Trash2,
  } from "@lucide/svelte";
  import { t } from "../lib/i18n.svelte";

  interface Props {
    title: string;
    readonly?: boolean;
    isEmpty?: boolean;
    copied?: boolean;
    pasted?: boolean;
    onOpen?: () => void;
    onSave?: () => void;
    onClear?: () => void;
    onCopy?: () => void;
    onPaste?: () => void;
  }

  let {
    title,
    readonly = false,
    isEmpty = true,
    copied = false,
    pasted = false,
    onOpen,
    onSave,
    onClear,
    onCopy,
    onPaste,
  }: Props = $props();
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
        onclick={onOpen}
        title={t("Open File")}
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
      onclick={onSave}
      title={t("Save File")}
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
        onclick={onClear}
        title={t("Clear")}
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
      onclick={onCopy}
      title={t("Copy")}
    >
      {#if copied}
        <Check size={14} />
      {:else}
        <Copy size={14} />
      {/if}
    </button>

    {#if !readonly}
      <button
        class={[
          "p-1.5 rounded-md transition-colors",
          pasted
            ? "text-green-600 dark:text-green-400 bg-green-50 dark:bg-green-900/20"
            : "text-gray-400 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700",
        ]}
        onclick={onPaste}
        title={t("Paste")}
      >
        {#if pasted}
          <Check size={14} />
        {:else}
          <Clipboard size={14} />
        {/if}
      </button>
    {/if}
  </div>
</div>

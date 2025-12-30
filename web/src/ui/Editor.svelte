<script lang="ts">
  import Toolbar from "./Toolbar.svelte";
  import DragOverlay from "./DragOverlay.svelte";
  import { LoaderCircle } from "@lucide/svelte";
  import { OverlayScrollbarsComponent } from "overlayscrollbars-svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { getCurrentWebview } from "@tauri-apps/api/webview";
  import { readText, writeText } from "@tauri-apps/plugin-clipboard-manager";
  import { open, save } from "@tauri-apps/plugin-dialog";
  import { writeTextFile } from "@tauri-apps/plugin-fs";
  import type { Encoding } from "./EncodingSelector.svelte";
  import type { Font } from "./FontFamilySelector.svelte";
  import { t } from "../lib/i18n.svelte";

  interface Props {
    value: string;
    placeholder?: string;
    readonly?: boolean;
    label: string;
    encoding?: Encoding;
    isLoading?: boolean;
    fontFamily?: Font;
    fontSize?: number;
    onScroll?: (scrollTop: number) => void;
    onEncodingDetected?: (encoding: Encoding) => void;
  }

  let {
    value = $bindable(),
    placeholder = "",
    readonly = false,
    label,
    encoding = "Auto",
    isLoading = false,
    fontFamily = "Sans Serif",
    fontSize = 16,
    onScroll,
    onEncodingDetected,
  }: Props = $props();

  let container: HTMLDivElement | undefined;
  let osComponent: any;
  let currentFilePath = $state<string | null>(null);
  let loadToken = 0;
  let isDragging = $state(false);
  let isFileLoading = $state(false);
  let copied = $state(false);
  let pasted = $state(false);
  let ignoreScroll = 0;

  const fileFilters = [
    { name: t("Text Files"), extensions: ["txt", "md"] },
    { name: t("All Files"), extensions: ["*"] },
  ];

  function pulse(target: "copied" | "pasted", ms = 2000) {
    if (target === "copied") {
      copied = true;
    } else {
      pasted = true;
    }

    setTimeout(
      () => (target === "copied" ? (copied = false) : (pasted = false)),
      ms,
    );
  }

  function hitTest(pos: { x: number; y: number }) {
    const rect = container?.getBoundingClientRect();
    return (
      rect &&
      pos.x >= rect.left &&
      pos.x <= rect.right &&
      pos.y >= rect.top &&
      pos.y <= rect.bottom
    );
  }

  async function loadFile(path: string, enc: Encoding) {
    const token = ++loadToken;

    isFileLoading = true;
    try {
      const [text, detected] = await invoke<[string, string]>(
        "read_text_file",
        {
          path,
          encoding: enc === "Auto" ? null : enc,
        },
      );

      if (token !== loadToken || path !== currentFilePath) {
        return;
      }

      if (text !== value) {
        value = text;
      }

      onEncodingDetected?.(detected as Encoding);
    } catch (e) {
      console.error("Failed to load file:", e);
    } finally {
      if (token === loadToken) {
        isFileLoading = false;
      }
    }
  }

  export async function openFile() {
    const selected = await open({
      title: t("Open File"),
      multiple: false,
      filters: fileFilters,
    }).catch(() => null);

    if (selected) {
      currentFilePath = selected;
    }
  }

  export async function saveFile() {
    if (!value) {
      return;
    }

    const filePath = await save({
      title: t("Save File"),
      filters: fileFilters,
    }).catch(() => null);

    if (!filePath) {
      return;
    }

    isFileLoading = true;
    try {
      await writeTextFile(filePath, value);
      currentFilePath = filePath;
    } finally {
      isFileLoading = false;
    }
  }

  export function clear() {
    value = "";
    currentFilePath = null;
  }

  export async function copy() {
    if (!value) {
      return;
    }

    await writeText(value);
    pulse("copied");
  }

  export async function paste() {
    if (readonly) {
      return;
    }

    const text = await readText().catch(() => null);
    if (text) {
      value = text;
      pulse("pasted");
    }
  }

  export function scrollTo(top: number) {
    const viewport = osComponent?.osInstance?.()?.elements()?.viewport;

    if (!viewport || Math.abs(viewport.scrollTop - top) <= 1) {
      return;
    }

    ignoreScroll++;
    viewport.scrollTop = top;
    requestAnimationFrame(() => (ignoreScroll = Math.max(0, ignoreScroll - 1)));
  }

  function handleScroll(top: number) {
    if (onScroll && ignoreScroll === 0) {
      onScroll(top);
    }
  }

  $effect(() => {
    if (currentFilePath && encoding) {
      loadFile(currentFilePath, encoding);
    }
  });

  $effect(() => {
    let unlisten: (() => void) | undefined;
    getCurrentWebview()
      .onDragDropEvent(({ payload }) => {
        if (readonly || !container || payload.type === "leave") {
          return (isDragging = false);
        }

        const inside = hitTest(payload.position) ?? false;
        if (payload.type === "over") {
          isDragging = inside;
        } else {
          if (inside) {
            currentFilePath = payload.paths[0] ?? null;
          }
          isDragging = false;
        }
      })
      .then((u) => (unlisten = u));
    return () => unlisten?.();
  });
</script>

<div
  class="flex flex-col h-full group relative bg-transparent"
  bind:this={container}
>
  <Toolbar
    title={label}
    {readonly}
    isEmpty={!value}
    {copied}
    {pasted}
    onOpen={openFile}
    onSave={saveFile}
    onClear={clear}
    onCopy={copy}
    onPaste={paste}
  />

  <div class="relative flex-1 w-full min-h-0" role="region" aria-label={label}>
    {#if isDragging}
      <DragOverlay />
    {/if}

    <OverlayScrollbarsComponent
      element="div"
      options={{
        scrollbars: { autoHide: "leave", theme: "os-theme-custom" },
        overflow: { x: "hidden" },
      }}
      class="h-full w-full"
      defer
      events={{
        scroll: (instance) => {
          const { viewport } = instance.elements();
          handleScroll(viewport.scrollTop);
        },
      }}
      bind:this={osComponent}
    >
      <textarea
        class={[
          "w-full min-h-full p-4 border-none outline-none bg-transparent",
          "text-gray-900 dark:text-gray-100 resize-none leading-relaxed",
          "placeholder:text-gray-400 dark:placeholder:text-gray-600",
          "cursor-text select-text block overflow-hidden transition-colors duration-500 ease-in-out",
        ]}
        style="font-family: {fontFamily}; font-size: {fontSize}px;"
        bind:value
        {placeholder}
        {readonly}
        spellcheck="false"
      ></textarea>
    </OverlayScrollbarsComponent>

    {#if isLoading || isFileLoading}
      <div
        class={[
          "absolute inset-0 z-40 flex items-center justify-center",
          "bg-white/50 dark:bg-gray-900/50 backdrop-blur-sm animate-in fade-in duration-200",
        ]}
      >
        <LoaderCircle
          class="animate-spin text-blue-600 dark:text-blue-400"
          size={32}
        />
      </div>
    {/if}
  </div>
</div>

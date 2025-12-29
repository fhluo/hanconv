<script lang="ts">
  import Toolbar from "./Toolbar.svelte";
  import DragOverlay from "./DragOverlay.svelte";
  import { LoaderCircle } from "@lucide/svelte";
  import { OverlayScrollbarsComponent } from "overlayscrollbars-svelte";
  import type { Encoding } from "./EncodingSelector.svelte";
  import type { Font } from "./FontFamilySelector.svelte";

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

  let isDragging = $state(false);
  let isFileLoading = $state(false);
  let copied = $state(false);
  let pasted = $state(false);

  export const openFile = () => {};
  export const saveFile = () => {};
  export const clear = () => {
    value = "";
  };
  export const copy = () => {};
  export const paste = () => {};

  export function scrollTo(scrollTop: number) {
    const instance = osComponent?.osInstance?.();
    if (instance) {
      const { viewport } = instance.elements();
      viewport.scrollTop = scrollTop;
    }
  }

  function handleScroll(scrollTop: number) {
    onScroll?.(scrollTop);
  }
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

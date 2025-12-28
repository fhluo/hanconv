<script lang="ts">
  import { t } from "../i18n.svelte";

  interface Props {
    size?: number;
  }

  let { size = $bindable(16) }: Props = $props();

  const MIN_SIZE = 12;
  const MAX_SIZE = 36;

  function decrease() {
    size = Math.max(MIN_SIZE, size - 1);
  }

  function increase() {
    size = Math.min(MAX_SIZE, size + 1);
  }
</script>

<div class="flex flex-col gap-1">
  <span class="text-[10px] tracking-wider font-medium text-gray-400 px-2"
    >{t("Font Size")}</span
  >
  <div class="flex items-center justify-between px-2 py-1">
    <button
      onclick={decrease}
      class={[
        "p-1 rounded text-lg leading-none text-gray-600 dark:text-gray-400",
        "hover:bg-gray-100 dark:hover:bg-gray-800 disabled:opacity-50",
      ]}
      disabled={size <= MIN_SIZE}>-</button
    >
    <span
      class="text-xs font-mono w-8 text-center text-gray-900 dark:text-gray-100"
      >{size}px</span
    >
    <button
      onclick={increase}
      class={[
        "p-1 rounded text-lg leading-none text-gray-600 dark:text-gray-400",
        "hover:bg-gray-100 dark:hover:bg-gray-800 disabled:opacity-50",
      ]}
      disabled={size >= MAX_SIZE}>+</button
    >
  </div>
</div>

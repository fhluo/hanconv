<script lang="ts">
  import ConversionSelector from "../ui/ConversionSelector.svelte";
  import ThemeToggle from "../ui/ThemeToggle.svelte";
  import LanguageSelector from "../ui/LanguageSelector.svelte";
</script>

<aside
  class={[
    "w-64 flex flex-col border-r border-gray-200 dark:border-gray-800",
    " backdrop-blur-xl z-20 transition-colors duration-300",
  ]}
>
  <div
    class={[
      "relative pt-10 pb-4 px-4 border-b border-gray-100/50 dark:border-gray-800/50",
      "flex items-center justify-between shrink-0 transition-colors duration-300 z-10",
      "bg-white dark:bg-white/2 backdrop-blur-sm",
    ]}
    data-tauri-drag-region
  >
    <div class="flex items-center gap-3">
      <div
        class={[
          "flex items-center justify-center w-8 h-8 rounded-lg shadow-lg shadow-blue-600/30",
          "bg-blue-600 text-white font-bold text-lg",
        ]}
        data-tauri-drag-region
      >
        汉
      </div>
      <h1
        class="text-lg font-mono font-semibold text-gray-900 dark:text-white"
        data-tauri-drag-region
      >
        Hanconv
      </h1>
    </div>

    <div class="flex items-center gap-1">
      <LanguageSelector />
      <ThemeToggle />
    </div>
  </div>

  <div class="flex-1 overflow-hidden bg-white dark:bg-gray-900/50">
    <ConversionSelector />
  </div>
</aside>

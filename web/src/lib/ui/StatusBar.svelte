<script lang="ts">
    import {Link2, Link2Off} from "@lucide/svelte";
    import EncodingSelector from "../ui/EncodingSelector.svelte";
    import FontSettings from "../ui/FontSettings.svelte";

    let charCount = $state(0);
    let scrollSync = $state(false);
</script>

<div
  class={[
    "flex px-4 h-9 gap-2 shrink-0 items-center justify-end ",
    "border-t border-gray-200/60 dark:border-gray-800 bg-white/80 dark:bg-gray-900/80",
    "text-xs text-gray-500 dark:text-gray-400 backdrop-blur-md transition-colors duration-300 ",
  ]}
>
  {#if charCount > 0}
    <span class="mr-2">{charCount} 字符</span>
  {/if}

  <button
    class={[
        "p-1.5 rounded-md hover:bg-gray-100 dark:hover:bg-gray-800 transition-colors outline-none",
        scrollSync
          ? "text-blue-600 dark:text-blue-400"
          : "text-gray-500 dark:text-gray-400",
      ]}
    onclick={() => (scrollSync = !scrollSync)}
  >
    {#if scrollSync}
      <Link2 size={14}/>
    {:else}
      <Link2Off size={14}/>
    {/if}
  </button>

  <FontSettings/>
  <div class="w-px h-4 mx-1 bg-gray-200 dark:bg-gray-800"></div>
  <EncodingSelector/>
</div>

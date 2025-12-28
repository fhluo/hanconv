<script>
  import { FileText } from "@lucide/svelte";
  import { t } from "../lib/i18n.svelte";
</script>

<div
  class={[
    "absolute inset-0 z-50 flex flex-col items-center justify-center gap-4",
    "bg-white/80 dark:bg-gray-900/80 backdrop-blur-md animate-in fade-in duration-200",
  ]}
>
  <div
    class={[
      "absolute inset-4 rounded-2xl pointer-events-none",
      "border-2 border-blue-500/50 dark:border-blue-400/50 border-dashed",
    ]}
  ></div>

  <div
    class={[
      "p-4 rounded-2xl ring-1 shadow-lg shadow-blue-500/10 transform transition-transform scale-110 duration-300",
      "bg-blue-50 dark:bg-blue-900/30 text-blue-600 dark:text-blue-400 ring-blue-500/20 dark:ring-blue-400/20",
    ]}
  >
    <FileText size={32} strokeWidth={1.5} />
  </div>

  <span class="text-sm font-semibold text-gray-900 dark:text-gray-100"
    >{t("Drop to import text")}</span
  >
</div>

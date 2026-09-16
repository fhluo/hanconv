<script lang="ts">
  import Editor from "./ui/Editor.svelte";
  import Sidebar from "./ui/Sidebar.svelte";
  import StatusBar from "./ui/StatusBar.svelte";
  import { t } from "./lib/i18n.svelte";
  import { createConversionState } from "./lib/states/conversion.svelte";
  import type { Font } from "./ui/FontFamilySelector.svelte";
  import type { Encoding } from "./ui/EncodingSelector.svelte";

  const conversionState = createConversionState();
  conversionState.init();

  let currentEncoding: Encoding = $state("Auto");
  let detectedEncoding = $state<Encoding | null>(null);
  let fontFamily: Font = $state("Sans Serif");
  let fontSize = $state(16);
  let scrollSyncEnabled = $state(true);

  interface EditorInstance {
    scrollTo: (scrollTop: number) => void;
  }

  let sourceEditor: EditorInstance | undefined = $state();
  let targetEditor: EditorInstance | undefined = $state();

  function handleScroll(scrollTop: number, source: "source" | "target") {
    if (!scrollSyncEnabled) return;

    if (source === "source" && targetEditor) {
      targetEditor.scrollTo(scrollTop);
    } else if (source === "target" && sourceEditor) {
      sourceEditor.scrollTo(scrollTop);
    }
  }
</script>

<main
  class={[
    "h-screen flex overflow-hidden font-sans select-none",
    "bg-gray dark:bg-black text-gray-900 dark:text-gray-100",
    "transition-colors duration-300",
  ]}
>
  <Sidebar bind:selectedConversion={conversionState.selectedConversion} />

  <div
    class={[
      "flex-1 flex flex-col min-w-0 relative isolate",
      "bg-white dark:bg-gray-950 transition-colors duration-300",
    ]}
  >
    <div class="flex-1 flex flex-col overflow-hidden relative z-0">
      <div class="flex-1 flex flex-col md:flex-row min-h-0 relative">
        <div class="flex-1 flex flex-col min-h-0 relative z-0">
          <Editor
            bind:this={sourceEditor}
            label={t("Source")}
            bind:value={conversionState.inputText}
            placeholder={t("Source Placeholder")}
            encoding={currentEncoding}
            {fontFamily}
            {fontSize}
            onScroll={(y) => handleScroll(y, "source")}
            onEncodingDetected={(enc) => (detectedEncoding = enc)}
          />
        </div>

        <div
          class={[
            "flex-1 flex flex-col min-h-0 relative z-0",
            "bg-gray-50/40 dark:bg-white/2 shadow-sm dark:shadow-sm",
            "transition-colors duration-300",
          ]}
        >
          <Editor
            bind:this={targetEditor}
            label={t("Result")}
            value={conversionState.outputText}
            readonly={true}
            placeholder={t("Result Placeholder")}
            {fontFamily}
            {fontSize}
            isLoading={conversionState.isConverting}
            onScroll={(y) => handleScroll(y, "target")}
          />
        </div>
      </div>

      <StatusBar
        charCount={conversionState.inputText.length}
        bind:scrollSync={scrollSyncEnabled}
        bind:fontFamily
        bind:fontSize
        bind:encoding={currentEncoding}
      />
    </div>
  </div>
</main>

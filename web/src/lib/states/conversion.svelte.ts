import { invoke } from "@tauri-apps/api/core";
import type { Conversion } from "../../ui/ConversionSelector.svelte";

export function createConversionState() {
  let inputText = $state("");
  let outputText = $state("");
  let isConverting = $state(false);
  let selectedConversion: Conversion = $state("s2t");

  let timer: ReturnType<typeof setTimeout> | undefined;
  let latestConversionId = 0;

  return {
    init() {
      $effect(() => {
        const text = inputText;
        const conversion = selectedConversion;

        clearTimeout(timer);

        let delay = 0;
        if (text.length > 50000) {
          delay = 200;
        } else if (text.length > 5000) {
          delay = 50;
        } else {
          delay = 10;
        }

        timer = setTimeout(() => {
          this.convert(text, conversion);
        }, delay);
      });
    },
    get inputText() {
      return inputText;
    },
    set inputText(v) {
      inputText = v;
    },
    get outputText() {
      return outputText;
    },
    get isConverting() {
      return isConverting;
    },
    get selectedConversion() {
      return selectedConversion;
    },
    set selectedConversion(v) {
      selectedConversion = v;
    },
    async convert(text: string, conversion: string) {
      const thisId = ++latestConversionId;

      if (!text) {
        if (thisId === latestConversionId) outputText = "";
        return;
      }

      const loadingTimer = setTimeout(() => {
        if (thisId === latestConversionId) {
          isConverting = true;
        }
      }, 150);

      const start = performance.now();
      try {
        const result = await invoke<string>(conversion, { s: text });
        const end = performance.now();
        console.debug(
          `Conversion of ${text.length} chars took ${(end - start).toFixed(2)}ms`,
        );

        if (thisId === latestConversionId) {
          outputText = result;
        }
      } catch (e) {
        console.error("Conversion failed:", e);
      } finally {
        clearTimeout(loadingTimer);
        if (thisId === latestConversionId) {
          isConverting = false;
        }
      }
    },
  };
}

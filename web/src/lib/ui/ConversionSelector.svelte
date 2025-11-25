<script lang="ts">
  import { ArrowRight, Globe, MapPin } from "@lucide/svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { OverlayScrollbarsComponent } from "overlayscrollbars-svelte";
  import { cubicInOut } from "svelte/easing";
  import { crossfade } from "svelte/transition";

  type Conversion =
    | "s2t"
    | "t2s"
    | "s2tw"
    | "tw2s"
    | "t2tw"
    | "tw2t"
    | "s2hk"
    | "hk2s"
    | "t2hk"
    | "hk2t"
    | "t2jp"
    | "jp2t";

  type Category = "常用" | "台湾" | "香港" | "日本";

  type Variant =
    | "简体中文"
    | "繁体中文"
    | "繁体中文（台湾）"
    | "繁体中文（香港）"
    | "繁体字"
    | "日文新字体";

  async function convert(conversion: Conversion, text: string) {
    return await invoke(conversion, { s: text });
  }

  interface Group {
    category: Category;
    icon: typeof Globe;
    items: {
      conversion: Conversion;
      source: Variant;
      target: Variant;
    }[];
  }

  const groups: Group[] = [
    {
      category: "常用",
      icon: Globe,
      items: [
        {
          conversion: "s2t",
          source: "简体中文",
          target: "繁体中文",
        },
        {
          conversion: "t2s",
          source: "繁体中文",
          target: "简体中文",
        },
      ],
    },
    {
      category: "台湾",
      icon: MapPin,
      items: [
        {
          conversion: "s2tw",
          source: "简体中文",
          target: "繁体中文（台湾）",
        },
        {
          conversion: "tw2s",
          source: "繁体中文（台湾）",
          target: "简体中文",
        },
        {
          conversion: "t2tw",
          source: "繁体中文",
          target: "繁体中文（台湾）",
        },
        {
          conversion: "tw2t",
          source: "繁体中文（台湾）",
          target: "繁体中文",
        },
      ],
    },
    {
      category: "香港",
      icon: MapPin,
      items: [
        {
          conversion: "s2hk",
          source: "简体中文",
          target: "繁体中文（香港）",
        },
        {
          conversion: "hk2s",
          source: "繁体中文（香港）",
          target: "简体中文",
        },
        {
          conversion: "t2hk",
          source: "繁体中文",
          target: "繁体中文（香港）",
        },
        {
          conversion: "hk2t",
          source: "繁体中文（香港）",
          target: "繁体中文",
        },
      ],
    },
    {
      category: "日本",
      icon: MapPin,
      items: [
        {
          conversion: "t2jp",
          source: "繁体字",
          target: "日文新字体",
        },
        {
          conversion: "jp2t",
          source: "日文新字体",
          target: "繁体字",
        },
      ],
    },
  ];

  type ConversionItem = Group["items"][number];

  let selected = $state<Conversion>("s2t");

  const [send, receive] = crossfade({
    duration: 250,
    easing: cubicInOut,
  });

  const items = groups
    .flatMap((category) => category.items)
    .map((item) => item.conversion);
  let itemElements: Record<string, HTMLElement> = {};

  $effect(() => {
    itemElements[selected]?.scrollIntoView({
      block: "nearest",
      behavior: "smooth",
    });
  });
</script>

<svelte:window
  onkeydown={(e: KeyboardEvent) => {
    const target = e.target as HTMLElement;
    if (target.tagName === "TEXTAREA" || target.isContentEditable) {
      return;
    }

    if (e.key === "ArrowUp" || e.key === "ArrowDown") {
      e.preventDefault();
      const i = items.findIndex((c) => c === selected);
      if (i === -1) {
        return;
      }

      if (e.key === "ArrowUp") {
        selected = items[(i - 1 + items.length) % items.length];
      } else {
        selected = items[(i + 1) % items.length];
      }
    }
  }}
/>

{#snippet conversionItem(item: ConversionItem)}
  <button
    onclick={() => {
      selected = item.conversion;
    }}
    bind:this={itemElements[item.conversion]}
    class={[
      "flex flex-col items-center gap-3 w-full text-left px-3 py-2.5 rounded-xl",
      "transition-all duration-200 active:scale-[0.98] relative group",
      selected === item.conversion
        ? "text-blue-900 dark:text-blue-100"
        : "text-gray-600 dark:text-gray-400 hover:text-gray-900 dark:hover:text-gray-200 hover:bg-gray-100 dark:hover:bg-white/5 ",
    ]}
  >
    {#if selected === item.conversion}
      <div
        in:receive={{ key: "active" }}
        out:send={{ key: "active" }}
        class={[
          "absolute inset-0 rounded-xl border shadow-sm",
          "bg-linear-to-b from-white to-blue-50/50 border-blue-100",
          "dark:from-blue-500/20 dark:to-blue-600/10 dark:border-blue-400/20 dark:shadow-none",
        ]}
      ></div>
    {/if}
    <div class="flex-col gap-0.5 relative z-10 w-full">
      <div class="flex items-center justify-between">
        <span
          class="text-[10px] font-semibold opacity-60 pl-0.5 transition-colors"
          >{item.source}</span
        >
      </div>
      <div class="flex items-center gap-2">
        <span
          class={[
            "w-5 h-5 rounded-full flex items-center justify-center shrink-0 transition-all duration-300",
            selected === item.conversion
              ? "bg-blue-100 text-blue-600 dark:bg-blue-500/30 dark:text-blue-200"
              : "bg-gray-100 text-gray-400 dark:bg-gray-800 dark:text-gray-600 group-hover:bg-white dark:group-hover:bg-gray-700 group-hover:shadow-sm",
          ]}
        >
          <ArrowRight size={12} strokeWidth={2.5} />
        </span>
        <span class="text-sm font-semibold transition-colors"
          >{item.target}</span
        >
      </div>
    </div>
  </button>
{/snippet}

{#snippet groupTitle(category: Category, Icon: typeof Globe)}
  <div
    class={[
      "px-2 py-1 flex items-center gap-2 opacity-80",
      "text-xs font-bold text-gray-400 dark:text-gray-500 tracking-wider",
    ]}
  >
    <Icon size={12} />
    <span>{category}</span>
  </div>
{/snippet}

{#snippet groupItems(items: ConversionItem[])}
  <div class="flex flex-col gap-1.5">
    {#each items as item}
      {@render conversionItem(item)}
    {/each}
  </div>
{/snippet}

<OverlayScrollbarsComponent
  element="div"
  options={{
    scrollbars: { autoHide: "leave", theme: "os-theme-custom" },
    overflow: { x: "hidden" },
  }}
  class="h-full"
  defer
>
  <div class="flex flex-col p-3 gap-6">
    {#each groups as group}
      <div class="flex flex-col gap-2">
        {@render groupTitle(group.category, group.icon)}
        {@render groupItems(group.items)}
      </div>
    {/each}
  </div>
</OverlayScrollbarsComponent>

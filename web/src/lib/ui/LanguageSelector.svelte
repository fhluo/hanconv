<script lang="ts">
    import { Check, Languages } from "@lucide/svelte";

    let isOpen = $state(false);
    let tag = $state("zh-Hans");

    const languages: { tag: string, name: string }[] = [
        {
            tag: "zh-Hans",
            name: "简体中文"
        },
        {
            tag: "zh-Hant",
            name: "繁体中文"
        },
        {
            tag: "en",
            name: "English"
        }
    ];
</script>

<div class="relative">
    <button onclick={()=> {isOpen = !isOpen}} aria-label="Change language"
            class="p-2 rounded-md text-gray-500 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-800 transition-colors cursor-pointer">
        <Languages size={18}/>
    </button>

    {#if isOpen}
        <div class="fixed inset-0 z-40" onclick={() => isOpen = false} role="presentation"></div>

        <div class={[
            "absolute top-full right-0 min-w-[120px] z-50 p-1 mt-2 flex flex-col rounded-lg shadow-sm",
            "border border-gray-200 dark:border-gray-800 bg-white dark:bg-gray-900",
            "animate-in fade-in zoom-in-95 slide-in-from-top-2 duration-200 ease-out origin-top-right"
        ]}>
            {#each languages as lang}
                <button onclick={()=> { tag = lang.tag; isOpen = false; }} class={[
                "flex items-center px-2 py-1.5 w-full rounded select-none outline-none transition-colors",
                tag === lang.tag ? 'text-gray-900 dark:text-gray-100 bg-gray-100 dark:bg-gray-800' :
                 'text-gray-600 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-800']}>
                <span class="w-4 mr-1.5 flex items-center justify-center shrink-0">
                    {#if tag === lang.tag}
                        <Check size={12} strokeWidth={2.5} class="text-blue-600 dark:text-blue-400"/>
                    {/if}
                </span>
                    <span class="text-xs font-medium shrink-0">{lang.name}</span>
                </button>
            {/each}
        </div>
    {/if}
</div>

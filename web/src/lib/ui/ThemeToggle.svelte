<script lang="ts">
    import { Sun, Moon } from "@lucide/svelte";
    import { getCurrentWindow, type Theme } from "@tauri-apps/api/window";
    import { load, Store } from "@tauri-apps/plugin-store";
    import { onMount } from "svelte";

    let theme = $state<Theme>("light");
    let settings = $state<Store | null>(null);

    onMount(async () => {
        try {
            settings = await load("settings.json");
            theme = await settings?.get<Theme>("theme") ?? await getCurrentWindow().theme() ?? "light";
        } catch (err) {
            console.error(err);
        }
    })

    $effect(() => {
        if (theme === "dark") {
            document.documentElement.classList.add("dark");
            settings?.set("theme", theme).catch(err => {
                console.error(err);
            });
        } else {
            document.documentElement.classList.remove("dark");
            settings?.set("theme", theme).catch(err => {
                console.error(err);
            });
        }
    })
</script>

<button onclick={()=>{theme = theme === "light" ? "dark" : "light"}} aria-label="Toggle theme"
        class={[
            "p-2 rounded-md transition-colors cursor-pointer",
            "text-gray-500 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-800"
        ]}>
    {#if theme === "light" }
        <Moon size={18}/>
    {:else}
        <Sun size={18}/>
    {/if}
</button>

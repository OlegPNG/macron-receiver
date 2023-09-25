<script lang="ts">
    import ConfigItem from "./lib/ConfigItem.svelte";
import MacronNavBar from "./lib/MacronNavBar.svelte";
import MacronSidebar from "./lib/MacronSidebar.svelte";
    import ScreenSelector from "./lib/ScreenSelector.svelte";


</script>

<main>
    <div class="flex flex-col m-8">
        <ScreenSelector />
        <ConfigItem />
    </div>
</main>

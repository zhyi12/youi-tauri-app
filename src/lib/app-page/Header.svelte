<script>

    import PageNav from "./PageNav.svelte";
    import {configStore} from "../app-stores/base/configStore";
    export let navPaths = [];

</script>

<div class="shadow items-center h-12 flex shrink-0 pl-2">
    <PageNav {navPaths}/>
    <div class="flex-1"></div>
    <div>
        {$configStore.areaId||''}
    </div>
</div>
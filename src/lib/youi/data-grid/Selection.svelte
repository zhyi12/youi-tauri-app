<script lang="ts">

    import Box from "./Box.svelte";

    export let selectionBoxes = [];

</script>

{#if selectionBoxes.length}
    <div class="selection-boxes">
        <div>
            {#each selectionBoxes as box}
                <Box {...box}></Box>
            {/each}
            <slot></slot>
        </div>
    </div>
{/if}
<style>
    .selection-boxes{
        pointer-events: none;
        position: absolute;
        overflow:hidden;
        top:0px;
        left:0px;
        right:0px;
        bottom:0px;
        height: auto;
    }
</style>
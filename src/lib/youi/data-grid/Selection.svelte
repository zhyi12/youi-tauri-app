<script lang="ts">

    import Box from "./Box.svelte";
    import {createEventDispatcher} from "svelte";

    const dispatch = createEventDispatcher();

    export let selectionBoxes = [];

    let selectionHelper = undefined;

    $: if(selectionHelper && selectionBoxes.length){
        selectionHelper.focus();
        dispatch('focus',{
            selectionBoxes
        });
    }

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
<input bind:this={selectionHelper}
       on:keydown
       class="-z-1 absolute w-0 h-0"
       style:left={`${selectionBoxes.length?selectionBoxes[0].x:-1}px`}
       style:top={`${selectionBoxes[0].y}px`}/>

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
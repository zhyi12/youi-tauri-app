<script lang="ts">

    import Button from "../../button/Button.svelte";
    import type {ButtonInfo} from "../../button/Button.svelte";
    import {isFunction} from "@tanstack/table-core";

    export let buttons:ButtonInfo[] = [];

    export let record:Record<string, any> = {};

    /**
     *
     * @param button
     */
    const doAction = (button) => {
        if(isFunction(button.action)){
            button.action(record);
        }
    }

</script>

<div>
    {#each buttons as button}
        <Button title={`${button.title||button.text}${record.caption||record.text}`} class="btn-sm btn-ghost" on:click={()=>doAction(button)}>
            {button.text}
        </Button>
    {/each}
</div>
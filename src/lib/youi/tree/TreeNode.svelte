<script lang="ts">

    import TreeText from "./TreeText.svelte";
    import {getContext} from "svelte";

    const {treeConfig,hoverNodeId,selectNode,selectedNodeIds} = getContext('Tree');

    export let id = '';
    export let text = '';
    export let href = 'javascript:void(0)';
    export let level = 0;
    export let icon:string = undefined;
    export let datas: Record<string,any> = undefined;

    $:selected = $selectedNodeIds.includes(id);
    $:hovered = $hoverNodeId === id;

</script>

<li {id}>
    <TreeText {level} {selected}
              on:mouseenter={()=>{ $hoverNodeId = id }}
              on:mouseleave={()=>{ $hoverNodeId = undefined }}
              on:click={selectNode({id})}>
        {#if $treeConfig.nodeRender}
            {@const result = $treeConfig.nodeRender({id,text,hovered,selected})}
            {#if result.component && result.props}
                <svelte:component this={result.component} {...result.props} />
            {:else if typeof result === 'string' || typeof result === 'number'}
                {result}
            {/if}
        {:else}
            {text}
        {/if}
    </TreeText>
</li>
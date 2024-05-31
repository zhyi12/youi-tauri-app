<script lang="ts">

    import TreeText from "./TreeText.svelte";
    import {getContext} from "svelte";
    import Icon from "../icon/Icon.svelte";

    const {treeConfig,hoverNodeId,selectNode,dblClick,selectedNodeIds,icons} = getContext('Tree');

    export let id = '';
    export let text = '';
    export let href = 'javascript:void(0)';
    export let level = 0;
    export let icon:string = undefined;
    export let group:string = undefined;
    export let datas: Record<string,any> = undefined;

    $:selected = $selectedNodeIds.includes(id);
    $:hovered = $hoverNodeId === id;

</script>

<li {id}>
    <TreeText {level} {selected}
              on:mouseenter={()=>{ $hoverNodeId = id }}
              on:mouseleave={()=>{ $hoverNodeId = undefined }}
              on:dblclick={()=>dblClick({id})}
              on:click={()=>selectNode({id})}>
        {#if icon && icons}
            {@const iconData = icons(icon)}
            <div><Icon class="w-4 h-4 mr-0.5 -mt-0.5" data={iconData}/></div>
        {/if}
        {#if $treeConfig.nodeRender}
            {@const result = $treeConfig.nodeRender({id,text,group,hovered,selected})}
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
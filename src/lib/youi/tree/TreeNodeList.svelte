<script lang="ts">
    import {getContext} from 'svelte';
    import TreeNode from "./TreeNode.svelte";

    import type {TreeItem} from "./Tree.d";
    import TreeText from "./TreeText.svelte";
    import Icon from "../icon/Icon.svelte";

    const {selectedNodeIds,expandedNodeIds,
        hoverNodeId,
        toggleNode,selectNode,treeConfig,icons} = getContext('Tree');

    export let root = false;

    export let id = undefined;
    export let text = undefined;
    export let level = 1;
    export let icon:string = undefined;
    export let group:string = undefined;

    export let children:TreeItem[] = [];

    export let datas:Record<string, any> = {};

    /**
     * 节点排序处理
     */
    $: sortedChildren = $treeConfig.sorted?children.sort((a,b)=>{
        const aOrder = a.datas?(a.datas.num||0):0;
        const bOrder = b.datas?(b.datas.num||0):0;
        return aOrder>bOrder?1:-1;
    }):children;

    $:expanded = $expandedNodeIds.includes(id);
    $:expandable = true;

    $:hovered = $hoverNodeId === id;
    $:selected = $selectedNodeIds.includes(id);

    /**
     *
     */
    const handle_dblclick = () => {
        toggleNode({id})
    }

</script>

{#if root}
    {#each sortedChildren as child (child.id)}
        {#if child.children && child.children.length}
            <svelte:self {...child} level={1}/>
        {:else}
            <TreeNode {...{
                id:child.id,
                text:child.text,
                icon:child.icon,
                group:child.group,
                href:child.href,
                level:1,
                datas:child.datas
            }}/>
        {/if}
    {/each}
{:else}
    <li {id}>
        <TreeText {selected} {level}
                  on:mouseenter={()=>{ $hoverNodeId = id }}
                  on:mouseleave={()=>{ $hoverNodeId = undefined }}
                  on:click={()=>selectNode({id})}
                  on:dblclick={handle_dblclick}>
            {#if expandable}
                <span class="tree-node-toggle" on:click={(e)=>{
                    toggleNode({id});
                    e.preventDefault();
                    e.stopPropagation();
                }}>
                    <svg aria-hidden="true" focusable="false" role="img" viewBox="0 0 16 16" width="16" height="16" fill="currentColor" style="display: inline-block; user-select: none; vertical-align: text-bottom; overflow: visible;">
                        {#if !expanded}
                            <path d="M6.22 3.22a.75.75 0 0 1 1.06 0l4.25 4.25a.75.75 0 0 1 0 1.06l-4.25 4.25a.751.751 0 0 1-1.042-.018.751.751 0 0 1-.018-1.042L9.94 8 6.22 4.28a.75.75 0 0 1 0-1.06Z"></path>
                        {:else }
                            <path d="M12.78 5.22a.749.749 0 0 1 0 1.06l-4.25 4.25a.749.749 0 0 1-1.06 0L3.22 6.28a.749.749 0 1 1 1.06-1.06L8 8.939l3.72-3.719a.749.749 0 0 1 1.06 0Z"></path>
                        {/if}

                    </svg>
                </span>
            {/if}

            {#if icon && icons}
                {@const iconData = icons(icon,expanded)}
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
                <span>{text}</span>
             {/if}
        </TreeText>
        {#if expanded && children.length}
            <ul>
                {#each sortedChildren as child (child.id)}
                    {#if child.children && child.children.length}
                        <svelte:self {...child} level={level+1}/>
                    {:else}
                        <TreeNode {...{
                            id:child.id,
                            text:child.text,
                            icon:child.icon,
                            group:child.group,
                            href:child.href,
                            level:level+1,
                            datas:child.datas
                        }}/>
                    {/if}
                {/each}
            </ul>
        {/if}
    </li>
{/if}

<style>
    .tree-node-toggle{
        margin-left:-16px;
    }
</style>
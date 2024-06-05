<script lang="ts">

    import {onMount} from 'svelte';
    import TreeText from "./TreeText.svelte";
    import {getContext} from "svelte";
    import {Icon,refreshIcon} from "../index";

    const {treeConfig,hoverNodeId,selectNode,dblClick,selectedNodeIds,loadChildren,icons} = getContext('Tree');

    export let id = '';
    export let text = '';
    export let href = 'javascript:void(0)';
    export let level = 0;
    export let icon:string = undefined;
    export let group:string = undefined;
    export let datas: Record<string,any> = undefined;

    $:selected = $selectedNodeIds.includes(id);
    $:hovered = $hoverNodeId === id;
    $:loading = datas && datas.loading;

    onMount(()=>{
        if(loading && datas.id){
            loadChildren(datas);
        }
    });
</script>

<li {id}>

        <TreeText {level} {selected}
                  on:mouseenter={()=>{ if(!loading) $hoverNodeId = id }}
                  on:mouseleave={()=>{ if(!loading) $hoverNodeId = undefined }}
                  on:dblclick={()=>!loading && dblClick({id})}
                  on:click={()=>!loading && selectNode({id})}>

            {#if loading}
                <div><Icon class="w-4 h-4 mr-0.5 -mt-0.5" spin={true} data={refreshIcon}/></div><span>加载中...</span>
            {:else}
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
            {/if}
        </TreeText>
</li>
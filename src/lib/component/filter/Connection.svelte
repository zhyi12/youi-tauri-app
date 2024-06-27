<script lang="ts">
    import {getContext} from "svelte";
    import type {FilterNode} from "./Filter";
    import Condition from "./Condition.svelte";
    import {Icon,plusIcon} from "../../youi";
    import {FILTER_CONTEXT} from "./helper";

    const {collapseConnectionIds,addCondition,toggleConnect} = getContext(FILTER_CONTEXT);

    export let items:FilterNode = [];

    export let root = false;

    export let id = '';

    export let connect = 'and';

    let hovering = false;

    let bracketsHovering = false;

    const toggleCollapse = () => {
        if(collapsed){
            $collapseConnectionIds = $collapseConnectionIds.filter(c=>c!==id);
        }else{
            $collapseConnectionIds.push(id);
            $collapseConnectionIds = $collapseConnectionIds;
        }
    }

    $:count = items?items.length:0;
    $:collapsed = $collapseConnectionIds.includes(id);

</script>


{#if count}
    {#if !root}
        <div on:mouseenter|stopPropagation={_e=>bracketsHovering=true}
            on:mouseleave|stopPropagation={_e=>bracketsHovering=false}
            on:click={()=>toggleCollapse()}
            class:bg-blue-200={bracketsHovering}
            class="float-left p-1 cursor-pointer">(</div>
    {/if}
    {#if collapsed}
        <div class="float-left p-1 ">...</div>
    {:else}
        {#each items as item,index}
            {#if 'Item' === item.nodeType}
                <Condition connectId={id} id={item.id}
                           dataType={item.dataType||'text'}
                           property={item.name||item.property} operator={item.operator} value={item.value}/>
            {:else}
                <svelte:self id={item.id} connect={item.nodeType.toLowerCase()} items={item.items}/>
            {/if}
            {#if index<count-1}
                <div class="float-left p-1 bg-blue-50"
                    on:mouseenter|stopPropagation={_e=>hovering=true}
                    on:mouseleave|stopPropagation={_e=>hovering=false}
                >
                    <div class="dropdown dropdown-hover">
                        <div tabindex="0" role="button" class="cursor-pointer rounded text-green-700 w-8 text-center mx-0.5 font-bold">
                            {connect}
                        </div>
                        {#if hovering}
                            <ul class="menu fixed dropdown-content bg-base-100 rounded border w-40  z-[2]">
                                <li on:click={()=>toggleConnect(id)}>
                                    <a>切换为{connect === 'and'?'或':'且'}</a>
                                </li>
                                <li on:click={()=>addCondition(id,'and',item.id)}>
                                    <a>插入且条件</a>
                                </li>
                                <li on:click={()=>addCondition(id,'or',item.id)}>
                                    <a>插入或条件</a>
                                </li>
                            </ul>
                        {/if}
                    </div>

                </div>
            {/if}
        {/each}
    {/if}

    {#if !root}
        <div on:mouseenter|stopPropagation={_e=>bracketsHovering=true}
            on:mouseleave|stopPropagation={_e=>bracketsHovering=false}
            on:click={()=>toggleCollapse()}
              class:bg-blue-200={bracketsHovering}  class="float-left p-1 cursor-pointer">)</div>
    {/if}
{/if}

<script>

    import {Input,Icon,moreVIcon} from "../../youi/index";
    import {getContext} from "svelte";
    import {FILTER_CONTEXT,FILTER_OPS,FILTER_OP_MAP} from "./helper";
    import Tree from "../../youi/tree/Tree.svelte";

    const {activeId,selectItems,valueItems,icons,removeCondition,addCondition,
        updateProperty,updateCondition,loadValueItems} = getContext(FILTER_CONTEXT);

    /**
     * 唯一标志
     */
    export let id;
    /**
     * 属性
     */
    export let property;

    /**
     * 操作符
     */
    export let operator;
    /**
     * 数据类型
     */
    export let dataType = 'text';

    export let value = [];
    /**
     * 上级连接ID
     */
    export let connectId;

    let hovering = false;

    let editing = false;

    $:column = $selectItems.find(item=>item.name === property);
    $:active = $activeId === id;

    /**
     *
     */
    const handle_input_focus = (e) => {
        $valueItems = [];
        loadValueItems({id, property,dataType});
    }

    /**
     *
     */
    const handle_input_change = (e) => {
        //参数变量处理
        let values = [e.target.value];
        if(operator === 'in'){
            values = e.target.value.split(',');
        }
        if(dataType === 'number'||dataType === 'f64'){
            values = values.map(v=>parseFloat(v));
        }else if(dataType === 'i64' || dataType === 'integer'){
            values = values.map(v=>parseInt(v));
        }
        updateCondition(id,'value',values,value);
        editing = false;
    }

</script>

<div class="float-left py-1 flex bg-gray-100 rounded hover:bg-gray-200"
    on:mouseenter|stopPropagation={()=> hovering = true}
    on:mouseleave|stopPropagation={()=> hovering = false}
>
    <div class="dropdown">
        <div tabindex="0" role="button" class="text-green-700 min-w-10 min-h-4">
            <Icon data={icons(dataType)}/>
            {column?column.text:property||''}
        </div>
        {#if hovering}
            <ul class="menu fixed dropdown-content w-80 bg-base-100 rounded border overflow-x-hidden z-[2]">
                {#if $selectItems}
                    {#each $selectItems as item}
                        <li title={item.text} on:mousedown={_=>updateProperty(id,item.dataType,item.name,property)}>
                            <a class:active={item.name === property}>
                                <Icon data={icons(item.dataType)}/>
                                {item.text}
                            </a>
                        </li>
                    {/each}
                {/if}
            </ul>
        {/if}
    </div>

    <div class="px-2 bg-blue-50 rounded">
        <div class="dropdown dropdown-end">
            <div tabindex="0" role="button" class="text-rose-600">{FILTER_OP_MAP.get(operator)}</div>
            {#if hovering}
                <ul class="menu dropdown-content w-40 bg-base-100 rounded border z-[2]">
                    {#each FILTER_OPS as op}
                        <li on:mousedown={_=>updateCondition(id,'operator',op.name,operator)}>
                            <a class:active={op.name === operator}>
                                {op.symbol || op.text}
                            </a>
                        </li>
                    {/each}
                </ul>
            {/if}
        </div>
    </div>

    {#if !(operator === 'is_null' || operator === 'is_not_null')}
        <div class="h-4 min-w-10 max-w-40"
             on:mouseenter={_=>{
                 editing = true;
                 $activeId = id;
             }}
             on:mouseleave={_=>{
                 if(!active){
                     editing = false
                 }
             }}
        >
            {#if editing}
                <div class="dropdown dropdown-end">
                    <Input tabindex="0" role="button"
                           on:blur={()=>editing = false}
                           on:change={handle_input_change}
                           on:focus={handle_input_focus} class="w-40" value={value}/>
                    {#if $valueItems.length}
                        <div class="menu dropdown-content w-96 bg-base-100 rounded border z-[2]">
                            <Tree children={$valueItems}/>
                        </div>
                    {/if}
                </div>
            {:else}
                {value}
            {/if}
        </div>
    {/if}

    <div class="dropdown dropdown-hover dropdown-end">
        <div tabindex="0" role="button" class="cursor-pointer w-4 text-center caret hover:text-sky-400">
            <Icon scale={1} data={moreVIcon}/>
        </div>
        {#if hovering}
            <ul class="menu dropdown-content bg-base-100 rounded border w-40  z-[2]">
                <li>
                    <a on:click={()=>removeCondition(connectId,id)}>删除条件</a>
                </li>
                <li on:click={()=>addCondition(connectId,'and',id)}>
                    <a>插入且条件</a>
                </li>
                <li on:click={()=>addCondition(connectId,'or',id)}>
                    <a>插入或条件</a>
                </li>
            </ul>
        {/if}
    </div>

</div>

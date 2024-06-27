<script>
    import {Icon,caretLeftIcon,caretRightIcon} from "../../youi";
    import CircleAddButton from "./CircleAddButton.svelte";
    import ColumnSelect from "./ColumnSelect.svelte";
    import {AGG_ITEMS} from "./helper";
    import {removeIcon} from "../../youi/icons";
    import {createEventDispatcher} from "svelte";

    const dispatch = createEventDispatcher();

    /**
     * 分组列名
     */
    export let groupNames = [];

    /**
     * 指标
     */
    export let measureItems = [];

    /**
     * 可选列
     */
    export let columns = [];

    export let icons = (_name)=>undefined;

    let measureOffsetLeft = 0;

    const addNewGroup = () => {
        groupNames = groupNames||[];
        groupNames.push('');
        groupNames = groupNames;
        dispatch('change',{type:'addGroup'});
    }

    const addNewMeasureItem = () => {
        measureItems = measureItems||[];
        measureItems.push({
            name:'',
            text:'',
            aggregate:'sum'
        });
        dispatch('change',{type:'addMeasureItem'});
        measureItems = measureItems;
    }

    const selectGroup = (index,{detail}) => {
        groupNames[index] = detail.name;
        dispatch('change',{type:'group'});
        groupNames = groupNames;
    }

    const selectMeasureItem = (measureItem,{detail}) => {
        measureItem.name = detail.name;
        measureItem.text = detail.text;
        measureItem.dataType = detail.dataType;
        if(detail.dataType === 'text' || detail.dataType === 'str' ){
            measureItem.aggregate = 'count';
        }
        dispatch('change',{type:'measureItem'});
        measureItems = measureItems;
    }

    const changeAggregate = (measureItem,aggregate) => {
        measureItem.aggregate = aggregate;
        dispatch('change',{type:'aggregate'});
        measureItems = measureItems;
    }

    const removeGroup = (name,index) => {
        groupNames.splice(index,1);
        dispatch('change',{type:'removeGroup'});
        groupNames = groupNames;
    }

    const removeMeasureItem = (item,index)=>{
        measureItems.splice(index,1);
        dispatch('change',{type:'removeMeasureItem'});
        measureItems = measureItems;
    }

    const handle_measure_scroll = (e) => {
        measureOffsetLeft = e.target.scrollLeft;
    }

    $:textMap = columns.reduce((m,c)=>m.set(c.name,c.text),new Map());
</script>


<div class="flex-1 flex">
    <div class="dimension-label">
        分组<CircleAddButton title="新分组" on:click={()=>addNewGroup()}/>
    </div>
    <div class="dimension-content">
        {#if groupNames}
            {#each groupNames as name,index}
                <div class="dimension-item">
                    <ColumnSelect {icons} on:select={(e)=>selectGroup(index,e)} {columns} property={name}
                                  text={textMap.get(name)||name}/>
                    <div class="flex items-center w-4 text-gray-400 hover:text-red-500" on:click={()=>removeGroup(name,index)}>
                        <Icon data={removeIcon}/>
                    </div>
                </div>
            {/each}
        {/if}
    </div>
</div>
<div class="flex-1 border-t flex">
    <div class="dimension-label">
        指标
        <CircleAddButton title="新指标" on:click={()=>addNewMeasureItem()}/>
    </div>
<!--    <div class="flex items-center justify-center w-3 hover:bg-gray-50 cursor-pointer text-gray-400">-->
<!--        <Icon data={caretLeftIcon}/>-->
<!--    </div>-->
    <div class="dimension-content w-0 overflow-x-auto" on:scroll={handle_measure_scroll}>
        <div class="flex w-[3000px]">
            {#if measureItems}
                {#each measureItems as item,index}
                    <div class="dimension-item">
                        <div class="dropdown">
                            <div tabindex="0" role="button"  class="text-green-700 h-8 p-1 flex items-center">
                                <Icon data={icons(item.aggregate)}/>
                            </div>
                            <ul style={`transform: translateX(${measureOffsetLeft*-1}px);`} class="menu fixed dropdown-content w-32 bg-base-100 rounded border overflow-x-hidden z-[2]">
                                {#each AGG_ITEMS as aggItem}
                                    <li on:mousedown={()=>changeAggregate(item,aggItem.name)}>
                                        <a>
                                            <Icon data={icons(aggItem.name)}/>
                                            {aggItem.text}
                                        </a>
                                    </li>
                                {/each}
                            </ul>
                        </div>
                        <ColumnSelect offset={measureOffsetLeft}  {icons} on:select={(e)=>selectMeasureItem(item,e)} {columns} property={item.name} text={item.text}/>
                        <div class="flex items-center w-4 text-gray-400 hover:text-red-500" on:click={()=>removeMeasureItem(item,index)}>
                            <Icon data={removeIcon}/>
                        </div>
                    </div>
                {/each}
            {/if}
        </div>
    </div>
<!--    <div class="flex items-center justify-center w-3 hover:bg-gray-50 cursor-pointer text-gray-400">-->
<!--        <Icon data={caretRightIcon}/>-->
<!--    </div>-->
</div>

<style lang="scss">
  .dimension-label{
    @apply flex w-16 h-16 bg-gray-50 items-center justify-center font-bold;
  }

  .dimension-content{
    @apply flex-1 flex items-center p-0.5;

    .dimension-item{
      @apply min-w-16 rounded-md cursor-pointer bg-blue-50 mr-1 flex;

      .aggregate{
        @apply flex items-center justify-center w-4;
      }
    }
  }
</style>
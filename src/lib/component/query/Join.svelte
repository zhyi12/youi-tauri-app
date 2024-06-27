<script>

    import {JOINS} from "./helper";
    import {Button,Icon,FieldInput, Select} from "../../youi/index";
    import CircleAddButton from "./CircleAddButton.svelte";
    import DataSourceSelector from "./DataSourceSelector.svelte";
    import {intersection} from 'lodash';

    export let icons = (_name)=>undefined;

    export let how = 'left';

    export let reader = '';

    export let uri = '';

    export let tableTitle = '';

    export let leftColumns = [];

    export let columns = [];

    export let joinColumns = [];

    export let selectedColumnNames = [];

    /**
     *
     */
    let selectorDialog;
    
    const addJoinColumn = () => {
        joinColumns.push({
            left:'',
            right:'',
            name:''
        });

        joinColumns = joinColumns;
    }

    const closeDsSelector = () => {
        if(selectorDialog){
            selectorDialog.close();
        }
    }

    /**
     * 选择数据源
     */
    const selectDs = () => {
        closeDsSelector();
        let sameNames = intersection(leftColumns.map(c=>c.name),columns.map(c=>c.name));
        joinColumns = sameNames.map(name=>({name,left:name,right:name}));
    }

    $:leftItems = leftColumns.map(c=>({...c,icon:c.dataType}));
    $:rightItems = columns.map(c=>({...c,icon:c.dataType}));

</script>

<div class="query-join-container">
    <div class="flex h-6 items-center p-1 bg-gray-50">
        连接数据
    </div>
    <div class="flex h-16">
        <div class="flex-1 items-center p-2 overflow-hidden">
            {tableTitle||uri}
        </div>
        <div class="flex items-center p-1 bg-gray-50">
            <Button title="选择连接数据" class="btn-xs btn-ghost" on:click={(e)=>{
            if(selectorDialog){
                selectorDialog.showModal();
            }
            e.stopPropagation();
        }}>
                ...
            </Button>
        </div>
    </div>
    <div class="flex h-6 items-center p-1 bg-blue-50">
        连接方式
    </div>
    <div class="flex flex-1 w-full">
        {#each JOINS as join}
            <div on:click={()=>how = join.name}
                    class:text-green-700={how === join.name}
                    class:bg-blue-50={how === join.name}
                 class="join-how">
                <div><Icon scale={3} data={icons(join.name)}/></div>
                <div>{join.text}</div>
            </div>
        {/each}
    </div>
</div>

<div class="p-1">
    <CircleAddButton title="添加合并条件" on:click={()=>addJoinColumn()}/>
</div>
<div class="flex-1 p-0 flex flex-col">
    <div class="flex-1 h-full overflow-auto relative pt-1">
        <div class="flex h-6 bg-gray-50">
            <div class="join-column-header bg-blue-50">
                合并结果
            </div>
            <div class="join-column-header" class:bg-green-50={how !== 'right'} class:bg-yellow-50={how === 'right'}>
                当前表列
            </div>
            <div class="join-column-header" class:bg-green-50={how === 'right'} class:bg-yellow-50={how !== 'right'}>
                连接表列
            </div>
        </div>
        {#each joinColumns as joinColumn}
            <div class="flex">
                <div class="flex-1 p-1">
                    <FieldInput bind:value={joinColumn.name}/>
                </div>
                <div class="flex-1 p-1">
                    <Select bind:value={joinColumn.left} valueProp={"name"} {icons} items={leftItems}/>
                </div>
                <div class="flex-1 p-1">
                    <Select bind:value={joinColumn.right} valueProp={"name"} {icons} items={rightItems}/>
                </div>
            </div>
        {/each}
    </div>
</div>
<!-- -->
<dialog class="modal" bind:this={selectorDialog}>
    <div class="modal-box w-11/12 max-w-5xl h-full flex flex-col overflow-hidden">
        <div class="flex flex-col flex-1 overflow-hidden">
            <DataSourceSelector bind:uri bind:columns bind:selectedColumnNames bind:reader/>
        </div>
        <div class="modal-action">
            <Button disabled={selectedColumnNames.length === 0} class="btn-sm btn-primary" title="选择" on:click={()=>selectDs()}>
                确定
            </Button>
            <Button class="btn-sm" title="返回" on:click={()=>closeDsSelector()}>
                返回
            </Button>
        </div>
    </div>
</dialog>

<style lang="scss">
    .query-join-container{
      @apply min-w-80 max-w-80 border-r flex flex-col;

      .join-how{
        @apply flex-1 flex flex-col justify-center cursor-pointer items-center h-full hover:bg-blue-50;
      }
    }

    .join-column-header{
      @apply flex flex-1 p-1 justify-center items-center;
    }
</style>

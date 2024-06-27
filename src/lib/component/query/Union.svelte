<script>
    import CircleAddButton from "./CircleAddButton.svelte";
    import DataSourceSelector from "./DataSourceSelector.svelte";
    import {Button,Select,FieldInput,Icon,removeIcon} from "../../youi";

    /**
     * 合并信息
     */
    export let unionColumns = [];

    /**
     * 合并表reader集合
     */
    export let readers = [];
    /**
     * 输入列
     */
    export let inputColumns = [];

    // union reader bind

    let selectorDialog;

    let reader = '';

    let uri = '';

    let columns = [];

    let selectedColumnNames = [];

    $:if(unionColumns.length === 0 && inputColumns.length){
        unionColumns = inputColumns.map(c=>({
            name:c.name,
            inputName:c.name,
            dataType:c.dataType,
            unionNames:[]
        }));
    }

    /**
     *
     */
    const openAddUnionReader = () => {
        if(selectorDialog){
            selectorDialog.showModal();
        }
    }

    /**
     *
     */
    const selectDs = () => {
        if(reader && uri && columns.length && selectedColumnNames.length){
            const index = readers.length;
            readers.push({
                reader,
                uri,
                columns,
                selectedColumnNames
            });

            unionColumns = unionColumns.map(uc=>{
                uc.unionNames = uc.unionNames||[];
                uc.unionNames[index] = '';
                return uc;
            });

            readers = readers;
            closeDsSelector();
        }
    }

    const closeDsSelector = () => {
        reader = '';
        uri = '';
        columns = [];
        selectedColumnNames = [];
        if(selectorDialog){
            selectorDialog.close()
        }
    }

    const doRemoveReader = (reader,index) => {
        readers.splice(index,1);
        unionColumns = unionColumns.map(uc=>{
            uc.unionNames.splice(index,1);
            return uc;
        });
        readers = readers;
    }

    $:inputItems = inputColumns.map(c=>({...c,icon:c.dataType}));
</script>
<div class="min-w-40 border-r">
    <div class="w-full h-12 flex justify-center items-center bg-blue-50 mb-1">合并结果</div>
    <div class="w-full h-8 flex justify-center items-center bg-green-50 mb-0.5">当前表</div>
    {#each readers as reader,index}
        <div class="w-full h-8 flex py-0.5 mb-0.5 relative">
            <div class="flex-1 flex bg-yellow-50 justify-center items-center">合并表{index+1}</div>
            <div on:click={()=>doRemoveReader(reader,index)}
                    class="absolute right-0 h-full w-6 flex justify-center items-center hover:text-red-500 cursor-pointer">
                <Icon data={removeIcon}/>
            </div>
        </div>
    {/each}
    <div class="flex justify-center p-1">
        <CircleAddButton title="添加合并表" on:click={()=>openAddUnionReader()}/>
    </div>
</div>
<div class="flex-1 flex flex-col overflow-x-auto relative w-0">
    <div class="whitespace-nowrap">
        <div class="flex">
            {#each unionColumns as column}
                <div class="min-w-32 overflow-hidden">
                    <div class="flex items-center h-12 mb-1 p-1">
                        <FieldInput class="mb-0" bind:value={column.name}/>
                    </div>
                    <div class="flex items-center h-8 p-1 mb-1">
                        <Select valueProp={'name'} bind:value={column.inputName} listWidth={'200px'} items = {inputItems}/>
                    </div>

                    {#if column.unionNames}
                        {#each column.unionNames as unionName,index}
                            <div class="flex items-center h-8 p-1 mb-1">
                                <Select valueProp={'name'} items={readers[index].columns} bind:value={unionName} listWidth={'200px'} />
                            </div>
                        {/each}
                    {/if}
                </div>
            {/each}
            <div class="min-w-20 h-full">
                &nbsp;
            </div>
        </div>
    </div>
</div>


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
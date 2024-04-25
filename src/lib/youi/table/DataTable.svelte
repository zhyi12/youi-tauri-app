<script lang="ts">
    import {createEventDispatcher} from "svelte";
    import { writable } from 'svelte/store'
    import {
        createSvelteTable,
        flexRender,
        getCoreRowModel,
        getSortedRowModel,
        getPaginationRowModel,
    } from '@tanstack/svelte-table'

    import type {TableOptions,SortingState,OnChangeFn} from '@tanstack/svelte-table'

    const dispatch = createEventDispatcher();

    export let records = [];

    export let columns = [];

    export let sorting: SortingState = [];
    //从1开始
    export let pageIndex = 1;
    export let pageSize = 15;
    export let totalCount = 0;

    const setSorting: OnChangeFn<SortingState> = updater => {
        //不启用页面排序
    }

    //不启用前台组件分页，state.pagination.pageIndex恒定为0
    const options = writable<TableOptions<Record<string, any>>>({
        data: records,
        columns,
        getCoreRowModel: getCoreRowModel(),
        state: {
            sorting,
            pagination:{pageIndex:0,pageSize}
        },
        onSortingChange: setSorting,
        getSortedRowModel: getSortedRowModel(),
        getPaginationRowModel:getPaginationRowModel(),
    })

    // table model (store)
    export let table = createSvelteTable(options)

    const refreshData = ()=>{
        options.update(old => ({
            ...old,
            pageCount:Math.ceil(totalCount/pageSize),
            state:{
                ...old.state,
                pagination:{pageIndex:0,pageSize}
            },
            data:records
        }))
    }

    $: if(records.length || totalCount || pageSize){
        //更新数据
        refreshData()
    }

    $:paginationItems = buildPaginationItems($table.getPageCount(),pageIndex);

    /**
     *
     * @param toPageIndex
     */
    const goPage = (toPageIndex) => {
        if(toPageIndex !== pageIndex){
            dispatch('pagination',{pageIndex:toPageIndex,fromIndex:pageIndex});
        }
    }

    /**
     * 构建分页信息
     * @param pageCount
     */
    function buildPaginationItems(pageCount,pageIndex){
        let items= [];

        let start = Math.max(1,pageIndex-2);
        let end = Math.min(pageIndex+2,pageCount);

        if(end<5 && pageCount>5){
            end = 5;
        }

        if(pageCount == end && pageCount>5){
            start = pageCount-4;
        }

        for(let i=start;i<=end;i++){
            items.push({index:i});
        }

        return items;
    }

</script>

<div class="p-2 overflow-x-auto">
    <table class="table table-zebra table-pin-rows table-pin-cols">
        <thead>
        {#each $table.getHeaderGroups() as headerGroup}
            <tr>
                {#each headerGroup.headers as header}
                    <th class="text-sm bg-gray-100 font-bold"
                            on:click={header.column.getToggleSortingHandler()}
                    >
                        {#if !header.isPlaceholder}
                            <svelte:component
                                    this={flexRender(header.column.columnDef.header,header.getContext())}
                            />

                            {#if header.column.getIsSorted().toString() === 'asc'}
                                🔼
                            {:else if header.column.getIsSorted().toString() === 'desc'}
                                🔽
                            {/if}
                        {/if}
                    </th>
                {/each}
            </tr>
        {/each}
        </thead>
        <tbody>
        {#each $table.getRowModel().rows as row}
            <tr class="hover">
                {#each row.getVisibleCells() as cell}
                    <td>
                        <svelte:component this={flexRender(cell.column.columnDef.cell, cell.getContext())}/>
                    </td>
                {/each}
            </tr>
        {/each}
        </tbody>
        <tfoot>
        </tfoot>
    </table>

    <div class="pt-1">
        <div class="join">
            {#each paginationItems as item}
                <button class="join-item btn btn-sm"
                        class:bg-gray-600={pageIndex === item.index}
                        class:text-white={pageIndex === item.index}
                        on:click={()=>goPage(item.index)}>{item.index.toLocaleString()}</button>
            {/each}
        </div>

        {pageIndex.toLocaleString()}/{$table.getPageCount().toLocaleString()}
    </div>

</div>

<script lang="ts">
    import {onMount} from "svelte";

    import {DataTable,Toolbar,Button,Icon,plusIcon} from "$lib/youi";

    import {metaItemStore} from "$lib/app-stores/dmp/metaItemStore";

    import type {ColumnDef} from "@tanstack/table-core";
    import type {MetaItem} from "$lib/app-entity/dmp/metaItem";
    import Editor from "./Editor.svelte";

    let pageSize = 10;
    let pageIndex = 1;
    let editorIsOpen = false;

    let columns:ColumnDef<MetaItem>[] = [
        {
            accessorKey: 'name',
            header:'数据项名'
        }, {
            accessorKey: 'caption',
            header:'中文描述'
        },{
            accessorKey: 'column_name',
            header:'列名'
        }, {
            accessorKey: 'full_caption',
            header:'中文全称'
        }
    ];

    const handle_pagination = async ({detail}) => {
        await doQuery(detail.pageIndex);
    }

    const openAddMetaItem = async () => {
        editorIsOpen = true;
    }

    const doQuery =  async (queryPageIndex) => {
        pageIndex = queryPageIndex;
        await metaItemStore.findByPager({pageIndex:queryPageIndex,pageSize})
    }

    onMount(async ()=>{
        await doQuery(pageIndex);
    });

</script>

<Toolbar>
    <Button title="新增数据项" class="btn-sm btn-ghost" tooltipPosition="top" on:click={()=>openAddMetaItem()}>
        <Icon data={plusIcon}/>
    </Button>
</Toolbar>

<DataTable
        {pageIndex} {pageSize}
        on:pagination={handle_pagination}
        records={$metaItemStore.records} {columns}
           totalCount={$metaItemStore.totalCount}/>

<Editor bind:isOpen={editorIsOpen}/>

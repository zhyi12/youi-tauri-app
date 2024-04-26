<script lang="ts">
    import {onMount} from "svelte";

    import {DataTable,ColumnButtons,Toolbar,Button,Icon,plusIcon,cellComponent} from "$lib/youi";
    import {metaItemStore} from "$lib/app-stores/dmp/metaItemStore";
    import {APP_MESSAGE} from "$lib/app-page/page.message";
    import Editor from "./Editor.svelte";

    import type {ColumnDef} from "@tanstack/table-core";
    import type {MetaItem} from "$lib/app-entity/dmp/metaItem";

    /**
     *
     */
    const NEW_META_ITEM:MetaItem = {
        id:'',
        caption:''
    };

    let pageSize = 10;
    let pageIndex = 1;
    let editorIsOpen = false;
    let editorTitle = '';

    let editingMetaItem = {...NEW_META_ITEM};

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
        },{
            accessorKey:'operator',
            header:'操作',
            cell:info=>{
                return cellComponent(ColumnButtons,{
                    record:{...info.row.original},
                    buttons:[
                        {name:"modify", text:'修改', action:openModify},
                        {name:"remove", text:'删除', action:doRemoveMetaItem},
                    ]
                })
            }
        }
    ];

    /**
     * 回掉函数 - 数据表格分页事件
     * @param detail
     */
    const handle_pagination = async ({detail}) => {
        await doQuery(detail.pageIndex);
    }

    /**
     * 打开数据项新建窗口
     */
    const openAddMetaItem = async () => {
        editingMetaItem = {...NEW_META_ITEM};
        editorTitle = '新增数据项';
        editorIsOpen = true;
    }

    /**
     * 打开数据项修改窗口
     * @param metaItem
     */
    const openModify = (metaItem) => {
        editingMetaItem = {...metaItem};
        editorTitle = '修改数据项';
        editorIsOpen = true;
    }

    /**
     * 删除数据项
     * @param metaItem
     */
    const doRemoveMetaItem = async (metaItem) => {
        const passed = await APP_MESSAGE.confirm(`确认删除数据项-${metaItem.caption}?`);
        if(passed){
            metaItemStore.removeMetaItem(metaItem.id)
        }
    }

    /**
     * 查询数据
     * @param queryPageIndex
     */
    const doQuery =  async (queryPageIndex) => {
        pageIndex = queryPageIndex;
        await metaItemStore.findByPager({pageIndex:queryPageIndex,pageSize})
    }

    onMount(async ()=>{
        await doQuery(pageIndex);
    });

</script>

<Toolbar>
    <Button title="新增数据项" class="btn-sm btn-ghost" tooltipPosition="right" on:click={()=>openAddMetaItem()}>
        <Icon data={plusIcon}/>
    </Button>
</Toolbar>
<!-- -->
<DataTable
        {pageIndex} {pageSize}
        on:pagination={handle_pagination}
        records={$metaItemStore.records} {columns}
           totalCount={$metaItemStore.totalCount}>
</DataTable>
<!-- -->
<Editor title={editorTitle} {...editingMetaItem} bind:isOpen={editorIsOpen}/>
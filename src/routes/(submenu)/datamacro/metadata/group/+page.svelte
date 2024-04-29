<script lang="ts">
    import {onMount} from "svelte";
    import {goto} from "$app/navigation";
    import {DataTable,ColumnButtons,Toolbar,Button,Icon,plusIcon,cellComponent} from "$lib/youi";
    import {macroGroupStore} from "$lib/app-stores/datamacro/macroGroupStore";
    import {APP_MESSAGE} from "$lib/app-page/page.message";
    import Editor from "./Editor.svelte";

    import type {ColumnDef} from "@tanstack/table-core";
    import type {MacroGroup} from "$lib/app-entity/datamacro/macroGroup";

    /**
     *
     */
    const NEW_RECORD:MacroGroup = {
        id:'',
        text:''
    };

    let pageSize = 10;
    let pageIndex = 1;
    let editorIsOpen = false;
    let editorTitle = '';

    let editingMacroGroup = {...NEW_RECORD};

    let columns:ColumnDef<MacroGroup>[] = [
        {
            accessorKey: 'text',
            header:'数据项名'
        },{
            accessorKey:'operator',
            header:'操作',
            cell:info=>{
                return cellComponent(ColumnButtons,{
                    record:{...info.row.original},
                    buttons:[
                        {name:"modify", text:'修改', action:openModify},
                        {name:"groupItem", text:'分组项', action:openGroupItem},
                        {name:"remove", text:'删除', action:doRemovemacroGroup},
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
     * 打开宏观分组新建窗口
     */
    const openAddMacroGroup = async () => {
        editingMacroGroup = {...NEW_RECORD};
        editorTitle = '新增宏观分组';
        editorIsOpen = true;
    }

    const openGroupItem = async(macroGroup) => {
        await goto(`/datamacro/metadata/group/${macroGroup.id}-group-item`);
    }

    /**
     * 打开宏观分组修改窗口
     * @param macroGroup
     */
    const openModify = (macroGroup) => {
        editingMacroGroup = {...macroGroup};
        editorTitle = '修改宏观分组';
        editorIsOpen = true;
    }

    /**
     * 删除数据项
     * @param macroGroup
     */
    const doRemovemacroGroup = async (macroGroup) => {
        const passed = await APP_MESSAGE.confirm(`确认删除宏观分组-${macroGroup.text}?`);
        if(passed){
            macroGroupStore.removeMacroGroup(macroGroup.id)
        }
    }

    /**
     * 查询数据
     * @param queryPageIndex
     */
    const doQuery =  async (queryPageIndex) => {
        pageIndex = queryPageIndex;
        await macroGroupStore.findByPager({pageIndex:queryPageIndex,pageSize})
    }

    onMount(async ()=>{
        await doQuery(pageIndex);
    });

</script>

<Toolbar>
    <Button title="新增宏观分组" class="btn-sm btn-ghost" tooltipPosition="right" on:click={()=>openAddMacroGroup()}>
        <Icon data={plusIcon}/>
    </Button>
</Toolbar>
<!-- -->
<DataTable
        {pageIndex} {pageSize}
        on:pagination={handle_pagination}
        records={$macroGroupStore.records} {columns}
           totalCount={$macroGroupStore.totalCount}>
</DataTable>
<!-- -->
<Editor title={editorTitle} {...editingMacroGroup} bind:isOpen={editorIsOpen}/>
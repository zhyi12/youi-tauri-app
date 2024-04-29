<script lang="ts">
    import {onMount,setContext} from "svelte";
    import {goto} from "$app/navigation";
    import {page} from "$app/stores";
    import {Tree,TreeNodeEditor,Toolbar,Button,Icon,plusIcon,renameIcon,closeIcon,addChildIcon,findPathNodes} from "$lib/youi";
    import {treeStore} from "$lib/app-stores/datamacro/macroGroupItemStore";
    import {APP_MESSAGE} from "$lib/app-page/page.message";
    import {writable} from "svelte/store";

    const TREE_BUTTONS = [
        {
            name:'rename', text:'重命名', icon:renameIcon,
            action:(id,newName)=>{
                treeStore.rename(id,newName).then(t=>t);
                editingNode.set({id,text:newName});
            }
        },{
            name:'addChild', text:'新增下级分组项', icon:addChildIcon,
            action:async (id)=>await doAddMacroGroupItem(id,'child')
        },{
            name:'insertBefore', text:'上插入分组项', icon:addChildIcon,
            action:async (id)=>await doAddMacroGroupItem(id,'before')
        },{
            name:'insertAfter', text:'下插入分组项', icon:addChildIcon,
            action:async (id)=>await doAddMacroGroupItem(id,'after')
        },{
            name:'remove', text:'删除分组项', icon:closeIcon,
            action:async (id,text)=>await doRemoveMacroGroupItem(id,text)
        }
    ];

    export let data;

    let expandedIds = [];

    const editingNode = writable({});
    setContext('PageContext',{editingNode});

    /**
     *
     */
    const doAddMacroGroupItem = async (refId,position) => {
        const macroGroupItem = {
            text:'新分组项',
            group_id:$page.params.groupId,
            num:1
        };
        if('child' === position){
            Object.assign(macroGroupItem,{pid:refId});
            await treeStore.addChild(macroGroupItem,macroGroupItem.group_id);
        }else if('before' === position){
            await treeStore.insertBefore(refId,macroGroupItem,macroGroupItem.group_id);
        }else if('after' === position){
            await treeStore.insertAfter(refId,macroGroupItem,macroGroupItem.group_id);
        }
        //定位到新分组项
        if(macroGroupItem.id){
            let paths = [];
            if(macroGroupItem.pid){
                paths = findPathNodes(macroGroupItem.pid).map(n=>n.id);
            }
            paths.push(macroGroupItem.id);
            goto(`${data.baseUri}/${paths.join('/')}`);
        }
    }

    /**
     * 删除分组项
     * @param id
     * @param text
     */
    const doRemoveMacroGroupItem = async (id,text) => {
        const passed = await APP_MESSAGE.confirm(`确认删除${text}！`);
        if(passed){
            await treeStore.removeNode(id,(node,redirectPaths)=>{
                //删除后定位到相邻节点
                goto(`${data.baseUri}/${redirectPaths.join('/')}`);
            });
        }
    }

    /**
     *
     * @param detail
     */
    const handle_node_move = async ({detail}) => {
        const {from,to,position} = detail;
        await treeStore.move(from,to,position);
    }

    /**
     *
     * @param detail
     */
    const handle_node_select = ({detail}) => {
        const {id} = detail;
        const pathNodes = findPathNodes($treeStore.nodes,id);
        const path = pathNodes.map(p=>p.id).join('/');
        if(id){
            goto(`${data.baseUri}/${path}`);
        }
    }
    /**
     *
     * @param e
     */
    const acceptDropping = (e) => {
        //drag
        return true;
    }

    onMount(async ()=>{
        const defaultExpandedIds = [...data.expandedNodeIds];
        await treeStore.fetch({group_id:$page.params.groupId},data.activeNodeId,defaultExpandedIds);
        expandedIds = defaultExpandedIds;
        console.log(expandedIds)
    })
</script>

<div class="flex flex-1 h-0">
    <div class="w-60 flex flex-col">
        <Toolbar>
            <Button class="btn-ghost btn-xs" title="新增分组项" on:click={()=>doAddMacroGroupItem('','child')}>
                <Icon class="w-4 h-4" data={plusIcon}/>
            </Button>
        </Toolbar>
        <Tree children={$treeStore.nodes}
              expandedIds={expandedIds}
              selectedIds = {[$treeStore.activeId]}
              on:move={handle_node_move}
              on:select={handle_node_select}
              {acceptDropping}
              class="w-full flex-1 h-0 overflow-y-auto overflow-x-hidden"
              nodeRender={({id,text,hovered,selected})=>({component:TreeNodeEditor,props:{id,text,hovered,selected,buttons:TREE_BUTTONS}})}/>
        <div class="youi-dropping-backup"></div>
    </div>

    <div class="flex-1 border-l flex flex-col">
        <slot/>
    </div>
</div>
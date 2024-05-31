<script lang="ts">
    import {onMount,setContext} from "svelte";
    import {writable} from "svelte/store";
    import {goto} from "$app/navigation";
    import {Tree,TreeNodeEditor,Toolbar,Button,Icon,plusIcon,renameIcon,closeIcon,addChildIcon,findPathNodes} from "$lib/youi";
    import {treeStore} from "$lib/app-stores/dmp/dashboardGroupStore";
    import {APP_MESSAGE} from "$lib/app-page/page.message";
    import {folderIcon, folderOpenIcon} from "../../../../lib/app-icons";

    const TREE_BUTTONS = [
        {
            name:'rename', text:'重命名', icon:renameIcon,
            action:(id,newName)=>{
                treeStore.rename(id,newName).then(t=>t);
                editingNode.set({id,text:newName});
            }
        },{
            name:'addChild', text:'新增下级数据看版分组', icon:addChildIcon,
            action:async (id)=>await doAddDashboardGroup(id,'child')
        },{
            name:'insertBefore', text:'上插入数据看版分组', icon:addChildIcon,
            action:async (id)=>await doAddDashboardGroup(id,'before')
        },{
            name:'insertAfter', text:'下插入数据看版分组', icon:addChildIcon,
            action:async (id)=>await doAddDashboardGroup(id,'after')
        },{
            name:'remove', text:'删除数据看版分组', icon:closeIcon,
            action:async (id,text)=>await doRemoveDashboardGroup(id,text)
        }
    ];

    const editingNode = writable({});
    setContext('PageContext',{editingNode});

    export let data;

    let expandedIds = [];
    /**
     *
     */
    const doAddDashboardGroup = async (refId,position) => {
        const dashboardGroup = {
            text:'新数据看版分组',
            num:1
        };
        if('child' === position){
            Object.assign(dashboardGroup,{pid:refId});
            await treeStore.addChild(dashboardGroup);
        }else if('before' === position){
            await treeStore.insertBefore(refId,dashboardGroup);
        }else if('after' === position){
            await treeStore.insertAfter(refId,dashboardGroup);
        }
        //定位到新数据看版分组
        if(dashboardGroup.id){
            let paths = [];
            if(dashboardGroup.pid){
                paths = findPathNodes(dashboardGroup.pid).map(n=>n.id);
            }
            paths.push(dashboardGroup.id);
            goto(`${data.baseUri}/${paths.join('/')}`);
        }
    }

    const doRemoveDashboardGroup = async (id,text) => {
        const passed = await APP_MESSAGE.confirm(`确认删除${text}！`);
        if(passed){
            await treeStore.removeNode(id,(node,redirectPaths)=>{
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
        expandedIds = [...data.expandedNodeIds];
        await treeStore.fetch({},data.activeNodeId,expandedIds);
    })
</script>

<div class="flex flex-1 h-0">
    <div class="w-60 flex flex-col">
        <Toolbar>
            <Button class="btn-ghost btn-xs" title="新增数据看版分组" on:click={()=>doAddDashboardGroup('','child')}>
                <Icon class="w-4 h-4" data={plusIcon}/>
            </Button>
        </Toolbar>
        <Tree children={$treeStore.nodes}
              icons={(iconName,expanded)=>{
                  return expanded?folderOpenIcon:folderIcon;
              }}
              {expandedIds}
              selectedIds = {[data.activeNodeId]}
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
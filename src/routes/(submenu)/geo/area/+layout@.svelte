<script lang="ts">
    import {onMount,setContext} from "svelte";
    import {writable} from "svelte/store";
    import {page} from "$app/stores";
    import {goto,afterNavigate} from "$app/navigation";
    import {Tree,TreeNodeEditor,Toolbar,Button,Icon,plusIcon,saveIcon,renameIcon,closeIcon,addChildIcon,findPathNodes} from "$lib/youi";
    import {upIcon} from "$lib/app-icons";
    import {APP_MESSAGE} from "$lib/app-page/page.message";
    import {treeStore,createStore} from "$lib/app-stores/base/areaStore";

    import AddArea from "./AddArea.svelte";


    const TREE_BUTTONS = [
        {
            name:'rename', text:'重命名', icon:renameIcon,
            action:(id,newName)=>{
                (showSecondTree?secondTreeStore:treeStore).rename(id,newName).then(t=>t);
                editingNode.set({id,text:newName});
            }
        }
    ];

    // 二级树
    const secondTreeStore = createStore();

    const editingNode = writable({});
    setContext('PageContext',{editingNode});

    export let data;

    let expandedIds = [];

    let isEditing = false;
    let editing = {};

    const openAddArea = (refId,position) => {
        isEditing = true;
        editing = {pid:refId};
    }

    /**
     *
     */
    const doAddArea = async (refId,position) => {
        const area = {
            text:'新行政区划',
            num:1
        };
        if('child' === position){
            Object.assign(area,{pid:refId});
            await treeStore.addChild(area);
        }else if('before' === position){
            await treeStore.insertBefore(refId,area);
        }else if('after' === position){
            await treeStore.insertAfter(refId,area);
        }
        //定位到新行政区划
        if(area.id){
            let paths = [];
            if(area.pid){
                paths = findPathNodes(area.pid).map(n=>n.id);
            }
            paths.push(area.id);
            goto(`${data.baseUri}/${paths.join('/')}`);
        }
    }

    const doRemoveArea = async (id,text) => {
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
        const pathNodes = findPathNodes(showSecondTree?$secondTreeStore.nodes:$treeStore.nodes,id);
        const path = pathNodes.map(p=>p.id).join('/');
        if(id){
            const url = `${data.baseUri}/${$page.params.rootId||'_root'}/${path}`;
            goto(url);
        }
    }

    const handle_node_dblclick = ({detail}) => {
        const {id} = detail;
        const pathNodes = findPathNodes(showSecondTree?$secondTreeStore.nodes:$treeStore.nodes,id);
        const path = pathNodes.map(p=>p.id).join('/');
        if(id && !showSecondTree){
            goto(`${data.baseUri}/${id}/${path}`);
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

    $:showSecondTree = $page.params.rootId && $page.params.rootId!='_root';

    onMount(async ()=>{
        const rootId = data.appConfig.areaId||'';
        expandedIds = [...data.expandedNodeIds];
        await treeStore.fetch({pid:rootId,maxLevel:4},data.activeNodeId,expandedIds);
    })

    afterNavigate(()=>{
        if(showSecondTree){
            secondTreeStore.fetch({pid:$page.params.rootId,maxLevel:4},data.activeNodeId,expandedIds);
        }
    });

    const back = () => {
        goto(`${data.baseUri}`);
    }
</script>

<div class="flex flex-1">
    <div class="w-60 flex flex-col">
        <Toolbar>
            {#if showSecondTree}
                <Button class="btn-ghost btn-xs" title="返回上级" on:click={()=>back()}>
                    <Icon class="w-4 h-4" data={upIcon}/>
                </Button>
            {/if}
            <Button class="btn-ghost btn-xs" title="新增区划" on:click={()=>openAddArea('','child')}>
                <Icon class="w-4 h-4" data={plusIcon}/>
            </Button>
        </Toolbar>

        <Tree children={showSecondTree?$secondTreeStore.nodes:$treeStore.nodes}
              {expandedIds}
              selectedIds = {[data.activeId]}
              on:move={handle_node_move}
              on:select={handle_node_select}
              on:dblclick={handle_node_dblclick}
              {acceptDropping}
              class="w-full flex-1 h-0 overflow-y-auto overflow-x-hidden"
              nodeRender={({id,text,hovered,selected})=>({component:TreeNodeEditor,props:{id,text,hovered,selected,buttons:TREE_BUTTONS}})}/>

    </div>

    <div class="flex-1 border-l flex flex-col">
        <slot/>
    </div>
</div>

<AddArea bind:isOpen={isEditing} record={editing}/>


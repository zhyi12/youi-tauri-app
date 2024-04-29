<script lang="ts">
    import {onMount,setContext} from "svelte";
    import {goto} from "$app/navigation";
    import {Tree,TreeNodeEditor,Toolbar,Button,Icon,plusIcon,renameIcon,closeIcon,addChildIcon,findPathNodes} from "$lib/youi";
    import {treeStore} from "$lib/app-stores/datamacro/macroIndicatorStore";
    import {APP_MESSAGE} from "$lib/app-page/page.message";
    import {writable} from "svelte/store";

    const TREE_BUTTONS = [
        {
            name:'rename', text:'重命名', icon:renameIcon,
            action:async (id,newName)=>{
                await treeStore.rename(id,newName).then(t=>t);
                editingNode.set({id,text:newName});
            }
        },{
            name:'addChild', text:'新增下级指标', icon:addChildIcon,
            action:async (id)=>await doAddMacroIndicator(id,'child')
        },{
            name:'insertBefore', text:'上插入指标', icon:addChildIcon,
            action:async (id)=>await doAddMacroIndicator(id,'before')
        },{
            name:'insertAfter', text:'下插入指标', icon:addChildIcon,
            action:async (id)=>await doAddMacroIndicator(id,'after')
        },{
            name:'remove', text:'删除', icon:closeIcon,
            action:async (id,text)=>await doRemoveMacroIndicator(id,text)
        }
    ];

    const editingNode = writable({});
    setContext('PageContext',{editingNode});

    export let data;

    let expandedIds = [];

    /**
     *
     */
    const doAddMacroIndicator = async (refId,position) => {
        const macroIndicator = {
            text:'新指标',
            num:1
        };
        if('child' === position){
            Object.assign(macroIndicator,{pid:refId});
            await treeStore.addChild(macroIndicator);
        }else if('before' === position){
            await treeStore.insertBefore(refId,macroIndicator);
        }else if('after' === position){
            await treeStore.insertAfter(refId,macroIndicator);
        }
        //定位到新指标
        if(macroIndicator.id){
            let paths = [];
            if(macroIndicator.pid){
                paths = findPathNodes(macroIndicator.pid).map(n=>n.id);
            }
            paths.push(macroIndicator.id);
            goto(`${data.baseUri}/${paths.join('/')}`);
        }
    }

    const doRemoveMacroIndicator = async (id,text) => {
        const passed = await APP_MESSAGE.confirm(text,`确认删除指标？`);
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
            <Button class="btn-ghost btn-xs" title="新增宏观指标" on:click={()=>doAddMacroIndicator('','child')}>
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

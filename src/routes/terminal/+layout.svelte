<script lang="ts">
    import {onMount,setContext} from "svelte";
    import {writable} from "svelte/store";
    import {goto} from "$app/navigation";
    import {Tree,TreeNodeEditor,Toolbar,Button,Icon,plusIcon,renameIcon,closeIcon,addChildIcon,findPathNodes} from "$lib/youi";
    import {treeStore} from "$lib/app-stores/base/dslScriptStore";
    import {APP_MESSAGE} from "$lib/app-page/page.message";

    const TREE_BUTTONS = [
        {
            name:'rename', text:'重命名', icon:renameIcon,
            action:(id,newName)=>{
                treeStore.rename(id,newName).then(t=>t);
                editingNode.set({id,text:newName});
            }
        },{
            name:'addChild', text:'新增下级DSL脚本', icon:addChildIcon,
            action:async (id)=>await doAddDslScript(id,'child')
        },{
            name:'insertBefore', text:'上插入DSL脚本', icon:addChildIcon,
            action:async (id)=>await doAddDslScript(id,'before')
        },{
            name:'insertAfter', text:'下插入DSL脚本', icon:addChildIcon,
            action:async (id)=>await doAddDslScript(id,'after')
        },{
            name:'remove', text:'删除DSL脚本', icon:closeIcon,
            action:async (id,text)=>await doRemoveDslScript(id,text)
        }
    ];

    const editingNode = writable({});
    setContext('PageContext',{editingNode});

    export let data;

    let expandedIds = [];
    /**
     *
     */
    const doAddDslScript = async (refId,position) => {
        const dslScript = {
            text:'新DSL脚本',
            num:1
        };
        if('child' === position){
            Object.assign(dslScript,{pid:refId});
            await treeStore.addChild(dslScript);
        }else if('before' === position){
            await treeStore.insertBefore(refId,dslScript);
        }else if('after' === position){
            await treeStore.insertAfter(refId,dslScript);
        }
        //定位到新DSL脚本
        if(dslScript.id){
            let paths = [];
            if(dslScript.pid){
                paths = findPathNodes(dslScript.pid).map(n=>n.id);
            }
            paths.push(dslScript.id);
            goto(`${data.baseUri}/${paths.join('/')}`);
        }
    }

    const doRemoveDslScript = async (id,text) => {
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
            console.log(`${data.baseUri}/${path}`)
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


<div class="w-60 flex flex-col">
    <Toolbar>
        <Button class="btn-ghost btn-xs" title="新增DSL脚本" on:click={()=>doAddDslScript('','child')}>
            <Icon class="w-4 h-4" data={plusIcon}/>
        </Button>
    </Toolbar>
    <Tree children={$treeStore.nodes}
          {expandedIds}
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

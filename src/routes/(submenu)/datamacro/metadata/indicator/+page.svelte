<script lang="ts">
    import {onMount} from "svelte";
    import {Tree,TreeNodeEditor,Toolbar,Button,Icon,plusIcon,renameIcon,closeIcon,addChildIcon} from "$lib/youi";
    import {treeStore} from "$lib/app-stores/datamacro/macroIndicatorStore";
    import {APP_MESSAGE} from "$lib/app-page/page.message";

    const TREE_BUTTONS = [
        {
            name:'rename',
            text:'重命名',
            icon:renameIcon,
            action:(id,newName)=>{
                treeStore.rename(id,newName).then(t=>t);
            }
        },{
            name:'addChild',
            text:'新增下级指标',
            icon:addChildIcon,
            action:async (id)=>await doAddMacroIndicator(id,'child')
        },{
            name:'insertBefore',
            text:'上插入指标',
            icon:addChildIcon,
            action:async (id)=>await doAddMacroIndicator(id,'before')
        },{
            name:'insertAfter',
            text:'下插入指标',
            icon:addChildIcon,
            action:async (id)=>await doAddMacroIndicator(id,'after')
        },{
            name:'remove',
            text:'删除',
            icon:closeIcon,
            action:async (id,text)=>{
                const passed = await APP_MESSAGE.confirm(`确认删除${text}！`);
                if(passed){
                    await treeStore.removeNode(id);
                }
            }
        }
    ];

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

    }

    const handle_node_move = async ({detail}) => {
        const {from,to,position} = detail;
        await treeStore.move(from,to,position);
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
        await treeStore.fetch();
    })
</script>

<div class="flex flex-1 h-0">
    <div class="w-60 flex flex-col">
        <Toolbar>
            <Button class="btn-ghost btn-xs" title="新增指标" on:click={()=>doAddMacroIndicator('','child')}>
                <Icon class="w-4 h-4" data={plusIcon}/>
            </Button>
        </Toolbar>
        <Tree on:move={handle_node_move}
                {acceptDropping}
                class="w-full flex-1 h-0 overflow-y-auto overflow-x-hidden" children={$treeStore.nodes} nodeRender={({id,text,hovered,selected})=>{
            return {
               component:TreeNodeEditor,props:{id,text,hovered,selected,buttons:TREE_BUTTONS}
            }
        }}/>
        <div class="youi-dropping-backup"></div>
    </div>

    <div class="flex-1 border-l">
        22
    </div>
</div>

<script lang="ts">
    import {getContext} from "svelte";
    import {afterNavigate} from "$app/navigation";
    import {Toolbar,Button,Icon,saveIcon,findTreeNode} from "$lib/youi";
    import {isEqual} from "lodash";
    import {treeStore} from "$lib/app-stores/datamacro/macroIndicatorStore";

    export let data;

    const {editingNode} = getContext("PageContext");

    /**
     * 原始数据
     */
    let orgRecord = undefined;
    let record = {};
    /**
     * 监听数据变化
     */
    $: dirty = !isEqual(orgRecord,record);

    // 监听上级页面树节点重命名，更新当前编辑的text
    $: if($editingNode.id == data.record.id){
        afterTreeNodeChange();
    }

    function afterTreeNodeChange() {
        record.text = $editingNode.text;
        orgRecord.text = record.text;
    }

    /**
     * 保存数据
     */
    const doSave = async () => {
        // 未变化直接返回
        if(!dirty)return;
        // 数据校验
        await treeStore.update(record);
        orgRecord = {...record};
    }

    afterNavigate(()=>{
        orgRecord = {...data.record};
        record = {...data.record};
    });

</script>

<Toolbar>
    <Button class="btn-ghost btn-xs" disabled={!dirty} title="保存" on:click={()=>doSave()}>
        <Icon data={saveIcon}/>
    </Button>
</Toolbar>
<div class="p-1 flex-1">
    <label class="input input-sm input-bordered flex items-center gap-2 mb-2">
        名称：
        <input type="text" class="grow" bind:value={record.text} placeholder="请输入宏观指标名称"/>
    </label>

    <textarea class="textarea textarea-bordered w-full" bind:value={record.desc} rows="10" placeholder="请输入指标解释"></textarea>
</div>

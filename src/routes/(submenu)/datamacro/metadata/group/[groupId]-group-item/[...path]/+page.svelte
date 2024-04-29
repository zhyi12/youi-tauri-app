<script lang="ts">
    import {getContext} from "svelte";
    import {afterNavigate} from "$app/navigation";
    import {Toolbar,Button,Icon,saveIcon} from "$lib/youi";
    import {isEqual} from "lodash";
    import {treeStore} from "$lib/app-stores/datamacro/macroGroupItemStore";

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
        <input type="text" class="grow" bind:value={record.text} placeholder="请输入名称"/>
    </label>
    <label class="input input-sm input-bordered flex items-center gap-2 mb-2">
        分组项解释：
        <input type="text" class="grow" bind:value={record.desc} placeholder="请输入分组项解释"/>
    </label>
</div>
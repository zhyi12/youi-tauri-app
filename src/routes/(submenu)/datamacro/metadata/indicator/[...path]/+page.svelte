<script lang="ts">

    import {afterNavigate} from "$app/navigation";
    import {Toolbar,Button,Icon,saveIcon} from "$lib/youi";
    import {isEqual} from "lodash";
    import {treeStore} from "$lib/app-stores/datamacro/macroIndicatorStore";

    export let data;

    /**
     * 原始数据
     */
    let orgRecord = undefined;
    /**
     * 监听数据变化
     */
    $: dirty = !isEqual(orgRecord,data.record);
    /**
     * 保存数据
     */
    const doSave = async () => {
        // 未变化直接返回
        if(!dirty)return;
        // 数据校验
        await treeStore.update(data.record);
    }

    afterNavigate(()=>{
        orgRecord = {...data.record};
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
        <input type="text" class="grow" bind:value={data.record.text} placeholder="请输入宏观指标名称"/>
    </label>

    <textarea class="textarea textarea-bordered w-full" bind:value={data.record.desc} rows="10" placeholder="请输入指标解释"></textarea>
</div>

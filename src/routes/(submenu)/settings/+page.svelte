<script>

    import {onMount} from "svelte";
    import {Toolbar,Button,Icon,saveIcon} from "$lib/youi";
    import {findConfig,saveConfig} from "$lib/app-services/base/configServices";

    const DEFAULT_CONFIG = {
        dataDir:'',
        areaId:'',
        ownerAreaId:'',
    };

    let config = {
        ...DEFAULT_CONFIG
    };
    
    const doSave = async () => {
        await saveConfig(config);
    }

    onMount(async ()=>{
        config = await findConfig();
    });

</script>

<Toolbar>
    <Button title="保存配置" class="btn-ghost btn-xs" on:click={()=>doSave()}>
        <Icon data={saveIcon}/>
    </Button>
</Toolbar>
<div class="p-1 flex-1">
    <label class="input input-sm input-bordered flex items-center gap-2 mb-2">
        <span class="w-28 text-right">本地路径：</span>
        <input type="text" class="grow" bind:value={config.dataDir} placeholder="本地数据文件路径"/>
    </label>
    <label class="input input-sm input-bordered flex items-center gap-2 mb-2">
        <span class="w-28 text-right">所属区划：</span>
        <input type="text" class="grow" bind:value={config.ownerAreaId} placeholder="请配置所属区划"/>
    </label>
    <label class="input input-sm input-bordered flex items-center gap-2 mb-2">
        <span class="w-28 text-right">数据处理地：</span>
        <input type="text" class="grow" bind:value={config.areaId} placeholder="请配置数据处理地"/>
    </label>

    <label class="input input-sm input-bordered flex items-center gap-2 mb-2">
        <span class="w-28 text-right">当前年报告期：</span>
        <input type="text" class="grow" bind:value={config.areaId} placeholder="当前年报告期"/>
    </label>
    <label class="input input-sm input-bordered flex items-center gap-2 mb-2">
        <span class="w-28 text-right">当前季报告期：</span>
        <input type="text" class="grow" bind:value={config.areaId} placeholder="当前季报告期"/>
    </label>
    <label class="input input-sm input-bordered flex items-center gap-2 mb-2">
        <span class="w-28 text-right">当前月报告期：</span>
        <input type="text" class="grow" bind:value={config.areaId} placeholder="当前月报告期"/>
    </label>

    <label class="input input-sm input-bordered flex items-center gap-2 mb-2">
        <span class="w-28 text-right">天地图key：</span>
        <input type="text" class="grow" bind:value={config.tdtKey} placeholder="天地图key"/>
    </label>
</div>
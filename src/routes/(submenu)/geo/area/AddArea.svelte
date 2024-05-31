<script>

    import {Button,Icon,Modal,saveIcon} from "../../../../lib/youi";

    export let isOpen = false;

    export let record = {};

</script>

<Modal bind:isOpen>

    <div class="p-1 flex-1">
        {#if record.pid}
            <label class="input input-sm input-bordered flex items-center gap-2 mb-2">
                上级代码：
                <input type="text" class="grow" bind:value={record.pid} readonly/>
            </label>
        {/if}
        <label class="input input-sm input-bordered flex items-center gap-2 mb-2">
            <span  class="w-8">区划代码：</span>
            <input type="text" class="grow" bind:value={record.id} placeholder="请输入区划代码"/>
        </label>

        <label class="input input-sm input-bordered flex items-center gap-2 mb-2">
            <span class="w-8">简称：</span>
            <input type="text" class="grow" bind:value={record.text} placeholder="请输入行政区划简称"/>
        </label>
        <label class="input input-sm input-bordered flex items-center gap-2 mb-2">
            <span class="w-8">全称：</span>
            <input type="text" class="grow" bind:value={record.full_text} placeholder="请输入行政区划全称"/>
        </label>

        <div class="flex">
            <div class="flex-1"></div>
            <Button title="保存" class="btn-sm" tooltipPosition="left">
                <Icon data={saveIcon}/>
            </Button>
        </div>

    </div>

</Modal>
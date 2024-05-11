<script>

    export let metaObject = {};

    export let projectCode = '';

    $:if(metaObject.year){
        metaObject.code = `${projectCode}_${metaObject.year}`
    }

</script>

<label class="col-span-3 input input-sm input-bordered flex items-center gap-2">
    制度名称
    <input type="text" class="grow form-control" placeholder="请输入名称" bind:value={metaObject.text} />
</label>

<label class="col-span-3 input input-sm input-bordered flex items-center gap-2">
    制度全称
    <input type="text" class="grow form-control" placeholder="请输入制度全称" bind:value={metaObject.sub_title} />
</label>

<label class="col-span-3 input input-sm input-bordered flex items-center gap-2">
    制度年份
    <input type="text" class="grow form-control" placeholder="请输入年份" bind:value={metaObject.year} />
</label>

<label class="col-span-3 input input-sm input-bordered flex items-center gap-2">
    制度编码
    <input type="text" readonly class="grow form-control" value={metaObject.code}/>
</label>
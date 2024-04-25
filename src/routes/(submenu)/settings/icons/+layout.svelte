<script>

    import {onMount} from "svelte";
    import {iconStore} from "$lib/app-stores/base/iconStore";
    import {Icon,Tooltip,Toolbar,Button,plusIcon} from "$lib/youi";
    import {goto} from "$app/navigation";

    let pageIndex = 1;
    let pageSize = 100;

    const openAddIcon = () => {
        goto(`/settings/icons/dialog/add-icon`);
    }

    onMount(()=>{
        iconStore.findByPager({pageIndex,pageSize});
    });

</script>

<Toolbar>
    <Button title="新增图标" class="btn-sm btn-ghost" on:click={()=>openAddIcon()}>
        <Icon data={plusIcon}/>
    </Button>
</Toolbar>
<div class="grid grid-cols-12 gap-4">
    {#each $iconStore.records as icon}
        <div class="items-center text-center pt-2">
            <Tooltip title={icon.name}>
                <Icon scale="3" class="hover:cursor-pointer hover:opacity-60" data={JSON.parse(icon.content)}/>
            </Tooltip>
        </div>
    {/each}
</div>

<!-- dialog -->
<slot/>
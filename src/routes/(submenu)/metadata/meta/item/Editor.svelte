<script lang="ts">
    import {Offcanvas,Toolbar,Button,Icon,saveIcon,helpIcon} from "$lib/youi";
    import {metaItemStore} from "$lib/app-stores/dmp/metaItemStore";
    import {snakeCase} from "lodash";

    export let isOpen;

    export let title = '新增数据项';

    let name;

    let caption;

    const addMetaItem = async () => {
        let time = new Date().getTime();
        await metaItemStore.addMetaItem({
            name,
            column_name:snakeCase(name),
            caption,
            update_time:time,
            create_time:time
        });

        isOpen = false;
        clear();
    }

    const clear = () => {
        name = '';
        caption = '';
    }

</script>

<Offcanvas bind:isOpen contentClass="p-2">
    <Toolbar>
        <Button title="保存数据项" class="btn-sm btn-ghost" on:click={()=>addMetaItem()}>
            <Icon data={saveIcon}/>
        </Button>
        <div class="navbar-end text-right w-full font-bold">
            {title}
            <Icon class="w-4 h-4 hover:cursor-pointer" data={helpIcon}/>
        </div>
    </Toolbar>
    <div class="grid grid-cols-1 gap-x-6 gap-y-8 sm:grid-cols-6 pt-2">
        <label class="col-span-3 input input-sm input-bordered flex items-center gap-2">
            数据项名
            <input type="text" class="grow form-control" placeholder="请输入数据项名" bind:value={name} />
        </label>
        <label class="col-span-3 input input-sm input-bordered flex items-center gap-2">
            中文描述
            <input type="text" class="grow form-control" placeholder="请输入中文描述" bind:value={caption} />
        </label>
    </div>

</Offcanvas>
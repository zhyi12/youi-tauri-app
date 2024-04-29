<script lang="ts">
    import {Offcanvas,Toolbar,Button,Icon,saveIcon,helpIcon} from "$lib/youi";
    import {macroGroupStore} from "$lib/app-stores/datamacro/macroGroupStore";

    export let isOpen;

    export let title = '新增宏观分组';

    /** ------------  数据项属性  -------------*/

    export let id;
    export let text;
    export let group_type;
    export let desc;
    export let create_time;
    export let update_time;
    export let creator;
    export let modified_by;

    /**
     * 保存数据项
     */
    const saveMacroGroup = async () => {
        let time = new Date().getTime();
        let macroGroup = {

            id,
            text,
            group_type,
            desc,
            create_time,
            update_time,
            creator,
            modified_by,
        };
        if(id){
            Object.assign(macroGroup,{id,create_time});
            await macroGroupStore.updateMacroGroup(macroGroup);
        }else{
            await macroGroupStore.addMacroGroup(macroGroup);
        }

        isOpen = false;
        clear();
    }

    const clear = () => {

        id = '';
        text = '';
        group_type = '';
        desc = '';
        create_time = '';
        update_time = '';
        creator = '';
        modified_by = '';
    }

</script>

<Offcanvas bind:isOpen contentClass="p-2">
    <Toolbar>
        <Button title="保存宏观分组" class="btn-sm btn-ghost" on:click={()=>saveMacroGroup()}>
            <Icon data={saveIcon}/>
        </Button>
        <div class="navbar-end text-right w-full font-bold">
            {title}
            <Icon class="w-4 h-4 hover:cursor-pointer" data={helpIcon}/>
        </div>
    </Toolbar>
    <div class="grid grid-cols-1 gap-x-6 gap-y-8 sm:grid-cols-6 pt-2">

        <label class="col-span-3 input input-sm input-bordered flex items-center gap-2">
            分组ID
            <input type="text" class="grow form-control" placeholder="请输入分组ID" bind:value={id} />
        </label>
        <label class="col-span-3 input input-sm input-bordered flex items-center gap-2">
            名称
            <input type="text" class="grow form-control" placeholder="请输入名称" bind:value={text} />
        </label>
        <label class="col-span-3 input input-sm input-bordered flex items-center gap-2">
            分组类型
            <input type="text" class="grow form-control" placeholder="请输入分组类型" bind:value={group_type} />
        </label>
        <label class="col-span-3 input input-sm input-bordered flex items-center gap-2">
            分组说明
            <input type="text" class="grow form-control" placeholder="请输入分组说明" bind:value={desc} />
        </label>
        <label class="col-span-3 input input-sm input-bordered flex items-center gap-2">
            创建时间
            <input type="text" class="grow form-control" placeholder="请输入创建时间" bind:value={create_time} />
        </label>
        <label class="col-span-3 input input-sm input-bordered flex items-center gap-2">
            更新时间
            <input type="text" class="grow form-control" placeholder="请输入更新时间" bind:value={update_time} />
        </label>
        <label class="col-span-3 input input-sm input-bordered flex items-center gap-2">
            创建者
            <input type="text" class="grow form-control" placeholder="请输入创建者" bind:value={creator} />
        </label>
        <label class="col-span-3 input input-sm input-bordered flex items-center gap-2">
            修改者
            <input type="text" class="grow form-control" placeholder="请输入修改者" bind:value={modified_by} />
        </label>
    </div>
</Offcanvas>
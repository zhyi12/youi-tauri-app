<script lang="ts">
    import {afterNavigate} from "$app/navigation";
    import {Icon,Toolbar,Button,saveIcon} from "$lib/youi";
    import {Offcanvas} from "$lib/youi";
    import CodeMirror from "$lib/component/codemirror/CodeMirror.svelte";
    import {iconStore} from "$lib/app-stores/base/iconStore";
    import {svgToData} from "../../helper";

    const SCALES = [1,2,3];

    let isOpen = true;

    let name = '';
    let text = '';
    let value = '';
    let content = undefined;

    let props: CodeMirror["$$prop_def"] = {
        basic: true,
        useTab: true,
        editable: true,
        lineWrapping: false,
        readonly: false,
        tabSize: 2,
        placeholder: null,
        lang: null,
        theme: null,
        nodebounce: false
    };

    /**
     * 编辑器内容变化
     * @param detail
     */
    const handle_change = ({detail}) => {
        if(name && detail){
            content = svgToData(name,detail);
        }
    }

    /**
     *
     */
    const addIcon = async () => {
        if(!name || !content)return;
        const time = new Date().getTime();
        await iconStore.addIcon({
            name,
            text,
            content:JSON.stringify(content),
            create_time:time,
            update_time:time
        });
        isOpen = false;
        text = '';
        name = '';
        value = '';
        content = undefined;
    }

    afterNavigate(()=>{
        if(!isOpen){
            isOpen = true;
        }
    })

</script>

<Offcanvas bind:isOpen contentClass="p-2">
    <Toolbar>
        <Button title="新增图标" class="btn-sm btn-ghost" on:click={()=>addIcon()}>
            <Icon data={saveIcon}/>
        </Button>
    </Toolbar>
    <div class="grid grid-cols-1 gap-x-6 gap-y-8 sm:grid-cols-6">
        <label class="col-span-3 input input-sm input-bordered flex items-center gap-2">
            图标名
            <input type="text" class="grow form-control" placeholder="请输入图标名" bind:value={name} />
        </label>
        <label class="col-span-3 input input-sm input-bordered flex items-center gap-2">
            中文名
            <input type="text" class="grow form-control" placeholder="请输入中文名" bind:value={text} />
        </label>
    </div>


    <div class="mt-2">
        <div class="p-1 h-8 bg-base-200 text-sm">
            图标SVG
        </div>
        <CodeMirror bind:value class="editor" {...props} on:change={handle_change}/>
    </div>
    <div>
        {#each SCALES as scale}
            <Icon {scale} data={content}/>
        {/each}
    </div>
</Offcanvas>
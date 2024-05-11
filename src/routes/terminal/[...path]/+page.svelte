<script lang="ts">

    import {afterNavigate} from "$app/navigation";
    import {Toolbar,Button,Icon,saveIcon,VDataTable,Dropdown,DropdownToggle,DropdownMenu,DropdownItem} from "$lib/youi";
    import CodeMirror from "$lib/component/codemirror/CodeMirror.svelte";
    import {updateDslScriptContent} from "$lib/app-services/base/dslScriptServices";
    import {configStore} from "$lib/app-stores/base/configStore";
    import {dsl,dslHover} from "$lib/component/codemirror/dsl";
    import {execute} from "$lib/tauri/dsl";
    import {playCircleOIcon} from "$lib/app-icons";

    const PAGESIZES = [50,100,150,200,500];

    export let data;

    let pageSize = 100;

    /**
     * 原始数据
     */
    let orgRecord = undefined;
    /**
     * 脚本
     */
    let dslContent = '';
    let orgDslContent = '';
    let columns = [];
    let records = [];

    let time = 0;

    $:dirty = dslContent!==orgDslContent;

    const doSave = async () => {
        await updateDslScriptContent(data.id,dslContent);
        orgDslContent = dslContent;
    }

    const doExecute = async () => {
        const start = new Date().getTime();
        const result = await execute(dslContent,[{
            name:'dataDir',
            dataType:'string',
            value:$configStore.dataDir
        },{
            name:'dbConnect',
            dataType: 'string',
            value:$configStore.dbConnect,
        }]);

        time = new Date().getTime() - start;

        if(result.length){
            columns = Object.keys(result[0]).map(key=>(
                {
                    dataType:typeof result[0][key],
                    accessorKey: key,
                    header:key
                }
            ));
            records = result;
        }
    }

    afterNavigate(({from})=>{
        dslContent = data.dslContent;
        orgDslContent = data.dslContent;
        records = [];
        time = 0;
        // if(from && dslContent){
        //     doExecute();
        // }
    });

</script>

<Toolbar>
    <Button  disabled={!dirty} title="保存" on:click={()=>doSave()}>
        <Icon data={saveIcon}/>
    </Button>

    <Button disabled={!dslContent} title="执行" on:click={()=>doExecute()}>
        <Icon class="text-green-700"  data={playCircleOIcon}/>
    </Button>

    <div class="flex-1">
        {#if time}
        执行耗时：{time}毫秒.
        {/if}
    </div>

    <Dropdown>
        <DropdownToggle>
            预览前{pageSize}条数据.
        </DropdownToggle>
        <DropdownMenu>
            {#each PAGESIZES as p}
                <DropdownItem active={p === pageSize} on:mousedown={()=>{
                    pageSize = p
                }}>
                    {p}
                </DropdownItem>
            {/each}
        </DropdownMenu>
    </Dropdown>
</Toolbar>
<div class="p-1 flex-1 flex flex-col h-0">
    <div class="max-h-64 overflow-auto border-b mb-1">
        <CodeMirror bind:value={dslContent}
                    extensions={[dslHover]}
                    lang={dsl()}/>
    </div>
    <div class="flex-1 flex h-0 overflow-hidden">
        <VDataTable {records} {columns} class="flex-1"/>
    </div>
</div>
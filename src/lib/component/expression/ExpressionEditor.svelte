<script>
    import CodeMirror from "$lib/component/codemirror/CodeMirror.svelte";
    import {FieldInput, Tree} from "../../youi";
    import {createEventDispatcher} from "svelte";
    import {createColPlaceholder} from "./extensions";
    import {insertIntoExpression, OPS} from "./helper";

    const dispatch = createEventDispatcher();

    export let expression = '';

    export let text = '';

    export let dataType = 'text';

    export let funcItems = [];

    export let columns = [];

    export let icons = _=>undefined;

    export let view;

    const colPlaceholder = createColPlaceholder((name)=>{
        return nameMap.get(name)||name;
    })

    const handle_editor_ready = ({detail}) => {
        view = detail;
    }

    const handle_func_dblclick = ({detail}) => {
        insertIntoExpression(view,detail.datas.apply||'')
    }

    $:nameMap = columns.reduce((m,v)=>m.set(v.name,v.text),new Map());

</script>

<div class="flex p-1 pb-0">
    <div class="flex-1 pr-2">
        <FieldInput placeholder="请输入计算列名." bind:value={text}/>
    </div>
    <div class="min-w-24">
        <FieldInput placeholder="数据类型." bind:value={dataType}/>
    </div>
</div>
<div class="h-8 border flex bg-gray-50">
    {#each OPS as op}
        <div class="flex justify-center items-center w-6 cursor-pointer bg-base-100 mr-0.5 hover:bg-blue-50">
            {op.symbol}
        </div>
    {/each}
</div>
<div class="h-36 overflow-auto expression-content w-full">
    <CodeMirror on:ready={handle_editor_ready} lineWrapping={true} extensions={[colPlaceholder]} bind:value={expression}/>
</div>
<div class="flex-1 border-t flex h-0 bg-gray-50">
    <Tree {icons} class="w-48 border-r h-full overflow-y-auto"
          on:dblclick={handle_func_dblclick}
          on:load children={funcItems}/>
    <div class="class=flex-1"></div>
</div>

<style>
    .expression-content{
        font-size: 1rem;
    }
</style>

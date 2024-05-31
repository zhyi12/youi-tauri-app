<script>

    import {onMount} from "svelte";
    import PageNav from "../../../../lib/app-page/PageNav.svelte";
    import {Button,Icon} from "$lib/youi";
    import {folderOpenIcon} from "$lib/app-icons";
    import {createModel} from "./store";

    import Designer from './_Designer.svelte';

    export let data;

    let scale = 80;

    /**
     * 模型存储
     */
    let model = createModel();

    const handle_designer_action = ({detail}) => {
        let {method,args} = detail;
        if(method && actions[method]){
            actions[method].apply(model,args);
        }
    }

    const actions = {
        lineMove:(value,index)=>{
            model.updateLine(index,value);
        },

        addLine:(direction,value)=>{
            model.addLine(direction,value);
        },

        removeLine:(index)=>{
            model.removeLine(index);
        }
    };

    onMount(()=>{
        model.setModel({
            page:{
                w:1920,
                h:1080,
                lines:[
                    {
                        value:0,
                        direction:'v'
                    },{
                        value:200,
                        direction:'v'
                    },{
                        value:1920,
                        direction:'v'
                    },{
                        value:0,
                        direction:'h'
                    },{
                        value:200,
                        direction:'h'
                    },{
                        value:1080,
                        direction:'h'
                    }
                ]
            }
        })
    });

</script>

<div class="relative p-0 h-full w-full">
    <div class="p-0 h-full w-full">
        <div class="de-framework overflow-hidden h-full w-full flex flex-col">
            <div class="editor-header h-12 shadow items-center flex shrink-0 pl-2">
                <div >
                    <Button title="打开数据看板" class="btn-ghost btn-sm">
                        <Icon data={folderOpenIcon}/>打开
                    </Button>
                </div>
                <PageNav navPaths={data.pathMenuPaths.slice(1)}/>
                <div class="flex-1"></div>
            </div>
            <div class="datav-editor-body flex-1 h-full w-full flex overflow-hidden">

                <div class="datav-custom-app-wrapper h-full w-full overflow-hidden">

                    <div class="datav-sketch-app relative flex flex-row h-full w-full">
                        <div class="w-16 h-dvh min-w-16 border-r bg-gray-100 relative overflow-hidden">

                        </div>
                        <div class="datav-editor-content-center flex-1 flex flex-row">
                            <div class="datav-editor-panel-wrapper w-52 min-w-52 border-r h-full bg-gray-100 relative"></div>
                            <div class="datav-editor-content-inside h-full w-full flex flex-1 flex-col overflow-hidden">
                                <div class="datav-editor-sketch-area flex-1 h-full w-full overflow-hidden">
                                    <div class="datav-sketch h-full w-full overflow-hidden relative">
                                        <Designer bind:scale model={$model} on:action={handle_designer_action}/>
                                    </div>
                                </div>
                                <div>
                                    <div class="absolute z-[12] pl-2 flex flex-row bottom-5 ml-10 rounded-md bg-white h-12 border w-96 pointer-events-auto">
                                        <div class="pt-4 w-64">
                                            <input type="range" min="40" max="300" bind:value={scale}
                                                   class="range range-xs"/>
                                        </div>
                                        <div class="pt-2">
                                            <label class="input input-sm flex items-center gap-2">
                                                <input on:keydown={e=>{
                                                    if(e.keyCode === 13){
                                                        scale = Math.min(Math.max(40,e.target.value),300);
                                                        e.preventDefault();
                                                        e.stopPropagation();
                                                    }
                                                }} size="3" type="text" class="grow" placeholder="" value={scale}/>%
                                            </label>
                                        </div>

                                        <div class="flex-1">

                                        </div>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>

        </div>
    </div>
</div>



<!-- 弹窗 -->
<slot/>
<!--
    报表组件
-->
<script>

    import CrossTableWidget from "./widget/CrossTableWidget.svelte";
    import TableWidget from "./widget/TableWidget.svelte";
    import {createEventDispatcher, onMount} from "svelte";
    import {DEFAULT_COL_WIDTH} from "./helper";

    const dispatch = createEventDispatcher();
    /**
     * 报表组件
     */
    export let widgets = [];
    /**
     * 是否启用编辑
     */
    export let editable = false;
    /**
     * 是否启用纸张打印排版
     */
    export let showPagePrint = false;

    /**
     * 当前选中的组件
     */
    export let activeWidgetId = '';

    let container = undefined;
    let dpi = 0;
    let dpiDom = undefined;

    const a4WidthInInches = 8.5;
    const a4HeightInInches = 11;
    let a4Width = 0;
    let a4Height = 0;

    let xScrollHelper = undefined;
    let yScrollHelper = undefined;
    let scrollLeft = 0;
    let scrollTop = 0;

    let tableHeight;
    let contentHeight = 0;

    $:tableWidth = widgets.reduce((maxWidth,widget)=>{
        const width = (widget.colModels||[]).map(col=>col.width||DEFAULT_COL_WIDTH).reduce((s,w)=>s+w,0);
        return Math.max(maxWidth,width)
    },0);

    /**
     *
     * @param widget
     * @param e
     */
    const selectWidget = (widget,e) => {
        activeWidgetId = widget.id;
        if(e.detail.scrollViewer){
            scrollToViewer(e.detail.scrollViewer);
        }
    };

    const scrollToViewer = ({x,contentWidth,tableWidth}) => {
        if(x>contentWidth){
            scrollLeft = Math.min(x+20,tableWidth-contentWidth);
        }
        if(x<scrollLeft){
            scrollLeft = x;
        }

        xScrollHelper.scrollLeft = scrollLeft;
    }

    /**
     * 横向滚动
     */
    const handle_x_scroll = (e) => scrollLeft = e.target.scrollLeft;

    /**
     * 纵向滚动
     */
    const handle_y_scroll = (e) => scrollTop = e.target.scrollTop;

    /**
     * 滚轮
     */
    const handle_wheel = (event) => {
        if(yScrollHelper && event.deltaY){
            scrollTop = Math.max(Math.min(tableHeight - contentHeight+300,scrollTop+event.deltaY),0);
            yScrollHelper.scrollTop = scrollTop;
        }
    }

    onMount(()=>{
        if(dpiDom){
            let size = dpiDom.offsetWidth;
            a4Width = size * a4WidthInInches;
            a4Height = size * a4HeightInInches;
        }
        contentHeight = container.offsetHeight;
    })

</script>

<div class="youi-report relative flex-1 flex flex-col overflow-hidden h-0" bind:this={container}
     on:wheel={handle_wheel}
     style="margin-left: calc(6px + env(safe-area-inset-left));margin-right: calc(6px + env(safe-area-inset-right));">
    {#if showPagePrint && a4Width === 0}
        <div bind:this={dpiDom} style="width:1in;height:1in;position:absolute;left:0;top:0;z-index:10000;visibility:hidden;">

        </div>
    {:else}
        <div class="relative overflow-hidden pb-4 flex-1 flex flex-col h-0" style:width={showPagePrint?`${a4Width}px`:''}>
            <div style:width={showPagePrint?`${a4Width}px`:''} class="flex flex-col flex-1">
                {#each widgets as widget}
                    {#if widget.name === 'Table'}
                        <TableWidget
                                {editable}
                                on:col-resize={e=>dispatch('col-resize',{...e.detail,widgetId:widget.id})}
                                on:selection-stop={(e)=>selectWidget(widget,e)} active={activeWidgetId === widget.id}
                                on:edited={e=>dispatch('edited',{...e.detail,widgetId:widget.id})}
                                {...widget}/>
                    {:else if widget.name === 'CrossTable'}
                        <div class="flex-1 overflow-hidden">
                            <CrossTableWidget
                                    {contentHeight} bind:tableHeight
                                    {scrollTop}
                                    {scrollLeft} {editable} showYScroll={false}
                                    on:col-resize={e=>dispatch('col-resize',{...e.detail,widgetId:widget.id})}
                                    on:edited={e=>dispatch('edited',{...e.detail,widgetId:widget.id})}
                                    on:table-height={()=>console.log('hhhh')}
                                    on:selection-stop={(e)=>selectWidget(widget,e)}  active={activeWidgetId === widget.id} {...widget}/>
                        </div>
                    {/if}
                {/each}
            </div>
        </div>
    {/if}
    <div class="absolute bottom-0 left-0 h-4 overflow-x-auto" bind:this={xScrollHelper}
         on:scroll={handle_x_scroll}
         style:width={showPagePrint?`${a4Width}px`:'100%'}>
        <div class="h-0.5" style:width={`${tableWidth}px`}></div>
    </div>

    <div class="absolute top-0 right-0 overflow-y-auto h-full w-4 bg-gray-100"  bind:this={yScrollHelper}
         on:scroll={handle_y_scroll}
    >
        <div class="w-0.5" style:height={`${tableHeight+Math.min(300,contentHeight)}px`}></div>
    </div>
</div>




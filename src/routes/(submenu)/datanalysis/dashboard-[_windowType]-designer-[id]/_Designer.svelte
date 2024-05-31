<script lang="ts">

    import {createEventDispatcher, onMount, tick} from "svelte";
    import {Box,Ruler,Icon,mouse} from "$lib/youi";
    import {lineHideIcon,lineShowIcon} from "$lib/app-icons";
    import type {DashboardModel} from "./dashboardModel";
    import {throttle} from 'lodash';

    const dispatch = createEventDispatcher();

    export let model:DashboardModel;

    let container = undefined;
    let parentOffset = {left:0,top:0};

    export let scale = 100;

    let screenWidth;
    let screenHeight;
    let contentWidth;
    let contentHeight;
    // 布局宽度
    let layoutWidth = 3000;
    // 布局高度
    let layoutHeight = 1000;

    let pageWidth = 1920;
    let pageHeight = 1080;
    let pageLeft = 0;
    let pageTop = 0;

    let xStartRuler = 0;
    let xMaxRuler = 0;
    let yStartRuler = 0;
    let yMaxRuler = 0;
    // 是否显示辅助线
    let showLine = true;

    let moveX = undefined;
    let movingLineIndex = -1;
    let movingLineValue = 0;

    let dragType;
    let dragHelper = undefined;
    let startPosition = {x:0,y:0};

    let selecting = false;
    let selectingBox = {x:0,y:0,width:0,height:0};

    let scrollTop = 0;
    let scrollLeft = 0;
    let translateX =0;
    let translateY =0;

    const mouseStart = (e) => {
        dragType = parseDragType(e.target);
        if ('line-v' === dragType || 'line-h' === dragType){
            // 刻度辅助线
            dragHelper = e.target;
            movingLineIndex = parseInt(dragHelper.getAttribute("data-index"));
        }else if('container' === dragType){
            selecting = true;
            startPosition = {
                x:e.pageX - parentOffset.left,
                y:e.pageY - parentOffset.top
            };
        }
    }

    const mouseDrag = (e) => {
        if(selecting){
            const position = {
                x:e.pageX - parentOffset.left,
                y:e.pageY - parentOffset.top
            };
            let x = Math.min(startPosition.x,position.x);
            let y = Math.min(startPosition.y,position.y);
            selectingBox = {
                x,y,width:Math.abs(startPosition.x-position.x),height:Math.abs(startPosition.y-position.y)
            }
        }else if(dragHelper){
            //
            if ('line-v' === dragType){
                const left = e.pageX - parentOffset.left;
                dragHelper.style.left = `${left + scrollLeft - 20}px`;

                movingLineValue = Math.ceil((left+(xStartRuler-translateX))/scaleRatio);
            }else if ('line-h' === dragType){
                const top = e.pageY - parentOffset.top;
                dragHelper.style.top = `${top-translateY-20}px`;
                movingLineValue = Math.ceil((top+(yStartRuler-translateY))/scaleRatio);
            }
        }
    }

    const mouseStop = (e) => {
        if(selecting === true){
            selecting = false;
            selectingBox = {x:0,y:0,width:0,height:0};
        }else if ('line-v' === dragType || 'line-h' === dragType){
            dispatch('action',{
                method:'lineMove',
                args:[movingLineValue,movingLineIndex]
            });
        }
        moveX = undefined;
        dragType = undefined;
        startPosition = {x:0,y:0};
        dragHelper = undefined;
    }

    const parseDragType = (dom) => {
        if(dom === container || dom.classList.contains('layout-scrollable')){
            return 'container';
        }else if (dom.classList.contains('line-direction-v')){
            return 'line-v';
        }else if (dom.classList.contains('line-direction-h')){
            return 'line-h';
        }
    }

    const handle_scroll = (event) => {
        scrollTop = container.scrollTop;
        scrollLeft = container.scrollLeft;
    }

    // const handle_wheel = (event) => {
    //     if(event.shiftKey){
    //         //横向滚动
    //         throttle(()=>{
    //             //if((event.deltaX>0 && scrollLeft > xStartRuler + 100) || (event.deltaX<0 && scrollLeft < xMaxRuler-contentWidth+20)){
    //                 scrollLeft = Math.ceil(scrollLeft-event.deltaX/3);
    //                 container.srcollLeft = scrollLeft;
    //             //}
    //         },100, { 'trailing': false })();
    //     }
    // }

    const handle_resize = (_e,autoScaleRatio) => {
        throttle(()=>{
            _resize(autoScaleRatio);
        },100, { 'trailing': false })();
    }

    const _resize=(autoScaleRatio)=>{
        if(!container)return;

        let ratio = scaleRatio;

        contentWidth = container.parentElement.offsetWidth;
        contentHeight = container.offsetHeight;

        container.style.width=`${contentWidth}px`;
        if(model && model.page){
            pageWidth = model.page.w;
            pageHeight = model.page.h;

            layoutWidth = (contentWidth-100)*2+model.page.w;
            layoutHeight = (contentHeight-100)*2+model.page.h;

            pageLeft = -(pageWidth-layoutWidth)/2;
            pageTop = -(pageHeight-layoutHeight)/2;

            //
            if(autoScaleRatio){
                scale = Math.ceil(Math.min(contentWidth/(pageWidth+200),contentHeight/(pageHeight+200))*100);
                ratio = scale/100;
            }

            if(scale!==100){
                //处理layout缩放
                layoutWidth = layoutWidth-(1-ratio)*model.page.w;
                layoutHeight = layoutHeight-(1-ratio)*model.page.h;
            }

            //水平线刻度计算，起始刻度=（页宽度 - 布局容器宽度）/2
            xStartRuler = (pageWidth*ratio-layoutWidth)/2;
            //水平线刻度计算，结束刻度=（布局容器宽度-页宽度）/2+页宽度
            xMaxRuler = (layoutWidth-pageWidth*ratio)/2+pageWidth*ratio;

            yStartRuler = (pageHeight*ratio-layoutHeight)/2;
            yMaxRuler = layoutHeight+yStartRuler;
        }

    }

    onMount(()=>{
        screenWidth = screen.width;
        screenHeight = screen.height;
        let op = container;
        while(op){
            parentOffset = {
                left:parentOffset.left + op.offsetLeft,
                top:parentOffset.top+op.offsetTop
            }
            op = op.offsetParent;
        }

        handle_resize({},true);
        // scroll to 0刻度偏移50像素
        window.setTimeout(()=>{
            container.scrollTo(-50-xStartRuler,-50-yStartRuler)
        },0);
    })
    // 比例
    $:scaleRatio = scale/100;

    $:lines = (showLine && model?.page?.lines)?model?.page?.lines.map((line,index)=>{
        line.index = index;
        line.id = index;
        return line;
    }):[];

    $:hLines = lines.filter(line=>line.direction === 'h');
    $:vLines = lines.filter(line=>line.direction === 'v');

    $:translateX = 0-scrollLeft;

    $:translateY = 0-scrollTop;
    /**
     * 刻度缩放后的X偏移量
     */
    $:offsetX = scaleRatio===1?0:pageWidth*(1-scaleRatio);
    /**
     * 刻度缩放后的Y偏移量
     */
    $:offsetY = scaleRatio===1?0:pageHeight*(1-scaleRatio);//yStartRuler - translateY - pageHeight*(scale/100);

    $:scale && handle_resize();

</script>

<svelte:window on:resize={handle_resize}/>

<div class="datav-absolute-canvas-wp relative h-full w-0 overflow-auto" bind:this={container}
     on:scroll={handle_scroll}
     use:mouse={{mouseStart,mouseDrag,mouseStop,delay:50}}>
    <!--    刻度    -->
    <div class="datav-absolute-ruler fixed z-[10] overflow-hidden pointer-events-none" style={`width: ${contentWidth}px;height:${contentHeight}px;`}>
        <!-- 横向刻度   -->
        <Ruler {scale} min={xStartRuler+20} max={xMaxRuler} translate={translateX}
               lines={vLines} step={scale/10} offset={offsetX}
               {movingLineIndex} {movingLineValue}
               on:ruler={({detail})=>dispatch('action',{
                    method:'addLine',
                    args:['v',detail.value]
                })}

               on:line-dblclick={({detail})=>{
                   dispatch('action',{
                        method:'removeLine',
                        args:[detail.index]
                   })
               }}

               class="absolute cursor-ew-resize pointer-events-auto left-5 bg-neutral-100 border-zinc-500"/>
        <!-- 纵向刻度   -->
        <Ruler {scale} min={yStartRuler+20} max={yMaxRuler} translate={translateY} vertical={true}
               lines={hLines} step={scale/10} offset={offsetY}
               on:ruler={({detail})=>dispatch('action',{
                    method:'addLine',
                    args:['h',detail.value]
                })}

               on:line-dblclick={({detail})=>{
                   dispatch('action',{
                        method:'removeLine',
                        args:[detail.index]
                   })
               }}
               {movingLineIndex} {movingLineValue}
               class="absolute cursor-ns-resize pointer-events-auto bg-neutral-100 border-zinc-500 left-0 top-0 origin-[0px_100%_0px]"/>
        <!-- 刻度交叉   -->
        <div class="corner absolute w-5 h-5 pointer-events-auto left-0 top-0 bg-gray-100 cursor-pointer"
             on:click={()=>showLine=!showLine}>
            <Icon data={showLine?lineHideIcon:lineShowIcon}/>
        </div>

        {#if selecting}
            <Box {...selectingBox}/>
        {/if}
    </div>

    <div class="layout-scrollable relative m-0 p-0" style={`width: ${layoutWidth}px; height: ${layoutHeight}px;`}>
        <div class="datav-absolute-layout-position absolute" style={`left: ${pageLeft}px; top: ${pageTop}px;`}>
            <div class="datav-common-hoc relative">
                <div class="datav-com-wrapper" style={`transform: scale(${scale/100}) translate3d(0px, 0px, 0px)`}>
                    <div class="datav-absolute-page-wp point-events-none" style={`width: ${pageWidth}px; height: ${pageHeight}px;`}>
                        <div class="datav-absolute-page bg-gray-100" style={`width: ${pageWidth}px; height: ${pageHeight}px;`}>

                        </div>
                    </div>
                </div>
            </div>
        </div>
    </div>
</div>

<style>
    .datav-absolute-layout-position{
        width: 0;
        height: 0;
    }
    .datav-common-hoc{
        display: block;
        opacity: 1;
        position: relative;
        transform: rotate(0deg) scaleX(1) scaleY(1) rotateZ(360deg);
    }

    .datav-com-wrapper{
        pointer-events: none;
        user-select: none;
        transform-origin: left top;
    }

    .datav-absolute-page-wp{

    }

    .datav-absolute-page{
        width: 100%;
        height: 100%;
        overflow: hidden;
    }

    .dashboard-designer-content{
        background: url("/images/point.png");
    }
</style>
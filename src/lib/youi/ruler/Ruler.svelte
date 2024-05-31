<!-- 刻度 -->

<script lang="ts">
    import Konva from 'konva';

    import {createEventDispatcher, onMount} from "svelte";
    import classnames from "../util/classname";
    import {throttle} from 'lodash';

    // 刻度颜色
    const RULER_COLOR = '#a3a3a3';

    const dispatch = createEventDispatcher();

    let className = '';
    export { className as class };

    export let min = 0;

    export let scale = 100;

    export let max = 0;

    export let lines = [];

    export let movingLineIndex = 0;

    export let movingLineValue = 0;

    export let step = 10;
    /**
     * 偏移
     */
    export let offset = 0;
    /**
     * 是否垂直刻度
     */
    export let vertical = false;

    /**
     *
     */
    export let translate = 0;
    
    let height = 20;

    let hoverRuler = undefined;

    let container:HTMLElement;
    let stage;
    let layer:Konva.Layer;

    const _drawRuler = (min,max,scale) => {
        if(layer){
            let width = (max-min);
            layer.removeChildren();
            stage.width(width);

            if(min<0){
                let negativeSteps = (0-min)/step;
                for(let i=0;i<negativeSteps;i++){
                    // 刻度
                    const ruler = i*-step;
                    // x坐标
                    const x = Math.abs(i*step+min);
                    drawLine(ruler,x)
                }
            }
            let steps = (max+offset*2)/step;
            for(let i=0;i<steps;i++){
                // 刻度
                const ruler = i*step;
                // x坐标
                const x = Math.abs(min)+ruler;
                drawLine(ruler,x)
            }
        }

    }

    const drawLine = (ruler,x)=>{
        let isZero = ruler === 0;
        let isTen = ruler%scale === 0;

        let line = new Konva.Line({
            x:x,
            y:0,
            stroke:isZero?'#d7a872':RULER_COLOR,
            strokeWidth:isZero?2:1,
            points:[0,isTen?5:15,0,20],
        });
        if(isTen){
            let text = new Konva.Text({
                x:x+5,
                y:3,
                fontSize:10,
                fill:RULER_COLOR,
                strokeWidth:1,
                text:Math.ceil(ruler*(100/scale))
            });
            layer.add(text);
        }
        layer.add(line);
    }

    const drawRuler = (min,max,scale) => {
        throttle(()=>{
            _drawRuler(min,max,scale);
        },100, { 'trailing': false })();
    }

    const handle_mousemove = (e) => {
        throttle(()=>{
            hoverRuler = Math.ceil((e.offsetX+min)/scale*100);
        },100, { 'trailing': false })();
    }

    /**
     *
     * @param e
     */
    const handle_click = (e) => {
        const ruler = Math.ceil((e.offsetX+min)/scale*100);
        dispatch('ruler',{value:ruler});
    }

    onMount(()=>{
        stage = new Konva.Stage({
            container,width:(max-min),height
        });
        layer = new Konva.Layer();
        stage.add(layer);
    });

    const calculateScaledLines = (lines,scale)=>{
        return lines.map(line=>{
            let position = (line.value - min) - line.value*(100-scale)/100;
            return {...line,positionStyle:`${line.direction==='v'?'left':'top'}:${position}px;`};
        });
    }

    $:drawRuler(min,max,scale);

    $: classes = classnames('youi-ruler',className);

    $:transformStyle = `${vertical?'rotate(90deg) ':''}translateX(${translate}px);`;
    /**
     * 缩放后的线条
     */
    $:scaledLines = calculateScaledLines(lines,scale);

</script>

<div class={classes} style={`transform:${transformStyle}`} class:v-ruler={vertical}>
    <div bind:this={container}
         on:mousemove={handle_mousemove}
         on:click={handle_click}
         on:mouseleave={()=>hoverRuler=undefined}
    >

    </div>
    <div class="lines-wrapper">
        {#each scaledLines as line}
            <div data-index={line.id} style={line.positionStyle}
                 on:dblclick={()=>dispatch('line-dblclick',{...line})}
                 class={`ruler-line line-direction-${line.direction}`}>
                <div class="line-action inline-block left-0 top-0 pointer-events-none">
                    <span class="line-value bg-pink-400 rounded-md p-0.5">
                        {movingLineIndex === line.id? movingLineValue:line.value}
                    </span>
                </div>
            </div>
        {/each}

        {#if hoverRuler !== undefined}
            <div class="absolute line-hover text-xs top-3 bg-blue-200 rounded-md p-0.5 z-[2]"
                 style={`${vertical?'top':'left'}:${(hoverRuler - min) - hoverRuler*(100-scale)/100}px;`}>
                {hoverRuler}
            </div>
        {/if}
    </div>
</div>


<style lang="scss">

  .v-ruler{
    .lines-wrapper{
      transform: rotate(-90deg);
      transform-origin: 0px 100% 0px;
    }

    .line-action{
      display: inline-block;
      left: 0px;
      top: 10px;
      transform: rotate(90deg);
      position: absolute;
    }

    .line-hover{
      transform:rotate(90deg) translateX(20px);
    }
  }

  .ruler-line {

    position: absolute;
    font-size: 0.64rem;
    pointer-events: all;

    &.line-direction-h{
      width: 100vw;
      height: 0;
      border: none;
      padding: 0;
      cursor: ns-resize;
      &:before{
        content: '';
        position: absolute;
        width: 100%;
        height: 7px;
        background-color: transparent;
        left: 0;
        top: 0;
        transform: translateY(-50%);
        pointer-events: all;
      }

      &:after{
        content: '';
        height: 1px;
        width: 100%;
        background: #EB33CB;
        position: absolute;
        left: 0;
        top: 0;
        transform: translateY(-75%) scaleY(0.5);
        transform-origin: center center;
      }
    }

    &.line-direction-v{
      height: 100vh;
      top: 0;
      width: 0;
      border: none;
      padding: 0;
      cursor: ew-resize;
      pointer-events: all;
      &:before{
        content: '';
        position: absolute;
        width: 7px;
        height: 100%;
        left: 0;
        top: 0;
        background: transparent;
        transform: translateX(-50%);
        pointer-events: all;
        cursor: ew-resize;
        pointer-events: all;
      }

      &:after{
        content: '';
        width: 1px;
        height: 100%;
        background: #EB33CB;
        position: absolute;
        left: 0;
        top: 0;
        transform: translateX(-75%) scaleX(0.5);
        transform-origin: center center;
      }
    }
  }
</style>


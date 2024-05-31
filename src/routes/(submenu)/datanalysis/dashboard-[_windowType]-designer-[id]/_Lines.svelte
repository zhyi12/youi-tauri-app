
<!--<script lang="ts">-->

<!--    import type {Line} from "./dashboardModel";-->

<!--    export let scale = 100;-->

<!--    export let lines:Line[] = [];-->

<!--    export let scrollLeft;-->

<!--    export let scrollTop;-->

<!--    export let movingIndex = -1;-->

<!--    export let movingValue = 0;-->

<!--    export let zeroOffsetX = 0;-->

<!--    export let zeroOffsetY = 0;-->

<!--    $:scaledLines = (scale>0 || scrollTop || scrollLeft)?lines.map(line=>{-->
<!--        line.scaleValue = computePosition(line);-->
<!--        return line;-->
<!--    }):lines;-->

<!--    const computePosition = (line) => {-->
<!--        let value = line.value;-->
<!--        if(line.direction === 'v'){-->
<!--            value = value - scrollLeft;-->
<!--        }else{-->
<!--            value = value - scrollTop;-->
<!--        }-->
<!--        console.log(value)-->
<!--        return value*(scale/100);-->
<!--    }-->
<!--</script>-->

<!--{#each scaledLines as line,index}-->
<!--    <div data-index={index}  style={`${line.direction==='v'?'left':'top'}:${line.scaleValue}px;`}-->
<!--         class={`ruler-line line-direction-${line.direction}`}>-->
<!--        <div class="line-action inline-block left-0 top-0">-->
<!--            <span class="line-value bg-pink-400 rounded-md p-0.5">-->
<!--                {movingIndex === index?movingValue:line.value}-->
<!--            </span>-->
<!--        </div>-->
<!--    </div>-->
<!--{/each}-->

<!--<style lang="scss">-->
<!--  .ruler-line {-->

<!--    position: absolute;-->
<!--    font-size: 0.64rem;-->
<!--    pointer-events: all;-->

<!--    &.line-direction-h{-->
<!--      width: 100vw;-->
<!--      height: 0;-->
<!--      border: none;-->
<!--      padding: 0;-->
<!--      cursor: ns-resize;-->
<!--      &:before{-->
<!--        content: '';-->
<!--        position: absolute;-->
<!--        width: 100%;-->
<!--        height: 7px;-->
<!--        background-color: transparent;-->
<!--        left: 0;-->
<!--        top: 0;-->
<!--        transform: translateY(-50%);-->
<!--        pointer-events: all;-->
<!--      }-->

<!--      &:after{-->
<!--        content: '';-->
<!--        height: 1px;-->
<!--        width: 100%;-->
<!--        background: #EB33CB;-->
<!--        position: absolute;-->
<!--        left: 0;-->
<!--        top: 0;-->
<!--        transform: translateY(-75%) scaleY(0.5);-->
<!--        transform-origin: center center;-->
<!--      }-->
<!--    }-->

<!--    &.line-direction-v{-->
<!--      height: 100vh;-->
<!--      top: 0;-->
<!--      width: 0;-->
<!--      border: none;-->
<!--      padding: 0;-->
<!--      cursor: ew-resize;-->
<!--      pointer-events: all;-->
<!--      &:before{-->
<!--        content: '';-->
<!--        position: absolute;-->
<!--        width: 7px;-->
<!--        height: 100%;-->
<!--        left: 0;-->
<!--        top: 0;-->
<!--        background: transparent;-->
<!--        transform: translateX(-50%);-->
<!--        pointer-events: all;-->
<!--        cursor: ew-resize;-->
<!--        pointer-events: all;-->
<!--      }-->

<!--      &:after{-->
<!--        content: '';-->
<!--        width: 1px;-->
<!--        height: 100%;-->
<!--        background: #EB33CB;-->
<!--        position: absolute;-->
<!--        left: 0;-->
<!--        top: 0;-->
<!--        transform: translateX(-75%) scaleX(0.5);-->
<!--        transform-origin: center center;-->
<!--      }-->
<!--    }-->
<!--  }-->
<!--</style>-->
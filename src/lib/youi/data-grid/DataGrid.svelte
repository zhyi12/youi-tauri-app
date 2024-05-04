<script lang="ts">

    import {createEventDispatcher, onMount,tick} from "svelte";
    import Konva from 'konva';
    import {throttle} from 'lodash';
    import {mouse} from '../mouse/mouse';

    import type {ItemSizer,CellPosition,CellData,Area} from "./DataGrid.d";
    import {
        buildFrozenColumnCells,
        buildFrozenRowCells, buildIntersectionCells,
        buildShowCells,
        calculateColumnWidths,
        calculateRange,
        calculateRowHeights, cellIdentifier, expandedAreaByMerge, findCellPosition, isEqualCells,
        toBox,
        toMergeMap
    } from "./helper";
    import Selection from "./Selection.svelte";
    import {drawCells} from "./cell";
    import classnames from "../util/classname";

    // 默认行高度
    const DEFAULT_ROW_HEIGHT = 30;

    // 默认列宽度
    const DEFAULT_COL_WIDTH = 80;

    const dispatch = createEventDispatcher();

    let className = '';
    export { className as class };

    export let contentWidth = 0;//宽度

    export let contentHeight = 0;//高度
    /**
     * 列数
     */
    export let columns = 10;

    /**
     * 行数
     */
    export let rows = 20;

    /**
     * Number of frozen rows
     */
    export let frozenRows = 0;

    /**
     * Number of frozen columns
     */
    export let frozenColumns = 0;

    /**
     *
     */
    export let scrollTop = 0;
    /**
     *
     */
    export let scrollLeft = 0;

    /**
     * 是否可编辑
     */
    export let editable = false;

    export let showGrid = true;
    /**
     * 合并区域
     */
    export let mergedCells: Area[] = [];

    /**
     * 鼠标移动到的单元格
     */
    export let overCell:CellPosition = undefined;


    /**
     * 行高度计数函数
     */
    export let rowHeight:ItemSizer = (_)=>DEFAULT_ROW_HEIGHT;

    /**
     * 列宽度计算函数
     */
    export let colWidth:ItemSizer = (_)=>DEFAULT_COL_WIDTH;

    /**
     * 单元格数据获取函数
     * @param _cellPos
     */
    export let data = (_cellPos:CellPosition)=>{return {text:''} as CellData};

    /**
     * 是否显示选区
     */
    export let showSelection = true;
    /**
     *
     */
    export let rowSelection = false;
    /**
     * 选区
     */
    export let selections: Area[] = [];

    //行、列定位数据
    $: tableRowRange = calculateRange(Infinity,0,rows,rowHeight);
    $: tableColRange = calculateRange(Infinity,0,columns,colWidth);

    $: tableWidth = columns>0?calculateColumnWidths(Infinity,0,columns,colWidth).reduce((t,v)=>t+v):0;
    $: tableHeight = rows>0?calculateRowHeights(Infinity,0,rows,rowHeight).reduce((t,v)=>t+v):0;
    //当前显示的行、列定位数据
    $: rowRange = calculateRange(contentHeight,scrollTop,rows,rowHeight);
    $: colRange = calculateRange(contentWidth,scrollLeft,columns,colWidth);

    $: mergedCellMap = toMergeMap(mergedCells);
    /**
     * 当前显示的单元格
     */
    $: showCells = buildShowCells(frozenRows,frozenColumns,rowRange,colRange,mergedCellMap,data);
    $: frozenRowCells = buildFrozenRowCells(frozenRows,tableRowRange,colRange,mergedCellMap,data);
    $: frozenColumnCells = buildFrozenColumnCells(frozenColumns,rowRange,tableColRange,mergedCellMap,data);
    $: frozenIntersectionCells = buildIntersectionCells(frozenRows,frozenColumns,tableRowRange,tableColRange,mergedCellMap,data);

    $:if(showCells){
        drawCells(layer,showCells,frozenRowCells,frozenColumnCells,frozenIntersectionCells,scrollTop,scrollLeft,showGrid)
    }
    /**
     * 冻结单元格
     */
    $: frozenRowHeight = frozenRows>0?calculateRowHeights(Infinity,0,frozenRows,rowHeight).reduce((t,v)=>t+v):0;
    $: frozenColumnWidth = frozenColumns>0?calculateColumnWidths(Infinity,0,frozenColumns,colWidth).reduce((t,v)=>t+v):0;

    //选区
    $: selectionBoxes = selections.map(({startRow,endRow,startCol,endCol})=>
        toBox({startRow,endRow,
            startCol:(rowSelection?0:startCol)
            ,endCol:(rowSelection?columns-1:endCol)},tableColRange,tableRowRange,frozenRows,frozenColumns,scrollLeft,scrollTop));

    // $: overCellStyle = overCell?buildOverCellStyle(overCell,frozenRows,frozenColumns,scrollLeft,scrollTop):'';

    $: classes = classnames('relative overflow-hidden',className);

    let panel = undefined;
    let container = undefined;

    let time = 0;
    let stage =undefined;
    let layer = undefined;
    let intersectionLayer = undefined;

    let activePosition;
    let startPosition;
    let overPosition;
    let containerOffsetX = 0;
    let containerOffsetY = 0;

    let rowResizeElement:HTMLElement = undefined;
    let rowResizing = false;
    let rowResizingStartPageY = 0;
    /* 高度调整行定位 */
    let rowResizingIndex:number = undefined;
    /* 行起始位置 */
    let rowResizingY:number = undefined;
    /* 原始行高 */
    let rowResizingHeight:number = undefined;
    let rowY = 0;
    let columnResizeElement:HTMLElement = undefined;
    let columnResizing = false;
    let columnResizingIndex:number = undefined;
    let columnResizingX:number = undefined;
    let columnResizingWidth:number = undefined;
    let columnX = 0;

    /**
     *
     * @param overCell
     * @param frozenRows
     * @param frozenColumns
     * @param scrollLeft
     * @param scrollTop
     */
    const buildOverCellStyle = (overCell,frozenRows,frozenColumns,scrollLeft,scrollTop) => {
        const {rowIndex,columnIndex} = overCell;
        const box = toBox({startRow:rowIndex,endRow:rowIndex,startCol:columnIndex,endCol:columnIndex},
            tableColRange,tableRowRange,frozenRows,frozenColumns,scrollLeft,scrollTop);
        return `left:${box.x}px;top:${box.y}px;width:${box.width}px;height:${box.height}px`;
    }

    /**
     *
     * @param position
     */
    const findCellByPosition = (position:CellPosition) => {
        if(!position)return;
        let cells = [...showCells];
        if(frozenRows>0 || frozenColumns>0){
            if(position.rowIndex<frozenRows && position.columnIndex<frozenColumns){
                //公共区域
                cells = [...frozenIntersectionCells];
            }else if(position.rowIndex<frozenRows && position.columnIndex>=frozenColumns){
                //行冻结区域
                cells = [...frozenRowCells];
            }else if(position.columnIndex<frozenColumns && position.rowIndex>=frozenRows){
                //列冻结区域
                cells = [...frozenColumnCells];
            }
        }

        cells = cells.filter(cell=>isEqualCells({rowIndex:cell.rowIndex,columnIndex:cell.columnIndex},position));

        if(cells.length){
            return cells[0];
        }
    }

    const normalMouseUp = (event) => {
        let {offsetX,offsetY,shiftKey} = event;
        activePosition = calculateCellPosition(offsetX,offsetY,mergedCellMap);

        if(activePosition){
            if(shiftKey && selections[0]){
                selections = [expandedAreaByMerge({
                    startRow:Math.min(selections[0].startRow,activePosition.rowIndex),
                    endRow:Math.max(selections[0].endRow,activePosition.rowIndex),
                    startCol:Math.min(selections[0].startCol,activePosition.columnIndex),
                    endCol:Math.max(selections[0].endCol,activePosition.columnIndex),
                },mergedCellMap)];
            }else{
                selections = [expandedAreaByMerge({
                    startRow:activePosition.rowIndex,
                    endRow:activePosition.rowIndex,
                    startCol:rowSelection?0:activePosition.columnIndex,
                    endCol:rowSelection?(columns-1):activePosition.columnIndex
                },mergedCellMap)];
            }
            //stopSelection({selections,evt:detail.evt,stopPosition:{...activePosition}});
        }
    }

    const handle_x_scroll = (e) => {
        scrollLeft = e.target.scrollLeft;
    }
    /**
     *
     * @param e
     */
    const handle_y_scroll = (e) => {
        scrollTop = e.target.scrollTop;
    }
    /**
     *
     */
    const handle_mouse_move = (e) => {
        const {offsetX,offsetY} = e;
        const movePosition = calculateCellPosition(offsetX,offsetY,mergedCellMap);
        if(movePosition &&(!overCell || !isEqualCells(overCell,movePosition))){
            overCell = movePosition;
            if(!rowResizing && !columnResizing){
                // //进入单元格，显示高、宽调整
                const cell = findCellByPosition(overCell);
                if(cell){
                    rowResizingY = cell.y + cell.height;
                    rowY = rowResizingY;
                    rowResizingHeight = cell.height;
                    rowResizingIndex = cell.rowIndex;

                    columnResizingX =  cell.x + cell.width;
                    columnX = columnResizingX;
                    columnResizingWidth = cell.width;
                    columnResizingIndex = cell.columnIndex;
                }
            }
        }
    }

    const mouseStart = (event) => {
        let {offsetX,offsetY,pageX,pageY,target,layerX,layerY} = event;
        //记录容器的offset量
        containerOffsetX = pageX - offsetX;
        containerOffsetY = pageY - offsetY;
        startPosition = calculateCellPosition(offsetX,offsetY,mergedCellMap);

        if(target === columnResizeElement){
            columnResizing = true;
            containerOffsetX = columnResizeElement.parentElement.parentElement.offsetLeft;
            startPosition = {...overCell};
        }
    }

    const mouseDrag = (event) => {
        if(rowResizing && rowResizeElement){
            const delta = event.pageY - rowResizingStartPageY;
            rowY = rowResizingY + delta;
        } else if(columnResizing && columnResizeElement){
            columnX =  event.pageX - containerOffsetX + scrollLeft;
        }else if(!rowResizing && startPosition) {
            // 拖动 - 选择区域
            const offsetX = event.pageX - containerOffsetX;
            const offsetY = event.pageY - containerOffsetY;

            //单元格选择
            let dragging = calculateCellPosition(offsetX, offsetY, mergedCellMap);
            //
            if(dragging && (!overPosition || !isEqualCells(dragging,overPosition))){
                overPosition = dragging;
                //更新选区
                selections = [expandedAreaByMerge({
                    startRow: Math.min(startPosition.rowIndex, overPosition.rowIndex),
                    endRow: Math.max(startPosition.rowIndex, overPosition.rowIndex),
                    startCol: Math.min(startPosition.columnIndex, overPosition.columnIndex),
                    endCol: Math.max(startPosition.columnIndex, overPosition.columnIndex)
                }, mergedCellMap)];

            }
        }
    }

    const mouseStop = (event) => {

        if(rowResizing){
            dispatch('row-resize',{rowIndex:rowResizingIndex,value:rowY - rowResizingY + rowResizingHeight});
            rowResizing = false;
        }else if(columnResizing){
            const value = columnX - columnResizingX + columnResizingWidth;
            if(value>5){
                dispatch('col-resize',{columnIndex:columnResizingIndex,value});
            }
            columnResizing = false;
        }
        startPosition = undefined;
        rowResizing = false;
        columnResizing = false;
    }

    /**
     *
     * @param offsetX
     * @param offsetY
     */
    const calculateCellPosition = (offsetX,offsetY,mergedCellMap)=>{
        let cellPosition;
        if(offsetX<frozenColumnWidth && offsetY<frozenRowHeight){
            //冻结的交叉部分
            cellPosition = findCellPosition(tableRowRange,tableColRange,offsetX,offsetY);
        }else if(offsetX<frozenColumnWidth){
            //行冻结区域
            cellPosition = findCellPosition(rowRange,tableColRange,offsetX,offsetY);
        }else if(offsetY<frozenRowHeight){
            //列冻结区域
            cellPosition = findCellPosition(tableRowRange,colRange,offsetX,offsetY);
        }else{
            //其他单元格区域
            cellPosition = findCellPosition(rowRange,colRange,offsetX,offsetY);
        }

        if(cellPosition){
            const cellKey = cellIdentifier(cellPosition.rowIndex,cellPosition.columnIndex);
            if(mergedCellMap.has(cellKey)){
                const merged = mergedCellMap.get(cellKey);
                return {rowIndex:merged.startRow,columnIndex:merged.startCol};
            }
        }

        return cellPosition;
    }

    onMount(()=>{
        stage = new Konva.Stage({
            container
        });
        handle_resize({});
        layer = new Konva.Layer();
        stage.add(layer);
    })

    const handle_resize = (e) => {
        throttle(()=>{
            contentHeight = panel.parentNode.offsetHeight;
            contentWidth = panel.parentNode.offsetWidth;
            if(stage){
                stage.size({
                    width:contentWidth,
                    height:contentHeight
                });
            }
        },100, { 'trailing': false })();
    }

</script>

<svelte:window on:resize={handle_resize}/>

<div class={classes} bind:this={panel} use:mouse={{mouseStart,mouseDrag,mouseStop,normalMouseUp}}>
    <div bind:this={container}
         on:mousemove={handle_mouse_move}
         class:cursor-col-resize={columnResizing}
    >
    </div>
    <!--  选区  -->
    {#if showSelection && Array.isArray(selectionBoxes)}
        <Selection {selectionBoxes}>

        </Selection>
    {/if}

    <!-- 行高度调整-->
    <div style:top={`${rowY-scrollTop}px`} bind:this={rowResizeElement} class="grid-row-resize"
         class:resizing={rowResizing}
         class:active={rowResizingIndex!==undefined && rowResizingIndex>-1}>
    </div>
    <!-- 列宽带度调整-->
    <div style:left={`${columnX-scrollLeft}px`} bind:this={columnResizeElement}
         class="absolute h-full cursor-col-resize w-1 top-0 -ml-1"
         class:bg-blue-50={columnResizing}
         class:active={columnResizingIndex!==undefined && columnResizingIndex>-1}></div>

    <!--  横向滚动条  -->
    {#if tableWidth>contentWidth}
        <div class="scrollbar overflow-x-auto absolute w-full h-4 left-0" tabIndex={-1}
             on:mousedown|stopPropagation
             on:scroll={handle_x_scroll}
             style:top={`${Math.min(tableHeight,contentHeight-16)}px`}
        >
            <div style:width={`${tableWidth+20}px`} style:height="1px"></div>
        </div>
    {/if}

    <!--  纵向滚动条  -->
    {#if tableHeight>contentHeight}
        <div class="scrollbar overflow-y-auto absolute w-4 h-full top-0" tabIndex={-1}
             on:scroll={handle_y_scroll}
             on:mousedown|stopPropagation
             style:left={`${Math.min(tableWidth,contentWidth-16)}px`}>
            <div style:height={`${(tableHeight+20)}px`}></div>
        </div>
    {/if}
</div>





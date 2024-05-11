<script>

    import DataGrid from "./DataGrid.svelte";
    import classnames from "../util/classname";

    let className = '';
    export { className as class };

    export let editable = false;

    /**
     * Number of frozen rows
     */
    export let frozenRows = 0;

    /**
     * Number of frozen columns
     */
    export let frozenColumns = 0;
    /**
     * 单元格
     */
    export let data = [];

    /**
     * 合并单元格
     */
    export let mergedCells = [];

    export let selections = [];

    /**
     * 列模型
     */
    export let colModels = [{},{},{},{},{},{},{},{}];//列数

    export let showYScroll = true;

    export let scrollLeft = 0;

    // 计算列数
    $:columns = colModels.length;
    $:rows = Math.ceil(data.length/columns);

    $:gridData = ({rowIndex,columnIndex})=>{
        const index = rowIndex*columns+columnIndex;
        const cellData = {...data[index]};

        return {
            text:'',
            ...cellData
        }
    }

    $:colWidth = columnIndex=>{
        if(colModels[columnIndex] && colModels[columnIndex].width>0){
            return colModels[columnIndex].width;
        }else{
            return 80;
        }
    }

    $:classes = classnames('p-0',className);


</script>

<DataGrid class={classes} data={gridData}
          bind:selections
          {colWidth}
          {columns}
          {rows}
          {frozenRows}
          {frozenColumns}
          {mergedCells}
          {editable}
          showGrid={editable}
          {showYScroll}
          {scrollLeft}
          on:col-resize
          on:selection-stop/>




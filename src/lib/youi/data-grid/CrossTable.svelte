<!--
    交叉数据表格组件
-->

<script lang="ts">
    import DataGrid from "./DataGrid.svelte";
    import classnames from "../util/classname";

    let className = '';
    export { className as class };

    export let editable = false;
    /**
     * 主栏列数
     */
    export let mainColumns = 1;

    export let mainFill = '#feff99';
    /**
     * 宾栏行数
     */
    export let slaveRows = 1;

    /**
     * 表脚行数
     */
    export let footerRows = 1;

    export let slaveFill = '#99ceff';
    /**
     * 合并单元格
     */
    export let mergedCells = [];
    /**
     *
     */
    export let colModels = [{},{},{},{},{},{},{},{}];//列数
    /**
     * 单元格
     */
    export let data = [];

    export let showXScroll = true;

    export let showYScroll = true;

    export let scrollLeft = 0;

    export let scrollTop = 0;

    export let contentHeight = undefined;

    export let selections = [];

    export let tableHeight;

    // 计算列数
    $:columns = colModels.length;
    $:rows = Math.ceil(data.length/columns);

    $:slaveEndMergedCellKeys = mergedCells
        .filter(mergedCell => mergedCell.startRow<slaveRows && mergedCell.endCol === columns-1)
        .map(mergedCell=>[mergedCell.startRow,mergedCell.startCol].join('_'));

    $:gridData = ({rowIndex, columnIndex})=>{
        let fill,align,border = {};
        const index = rowIndex*columns+columnIndex;
        const cellData = {...data[index]};

        if(rowIndex<slaveRows && columnIndex<mainColumns){
            //交叉单元格
            fill = slaveFill;
            align = 'center';
            if(columnIndex>0){
                border.borderLeft = 'black';
            }
            border.borderTop='black';
            border.borderBottom='black';
            border.borderRight='black';
        }else if(rowIndex>=slaveRows && columnIndex<mainColumns && rowIndex<rows-footerRows){
            //主栏单元格
            fill = mainFill;
            align = 'left';

            if(columnIndex === mainColumns-1){
                border.borderRight='black';
            }
        }else if(rowIndex<slaveRows && columnIndex>=mainColumns){
            //宾栏单元格
            fill = slaveFill;
            align = 'center';
            border.borderLeft = 'black';
            border.borderTop='black';
            if(columnIndex<columns-1 && !slaveEndMergedCellKeys.includes([rowIndex,columnIndex].join('_'))){
                border.borderRight='black';
            }
            border.borderBottom='black';
            //
            if(cellData.corner){
                border.borderLeft = slaveFill;
            }
        }else if(rowIndex<rows-footerRows){
            //数据单元格
            align = 'right';
        }

        if(rowIndex === 0){
            border.borderTopWidth = 3;
        }else if(rowIndex === rows-1-footerRows){
            border.borderBottom = 'black';
            border.borderBottomWidth = 2;
        }else if(rowIndex === rows-footerRows && footerRows>0){
            border.borderTop = 'black';
            border.borderTopWidth = 1;
        }

        return {
            text:'',
            align,
            fill,
            fontSize:14,
            ...border,
            ...cellData
        }
    };

    $:colWidth = columnIndex=>{
        if(colModels[columnIndex] && colModels[columnIndex].width>0){
            return colModels[columnIndex].width;
        }else{
            return 80;
        }
    }

    $:classes = classnames('p-0',className);

    // const handle_col_resize = ({detail}) => {
    //     const {columnIndex,value } = detail;
    //     colModels[columnIndex] = colModels[columnIndex]||{};
    //     Object.assign(colModels[columnIndex],{width:value});
    // }

</script>

<DataGrid bind:tableHeight
        class={classes} data={gridData}
          {contentHeight}
          {colWidth}
          {columns}
          {rows}
          {mergedCells}
          {editable}
          showGrid={editable}
          frozenRows = {slaveRows}
          frozenColumns = {mainColumns}
          {showXScroll}
          {showYScroll}
          {scrollLeft}
          {scrollTop}
          bind:selections
          on:col-resize
          on:selection-stop
          on:edited
          on:focus
          on:table-height
/>
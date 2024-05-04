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
        }else if(rowIndex>=slaveRows && columnIndex<mainColumns){
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
        }else{
            //数据单元格
        }

        if(rowIndex === 0){
            border.borderTopWidth = 3;
        }else if(rowIndex === rows-1){
            border.borderBottom = 'black';
            border.borderBottomWidth = 2;
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

    const handle_col_resize = ({detail}) => {
        const {columnIndex,value } = detail;
        colModels[columnIndex] = colModels[columnIndex]||{};
        Object.assign(colModels[columnIndex],{width:value});
    }

</script>

<DataGrid class={classes} data={gridData}
          {colWidth}
          {columns}
          {rows}
          {mergedCells}
          {editable}
          showGrid={editable}
          frozenRows = {slaveRows}
          frozenColumns = {mainColumns}
          on:col-resize = {handle_col_resize}/>
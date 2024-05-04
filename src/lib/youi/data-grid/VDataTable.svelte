<!--
    虚拟化数据表格组件
-->
<script>
    import DataGrid from "./DataGrid.svelte";

    let className = '';
    export { className as class };
    /**
     * 数据
     */
    export let records = [];

    /**
     * 列信息
     * @type {*[]}
     */
    export let columns = [];

    const parseHeaderRows = (_columns) => {
        return 1;
    }

    $:headerRows = parseHeaderRows(columns);

    $:gridData = ({rowIndex,columnIndex})=>{
        if(rowIndex ===0){
            if(columnIndex<columns.length){
                return {
                    text:columns[columnIndex].header,
                    align:'center',
                    fill:'#f1f1f1',
                    fontStyle:'bold'
                }
            }
        }else if(rowIndex>0 && rowIndex<records.length){
            if(columnIndex<columns.length){
                let dataType = columns[columnIndex].dataType||'string';
                let value = records[rowIndex-1][columns[columnIndex].header];
                return {
                    text:value,
                    align: dataType==='number'?'right':''
                }
            }
        }
    };

    $:colWidth = columnIndex => {
        if(columns[columnIndex] && columns[columnIndex].width){
            return columns[columnIndex].width;
        }
        return 80;
    }

    /**
     *  列宽度调整
     * @param detail
     */
    const handle_col_resize = ({detail}) => {
        const {columnIndex,value} = detail;
        if(columnIndex>-1 && columns[columnIndex]){
            columns[columnIndex].width = value;
            columns = columns;
        }
    }

</script>

<DataGrid class={className} data={gridData}
          columns={columns.length}
          rows={records.length}
          frozenRows={headerRows}
          {colWidth}
          contentWidth={1024} contentHeight={450}
          on:col-resize = {handle_col_resize}/>
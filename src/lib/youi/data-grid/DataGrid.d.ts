
export type ItemSizer = (index: number) => number;

/**
 * 行或者列位置
 */
export interface Position{
    index:number,
    size:number,
    offset:number
}

/**
 * 行或者列区域
 */
export interface Range{
    start:number,
    stop:number,
    scroll:number,
    sizes:Position[]
}
/**
 * 单元格行列定位
 */
export interface CellPosition {
    rowIndex: number;
    columnIndex: number;
}

/**
 * 单元格数据
 */
export interface CellData{
    text:string,
    x:number,
    y:number,
    width:number,
    height:number
}

export interface Area{
    startRow:number,
    endRow:number,
    startCol:number,
    endCol:number
}
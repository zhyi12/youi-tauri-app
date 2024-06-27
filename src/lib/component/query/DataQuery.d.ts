import type {FilterNode} from "../filter/Filter";

export interface QueryModel{

    id?:string,

    steps:IStep[]

    columnInfos?:ColumnInfo[]
}

/**
 * 列信息
 */
export interface ColumnInfo {
    name:string,
    width:number
}

export interface IQueryModelService<T> {

    fetch:(id)=>Promise<QueryModel>,

    save:(t:T)=>Promise<number>
}

export interface IStep{
    id:string,
    text:string
}

export interface ColumnsStep extends IStep{
    columns:Column[],

    selectedColumnNames:string[]
}

/**
 * 列选择
 */
export interface Select extends ColumnsStep{

}

/**
 * 过滤条件
 */
export interface Filter extends IStep{
    /**
     * 条件集合
     */
    items:FilterNode[]
}

/**
 * 列计算
 */
export interface Calculator extends IStep{
    calculators:Column[]
}

/**
 * 左右连接
 */
export interface Join extends IStep{
    /**
     * 左连接、右连接
     */
    how:string,
    /**
     * 左连接条件
     */
    leftOn:Column[],
    /**
     * 右连接条件
     */
    rightOn:Column[]
}

export interface Column {
    name:string,
    text:string,
    dataType:string,
    expression?:string
}
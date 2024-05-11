/**
 *
 */
export interface ReportModel{
    widgets:IWidget[],
    width?:number,
}

/**
 *
 */
export interface IWidget {
    id:string,
    name:string,
}

export interface TableWidget<T> extends IWidget{
    /**
     * 表格数据
     */
    data:T[],
    /**
     * 列模型
     */
    colModels:ColModel[],
    /**
     * 行描述信息
     */
    rows?:Row[],
}

export interface CrossTable<T> extends TableWidget<T>{
    /**
     * 主栏列数
     */
    mainColumns:number,
    /**
     * 宾栏行数
     */
    salveRows:number
}

export interface ColModel {
    width?:number
}

export interface Row {
    height?:number,
    id:string,
}
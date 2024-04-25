export interface Condition {
    property:string,
    value:any,
    operator:string
}

export interface Pager{
    pageSize:number,
    pageIndex:number
}

export interface Total{
    count:number
}
/**
 * 过滤条件节点，包括连接和条件
 */
export interface FilterNode {

    id:string,

    text?:string,
    /**
     * 节点类型，Or,And,Item
     */
    nodeType:string,

    level:number,
}

export interface Condition extends FilterNode{

    property:string,

    operator:string,

    dataType?:string,

    value:string[]|number[]
}

export interface Connection extends FilterNode {
    /**
     *
     */
    items?:FilterNode[]
}
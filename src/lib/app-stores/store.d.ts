import type {TreeItem} from "../youi/tree/Tree";

/**
 * 树数据模型
 */
export interface TreeStore{
    nodes:TreeItem[],
    activeId?:string,
    expandedIds?:string[]
}
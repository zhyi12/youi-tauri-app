import type {TreeItem} from "../youi/index.d";
import type {TreeObject} from "../app-entity/entity";
/**
 *
 */
export interface ITreeService<T extends TreeObject>{
    /**
     *
     * @param params
     */
    fetch:(params?)=>Promise<TreeItem[]>,
    /**
     *
     * @param id
     */
    findNode:(id:number)=>Promise<TreeItem>,

    /**
     *
     * @param id
     */
    findPrev:(id:number,pid:number,num:number)=>Promise<TreeItem>,

    /**
     *
     * @param id
     */
    findNext:(id:number,pid:number,num:number)=>Promise<TreeItem>,
    /**
     *
     * @param record
     */
    insert: (record:T)=>Promise<TreeItem>,

    /**
     * 重命名
     * @param id
     * @param text
     */
    rename:(id:number,text:string)=>Promise<void>,

    /**
     * 更新节点序号
     * @param id
     * @param num
     */
    updateNum:(id:number,pid:number,num:number)=>Promise<void>,

    /**
     *
     * @param id
     */
    remove:(id:number)=>Promise<void>,

}
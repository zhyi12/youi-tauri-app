import type {TreeObject} from "../entity";

export interface MetaNode extends TreeObject{

    /* ID */
    id: number ,
    /* PID */
    pid: number ,
    /* 序号 */
    num: number ,
    /* 名称 */
    text: string ,
    /* 数据对象名 */
    object_name: string ,
    /* 数据对象ID */
    object_id: number ,
    /* 创建时间 */
    create_time: number ,
    /* 更新时间 */
    update_time: number ,
    /* 创建者 */
    creator: string ,
    /* 修改者 */
    modified_by: string ,
}
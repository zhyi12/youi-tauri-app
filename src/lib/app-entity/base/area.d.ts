import type {TreeObject} from "../entity";

export interface Area extends TreeObject{

    /* ID */
    id: string ,
    /* PID */
    pid: string ,
    /* 序号 */
    num: string ,
    /* 简称 */
    text: string ,
    /* 全称 */
    full_text: string ,
    /* 创建时间 */
    create_time: number ,
    /* 更新时间 */
    update_time: number ,
    /* 创建者 */
    creator: string ,
    /* 修改者 */
    modified_by: string ,
}
import type {TreeObject} from "../entity";

export interface MacroGroupItem extends TreeObject{

    /* 分组项ID */
    id: number ,
    /* 名称 */
    text: string ,
    /* 父级ID */
    pid: number ,
    /* 分组ID */
    group_id: number ,
    /* 顺序号 */
    num: number ,
    /* 分组项解释 */
    desc: string ,
    /* 创建时间 */
    create_time: number ,
    /* 更新时间 */
    update_time: number ,
    /* 创建者 */
    creator: string ,
    /* 修改者 */
    modified_by: string ,
}
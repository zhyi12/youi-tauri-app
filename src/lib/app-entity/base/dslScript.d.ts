import type {TreeObject} from "../entity";

export interface DslScript extends TreeObject{

    /* ID */
    id: number ,
    /* 名称 */
    text: string ,
    /* 父级ID */
    pid: number ,
    /* 顺序号 */
    num: number ,
    /* 列宽 */
    col_widths: string ,
    /* 创建时间 */
    create_time: number ,
    /* 更新时间 */
    update_time: number ,
    /* 创建者 */
    creator: string ,
    /* 修改者 */
    modified_by: string ,
}

export interface DslScriptContent{
    id:number,
    text:string,
    content:string,
    col_widths:string,
}
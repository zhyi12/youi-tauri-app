export interface SurveyProject{

    /* ID */
    id: number ,
    /* 名称 */
    text: string ,
    /* 编码 */
    code: string ,
    /* 项目分类 */
    project_type: string ,
    /* 项目说明 */
    desc: string ,
    /* 图标 */
    icon: string ,
    /* 创建时间 */
    create_time: number ,
    /* 更新时间 */
    update_time: number ,
    /* 创建者 */
    creator: string ,
    /* 修改者 */
    modified_by: string ,
}
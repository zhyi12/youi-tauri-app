export interface SurveyTask{

    /* ID */
    id: number ,
    /* 编码 */
    code: string ,
    /* 名称 */
    text: string ,
    /* 报告期别 */
    period_type: string ,
    /* 调查范围 */
    stats_scope: string ,
    /* 创建时间 */
    create_time: number ,
    /* 更新时间 */
    update_time: number ,
    /* 创建者 */
    creator: string ,
    /* 修改者 */
    modified_by: string ,
}
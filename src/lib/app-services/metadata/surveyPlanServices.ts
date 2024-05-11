import {appDataBase} from "../../app-dao/dao.util";
import {findByPager} from "../service.util";
import {
    PROPERTIES,
    INSERT_PROPERTIES,
    SQL_FIND,
    SQL_SELECT,
    SQL_INSERT,
    SQL_REMOVE,
    SQL_UPDATE
} from "../../app-dao/metadata/surveyPlan.sql";

import type {Pager} from "../../app-dao/dao";
import type {SurveyPlan} from "../../app-entity/metadata/surveyPlan";
import {execute} from "../../tauri/dsl";

/**
 * 主键查询
 * @param id
 */
export async function findSurveyPlan(id:number|string):Promise<SurveyPlan|null>{
    const surveyPlans = await appDataBase.select<SurveyPlan[]>(SQL_FIND,[id]);
    if(surveyPlans.length){
        return surveyPlans[0];
    }
    return null;
}

/**
 * 插入调查制度
 * @param SurveyPlan
 */
export async function insertSurveyPlan(surveyPlan:SurveyPlan):Promise<number|string>{
    // eslint-disable-next-line @typescript-eslint/ban-ts-comment
    // @ts-ignore
    const bindValues= INSERT_PROPERTIES.map(prop=>surveyPlan[prop]||'');
    //
    const result = await appDataBase.execute(SQL_INSERT,bindValues);
    return result.lastInsertId;
}

/**
 * 更新调查制度
 * @param SurveyPlan
 */
export async function updateSurveyPlan(surveyPlan:SurveyPlan):Promise<number>{
    // eslint-disable-next-line @typescript-eslint/ban-ts-comment
    // @ts-ignore
    const bindValues= PROPERTIES.map(prop=>surveyPlan[prop]||'');
    //
    const result = await appDataBase.execute(SQL_UPDATE,bindValues);
    return result.rowsAffected;
}

/**
 * 删除调查制度
 * @param SurveyPlan
 */
export async function removeSurveyPlan(id:number):Promise<number>{
    const result = await appDataBase.execute(SQL_REMOVE,[id]);
    return result.rowsAffected;
}


/**
 * 分页查询
 * @param pager
 * @param surveyPlan
 */
export async function findSurveyPlanByPager(pager:Pager,surveyPlan?:SurveyPlan) {
    return await findByPager(pager,SQL_SELECT,surveyPlan)
}

/**
 * 获取每个项目最新的N个制度
 */
export async function findLastNByProject(n:number,dbConnect) {
    const dsl = 'read_sql("${dbConnect}","select id,text,project_id,update_time from stats_survey_plan order by year desc")' +
        '.agg("project_id",[col("id").list().slice(0,2),col("text").list().slice(0,'+n+')]).explode([col("id"),col("text")])';
    const result = await execute(dsl,[{name:'dbConnect',dataType:'string',value:dbConnect}]);
    return result;
}
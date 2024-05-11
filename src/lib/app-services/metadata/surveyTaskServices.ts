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
} from "../../app-dao/metadata/surveyTask.sql";

import type {Pager} from "../../app-dao/dao";
import type {SurveyTask} from "../../app-entity/metadata/surveyTask";

/**
 * 主键查询
 * @param id
 */
export async function findSurveyTask(id:number|string):Promise<SurveyTask|null>{
    const surveyTasks = await appDataBase.select<SurveyTask[]>(SQL_FIND,[id]);
    if(surveyTasks.length){
        return surveyTasks[0];
    }
    return null;
}

/**
 * 插入调查方法
 * @param SurveyTask
 */
export async function insertSurveyTask(surveyTask:SurveyTask):Promise<number|string>{
    // eslint-disable-next-line @typescript-eslint/ban-ts-comment
    // @ts-ignore
    const bindValues= INSERT_PROPERTIES.map(prop=>surveyTask[prop]||'');
    //
    const result = await appDataBase.execute(SQL_INSERT,bindValues);
    return result.lastInsertId;
}

/**
 * 更新调查方法
 * @param SurveyTask
 */
export async function updateSurveyTask(surveyTask:SurveyTask):Promise<number>{
    // eslint-disable-next-line @typescript-eslint/ban-ts-comment
    // @ts-ignore
    const bindValues= PROPERTIES.map(prop=>surveyTask[prop]||'');
    //
    const result = await appDataBase.execute(SQL_UPDATE,bindValues);
    return result.rowsAffected;
}

/**
 * 删除调查方法
 * @param SurveyTask
 */
export async function removeSurveyTask(id:number):Promise<number>{
    const result = await appDataBase.execute(SQL_REMOVE,[id]);
    return result.rowsAffected;
}


/**
 * 分页查询
 * @param pager
 * @param surveyTask
 */
export async function findSurveyTaskByPager(pager:Pager,surveyTask?:SurveyTask) {
    return await findByPager(pager,SQL_SELECT,surveyTask)
}
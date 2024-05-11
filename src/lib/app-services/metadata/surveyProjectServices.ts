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
} from "../../app-dao/metadata/surveyProject.sql";

import type {Pager} from "../../app-dao/dao";
import type {SurveyProject} from "../../app-entity/metadata/surveyProject";

/**
 * 主键查询
 * @param id
 */
export async function findSurveyProject(id:number|string):Promise<SurveyProject|null>{
    const surveyProjects = await appDataBase.select<SurveyProject[]>(SQL_FIND,[id]);
    if(surveyProjects.length){
        return surveyProjects[0];
    }
    return null;
}

/**
 * 插入调查项目
 * @param SurveyProject
 */
export async function insertSurveyProject(surveyProject:SurveyProject):Promise<number|string>{
    // eslint-disable-next-line @typescript-eslint/ban-ts-comment
    // @ts-ignore
    const bindValues= INSERT_PROPERTIES.map(prop=>surveyProject[prop]||'');
    //
    const result = await appDataBase.execute(SQL_INSERT,bindValues);
    return result.lastInsertId;
}

/**
 * 更新调查项目
 * @param SurveyProject
 */
export async function updateSurveyProject(surveyProject:SurveyProject):Promise<number>{
    // eslint-disable-next-line @typescript-eslint/ban-ts-comment
    // @ts-ignore
    const bindValues= PROPERTIES.map(prop=>surveyProject[prop]||'');
    //
    const result = await appDataBase.execute(SQL_UPDATE,bindValues);
    return result.rowsAffected;
}

/**
 * 删除调查项目
 * @param SurveyProject
 */
export async function removeSurveyProject(id:number):Promise<number>{
    const result = await appDataBase.execute(SQL_REMOVE,[id]);
    return result.rowsAffected;
}


/**
 * 分页查询
 * @param pager
 * @param surveyProject
 */
export async function findSurveyProjectByPager(pager:Pager,surveyProject?:SurveyProject) {
    return await findByPager(pager,SQL_SELECT,surveyProject)
}
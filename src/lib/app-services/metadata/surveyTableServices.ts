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
} from "../../app-dao/metadata/surveyTable.sql";

import type {Pager} from "../../app-dao/dao";
import type {SurveyTable} from "../../app-entity/metadata/surveyTable";

/**
 * 主键查询
 * @param id
 */
export async function findSurveyTable(id:number|string):Promise<SurveyTable|null>{
    const surveyTables = await appDataBase.select<SurveyTable[]>(SQL_FIND,[id]);
    if(surveyTables.length){
        return surveyTables[0];
    }
    return null;
}

/**
 * 插入调查制度
 * @param SurveyTable
 */
export async function insertSurveyTable(surveyTable:SurveyTable):Promise<number|string>{
    // eslint-disable-next-line @typescript-eslint/ban-ts-comment
    // @ts-ignore
    const bindValues= INSERT_PROPERTIES.map(prop=>surveyTable[prop]||'');
    //
    const result = await appDataBase.execute(SQL_INSERT,bindValues);
    return result.lastInsertId;
}

/**
 * 更新调查制度
 * @param SurveyTable
 */
export async function updateSurveyTable(surveyTable:SurveyTable):Promise<number>{
    // eslint-disable-next-line @typescript-eslint/ban-ts-comment
    // @ts-ignore
    const bindValues= PROPERTIES.map(prop=>surveyTable[prop]||'');
    //
    const result = await appDataBase.execute(SQL_UPDATE,bindValues);
    return result.rowsAffected;
}

/**
 * 删除调查制度
 * @param SurveyTable
 */
export async function removeSurveyTable(id:number):Promise<number>{
    const result = await appDataBase.execute(SQL_REMOVE,[id]);
    return result.rowsAffected;
}


/**
 * 分页查询
 * @param pager
 * @param surveyTable
 */
export async function findSurveyTableByPager(pager:Pager,surveyTable?:SurveyTable) {
    return await findByPager(pager,SQL_SELECT,surveyTable)
}
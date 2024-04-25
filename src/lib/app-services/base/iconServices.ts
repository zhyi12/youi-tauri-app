import {
    appDataBase,
    buildConditions,
    buildConditionsSql,
    buildLimitParamValues,
    wrapSqlCount, wrapSqlLimit
} from "../../app-dao/dao.util";

import {
    PROPERTIES,
    INSERT_PROPERTIES,
    SQL_FIND,
    SQL_SELECT,
    SQL_INSERT,
    SQL_REMOVE,
    SQL_UPDATE
} from "../../app-dao/base/icon.sql";

import type {Pager} from "../../app-dao/dao";
import type {Icon} from "../../app-entity/base/icon";

/**
 * 主键查询
 * @param id
 */
export async function findIcon(id:number|string):Promise<Icon|null>{
    const icons = await appDataBase.select<Icon[]>(SQL_FIND,[id]);
    if(icons.length){
        return icons[0];
    }
    return null;
}

/**
 * 插入图标
 * @param Icon
 */
export async function insertIcon(icon:Icon):Promise<number|string>{
    // eslint-disable-next-line @typescript-eslint/ban-ts-comment
    // @ts-ignore
    const bindValues= INSERT_PROPERTIES.map(prop=>icon[prop]||'');
    //
    const result = await appDataBase.execute(SQL_INSERT,bindValues);
    return result.lastInsertId;
}

/**
 * 更新图标
 * @param Icon
 */
export async function updateIcon(icon:Icon):Promise<number>{
    // eslint-disable-next-line @typescript-eslint/ban-ts-comment
    // @ts-ignore
    const bindValues= PROPERTIES.map(prop=>icon[prop]||'');
    //
    const result = await appDataBase.execute(SQL_UPDATE,bindValues);
    return result.rowsAffected;
}

/**
 * 删除图标
 * @param Icon
 */
export async function removeIcon(id:number):Promise<number>{
    const result = await appDataBase.execute(SQL_REMOVE,[id]);
    return result.rowsAffected;
}


/**
 * 分页查询
 * @param pager
 * @param icon
 */
export async function findByPager(pager:Pager,icon?:Icon) {
    const conditions = buildConditions(icon);
    const querySql = conditions.length?`${SQL_SELECT} where ${buildConditionsSql(conditions)}`:SQL_SELECT;
    let bindValues:any[] = conditions.map(condition=>condition.value);
    //获取记录总数
    const countResult = await appDataBase.select<{count:number}[]>(wrapSqlCount(querySql),bindValues);
    let totalCount = 0;
    if(countResult){
        totalCount = countResult[0].count;
    }
    //分页参数
    bindValues = bindValues.concat(buildLimitParamValues(pager));
    const records = await appDataBase.select<Icon[]>(wrapSqlLimit(querySql,conditions.length),bindValues);

    return {
        totalCount,
        records
    }
}
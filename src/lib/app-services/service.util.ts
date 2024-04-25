import type {Pager} from "../app-dao/dao";
import {
    appDataBase,
    buildConditions,
    buildConditionsSql,
    buildLimitParamValues,
    wrapSqlCount, wrapSqlLimit
} from "../app-dao/dao.util";

/**
 * 通用分页查询
 * @param pager
 * @param selectSql
 * @param entity
 */
export async  function findByPager<T>(pager:Pager,selectSql,entity?:T) {
    const conditions = buildConditions(entity);
    const querySql = conditions.length?`${selectSql} where ${buildConditionsSql(conditions)}`:selectSql;
    let bindValues:any[] = conditions.map(condition=>condition.value);
    //获取记录总数
    const countResult = await appDataBase.select<{count:number}[]>(wrapSqlCount(querySql),bindValues);
    let totalCount = 0;
    if(countResult){
        totalCount = countResult[0].count;
    }
    //分页参数
    bindValues = bindValues.concat(buildLimitParamValues(pager));

    const records = await appDataBase.select<T[]>(wrapSqlLimit(querySql,conditions.length),bindValues);

    return {
        totalCount,
        records
    }
}
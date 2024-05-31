import type {Condition,Pager} from "./dao";
import Database from "@tauri-apps/plugin-sql";

export const appDataBase = new Database("sqlite:app.db");

export const buildSelectSql = (tableName:string,properties:string[]) => `select ${properties.join(',')} from ${tableName}`;
/**
 *
 * @param tableName
 * @param properties
 */
export const buildInsertSql = (tableName:string,properties:string[]) =>
    `insert into ${tableName}(${properties.join(',')}) values (${properties.map((prop,idx)=>'$'+(idx+1)).join(',')})`;

/**
 *
 * @param tableName
 * @param properties
 */
export const buildUpdateSql = (tableName:string,properties:string[],idProp?:string) =>
    `update ${tableName} set ${properties.map((prop,idx)=>prop+'= $'+(idx+2)).join()} where ${idProp||'id'} = $1`;
/**
 *
 * @param tableName
 */
export const buildDeleteSql = (tableName:string) => `delete from  ${tableName} where id = $1`;
/**
 *
 * @param tableName
 * @param properties
 */
export const buildTreeSql = (tableName:string,properties:string[],whereSql?:string,orderBy?:string,filter?:string)=>{
    const prop_str = properties.join();
    return [`WITH RECURSIVE t(${prop_str},level,path) AS (`,
        `   SELECT ${prop_str},1 as level,'/'||id as path from ${tableName} ${whereSql?` where ${whereSql}`:''}`,
        `   union all`,
        `   SELECT ${properties.map(prop=>`d.${prop}`).join()},t.level+1 as level,t.path||'/'||d.id as path from ${tableName} d inner join t on t.id = d.pid `,
        `) select ${prop_str},level,path from t ${filter?`where ${filter} `:''} order by ${orderBy?orderBy:'path'}`
    ].join(' ');
}

/**
 *
 * @param tableName
 * @param properties
 */
export const buildLevelTreeSql = (tableName:string,properties:string[],maxLevelParamIndex:number,whereSql?:string)=>{
    const prop_str = properties.join();
    return [`WITH RECURSIVE t(${prop_str},level,path) AS (`,
        `   SELECT ${prop_str},1 as level,'/'||id as path from ${tableName} ${whereSql?` where ${whereSql}`:''}`,
        `   union all`,
        `   SELECT ${properties.map(prop=>`d.${prop}`).join()},t.level+1 as level,t.path||'/'||d.id as path from ${tableName} d inner join t on t.id = d.pid `,
        `) select ${prop_str},level,path from t where level < 3 order by path`
    ].join(' ');
}
/**
 *
 * @param o 构建等于条件
 */
export const buildConditions = (o:unknown):Condition[] =>{
    const conditions:Condition[]= [];
    if(o){
        Object.keys(o).forEach((key)=>{
            //
            // eslint-disable-next-line @typescript-eslint/ban-ts-comment
            // @ts-ignore
            const value = o[key];
            if(!isNull(value)){
                conditions.push({
                    property:key,
                    value:value,
                    operator:'='
                });
            }
        })
    }
    return conditions;
}
/**
 *
 * @param conditions
 */
export const buildConditionsSql = (conditions:Condition[]) => {
    let sql:string[] = [];
    if(conditions.length){
        sql = conditions.map(condition=>`${condition.property} ${condition.operator} '${condition.value}' `);
    }
    return sql.join(' and ');
}


/**
 *
 * @param querySql
 * @param vLen
 */
export const wrapSqlLimit = (querySql:string,vLen:number) => `${querySql} limit $${vLen+1} offset $${vLen+2}`;

/**
 *
 * @param pager
 */
export const buildLimitParamValues = (pager:Pager)=>{
    if(pager.pageIndex<1){
        pager.pageIndex = 1;
    }
    //limit $1 offset $2
    return [pager.pageSize,(pager.pageIndex-1)*pager.pageSize];
};
/**
 *
 * @param querySql
 */
export const wrapSqlCount = (querySql:string) => `select count(1) as count from (${querySql}) t_`;

export const isNull = (value: any) =>
    value === void 0 || value === null || value === "";
import {buildDeleteSql, buildInsertSql, buildSelectSql, buildUpdateSql} from "../dao.util";

const TABLE_NAME = 'youi_geo_json';

export const PROPERTIES = ['area_id','geo_json','version','rect','center','create_time','update_time','creator','modified_by'];
export const INSERT_PROPERTIES = PROPERTIES.slice(1);

/**
 * geoArea 查询语句
 */
export const SQL_SELECT = buildSelectSql(TABLE_NAME,PROPERTIES);
/**
 * geoArea 主键查询语句
 */
export const SQL_FIND = `${SQL_SELECT} where id = $1`;
/**
 * geoArea 插入语句
 */
export const SQL_INSERT =  buildInsertSql(TABLE_NAME,INSERT_PROPERTIES);
/**
 * geoArea 更新语句
 */
export const SQL_UPDATE = buildUpdateSql(TABLE_NAME,INSERT_PROPERTIES);
/**
 * geoArea 删除语句
 */
export const SQL_REMOVE = buildDeleteSql(TABLE_NAME);
import {buildDeleteSql, buildInsertSql, buildSelectSql, buildUpdateSql} from "../dao.util";

const TABLE_NAME = 'stats_macro_group';

export const PROPERTIES = ['id','text','group_type','desc','create_time','update_time','creator','modified_by'];
export const INSERT_PROPERTIES = PROPERTIES.slice(1);

/**
 * macroGroup 查询语句
 */
export const SQL_SELECT = buildSelectSql(TABLE_NAME,PROPERTIES);
/**
 * macroGroup 主键查询语句
 */
export const SQL_FIND = `${SQL_SELECT} where id = $1`;
/**
 * macroGroup 插入语句
 */
export const SQL_INSERT =  buildInsertSql(TABLE_NAME,INSERT_PROPERTIES);
/**
 * macroGroup 更新语句
 */
export const SQL_UPDATE = buildUpdateSql(TABLE_NAME,INSERT_PROPERTIES);
/**
 * macroGroup 删除语句
 */
export const SQL_REMOVE = buildDeleteSql(TABLE_NAME);
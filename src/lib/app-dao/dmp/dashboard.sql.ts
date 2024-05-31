import {buildDeleteSql, buildInsertSql, buildSelectSql, buildUpdateSql} from "../dao.util";

const TABLE_NAME = 'youi_dmp_dashboard';

export const PROPERTIES = ['id','group_id','num','text','full_text','create_time','update_time','creator','modified_by'];
export const INSERT_PROPERTIES = PROPERTIES.slice(1);

/**
 * dashboard 查询语句
 */
export const SQL_SELECT = buildSelectSql(TABLE_NAME,PROPERTIES);
/**
 * dashboard 主键查询语句
 */
export const SQL_FIND = `${SQL_SELECT} where id = $1`;

export const SQL_FIND_BY_GROUP = `${SQL_SELECT} where group_id = $1`;
/**
 * dashboard 插入语句
 */
export const SQL_INSERT =  buildInsertSql(TABLE_NAME,INSERT_PROPERTIES);
/**
 * dashboard 更新语句
 */
export const SQL_UPDATE = buildUpdateSql(TABLE_NAME,INSERT_PROPERTIES);
/**
 * dashboard 删除语句
 */
export const SQL_REMOVE = buildDeleteSql(TABLE_NAME);
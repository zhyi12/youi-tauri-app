import {buildDeleteSql, buildInsertSql, buildSelectSql, buildUpdateSql} from "../dao.util";

const TABLE_NAME = 'youi_icon';

export const PROPERTIES = ['id','text','name','content','create_time','update_time'];
export const INSERT_PROPERTIES = PROPERTIES.slice(1);

/**
 * icon 查询语句
 */
export const SQL_SELECT = buildSelectSql(TABLE_NAME,PROPERTIES);
/**
 * icon 主键查询语句
 */
export const SQL_FIND = `${SQL_SELECT} where id = $1`;
/**
 * icon 插入语句
 */
export const SQL_INSERT =  buildInsertSql(TABLE_NAME,INSERT_PROPERTIES);
/**
 * icon 更新语句
 */
export const SQL_UPDATE = buildUpdateSql(TABLE_NAME,INSERT_PROPERTIES);
/**
 * icon 删除语句
 */
export const SQL_REMOVE = buildDeleteSql(TABLE_NAME);
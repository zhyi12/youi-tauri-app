import {buildDeleteSql, buildInsertSql, buildSelectSql, buildUpdateSql} from "../dao.util";

const TABLE_NAME = 'youi_dmp_meta_item';

export const PROPERTIES = ['id','name','column_name','caption','full_caption','data_type','len','code','create_time','update_time','creator','modified_by'];
export const INSERT_PROPERTIES = PROPERTIES.slice(1);

/**
 * metaItem 查询语句
 */
export const SQL_SELECT = buildSelectSql(TABLE_NAME,PROPERTIES);
/**
 * metaItem 主键查询语句
 */
export const SQL_FIND = `${SQL_SELECT} where id = $1`;
/**
 * metaItem 插入语句
 */
export const SQL_INSERT =  buildInsertSql(TABLE_NAME,INSERT_PROPERTIES);
/**
 * metaItem 更新语句
 */
export const SQL_UPDATE = buildUpdateSql(TABLE_NAME,INSERT_PROPERTIES);
/**
 * metaItem 删除语句
 */
export const SQL_REMOVE = buildDeleteSql(TABLE_NAME);
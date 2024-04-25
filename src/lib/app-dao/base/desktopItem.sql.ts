import {buildDeleteSql, buildInsertSql, buildSelectSql, buildUpdateSql} from "../dao.util";

const TABLE_NAME = 'youi_desktop_item';

export const PROPERTIES = ['id','pid','num','text','item_type','uri','image','icon','params','create_time','update_time','creator','modified_by'];
export const INSERT_PROPERTIES = PROPERTIES.slice(1);

/**
 * desktopItem 查询语句
 */
export const SQL_SELECT = buildSelectSql(TABLE_NAME,PROPERTIES);
/**
 * desktopItem 主键查询语句
 */
export const SQL_FIND = `${SQL_SELECT} where id = $1`;
/**
 * desktopItem 插入语句
 */
export const SQL_INSERT =  buildInsertSql(TABLE_NAME,INSERT_PROPERTIES);
/**
 * desktopItem 更新语句
 */
export const SQL_UPDATE = buildUpdateSql(TABLE_NAME,INSERT_PROPERTIES);
/**
 * desktopItem 删除语句
 */
export const SQL_REMOVE = buildDeleteSql(TABLE_NAME);
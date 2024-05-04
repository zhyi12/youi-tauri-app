import {buildDeleteSql, buildInsertSql, buildSelectSql, buildUpdateSql,buildTreeSql} from "../dao.util";

const TABLE_NAME = 'youi_dsl_script';

export const PROPERTIES = ['id','text','pid','num','col_widths','create_time','update_time','creator','modified_by'];
export const INSERT_PROPERTIES = PROPERTIES.slice(1);

/**
 * dslScript 查询语句
 */
export const SQL_SELECT = buildSelectSql(TABLE_NAME,PROPERTIES);
/**
 * dslScript 主键查询语句
 */
export const SQL_FIND = `${SQL_SELECT} where id=$1`;
/**
 * dslScript 插入语句
 */
export const SQL_INSERT =  buildInsertSql(TABLE_NAME,INSERT_PROPERTIES);
/**
 * dslScript 更新语句
 */
export const SQL_UPDATE = buildUpdateSql(TABLE_NAME,INSERT_PROPERTIES);
/**
 * dslScript 删除语句
 */
export const SQL_REMOVE = buildDeleteSql(TABLE_NAME);
/**
 * dslScript 获取树语句
 */
export const SQL_TREE = buildTreeSql(TABLE_NAME,PROPERTIES,`pid is null or pid=''`);
/**
 * dslScript 获取同级别下一个节点
 */
export const SQL_FIND_NEXT = `${SQL_SELECT} where pid=$1 and num>=$2 order by num limit 2`;
/**
 * dslScript 获取同级别上一个节点
 */
export const SQL_FIND_PREV = `${SQL_SELECT} where pid=$1 and num<=$2 order by num desc limit 2`;
/**
 * dslScript 顶级获取同级别下一个节点
 */
export const SQL_TOP_FIND_NEXT = `${SQL_SELECT} where (pid is null or pid ='') and num>=$1 order by num limit 2`;
/**
 * dslScript 顶级获取同级别上一个节点
 */
export const SQL_TOP_FIND_PREV = `${SQL_SELECT} where (pid is null or pid ='') and num<=$1 order by num desc limit 2`;
/**
 * dslScript 更新名称
 */
export const SQL_UPDATE_TEXT = `update ${TABLE_NAME} set text=$2 where id = $1`;
/**
 * dslScript 节点移动更新父级和序号
 */
export const SQL_UPDATE_MOVE = `update ${TABLE_NAME} set pid=$2,num=$3 where id = $1`;

export const SQL_FIND_CONTENT = `select id,content,text,col_widths from ${TABLE_NAME} where id=$1`;

export const SQL_UPDATE_CONTENT = `update ${TABLE_NAME} set content=$2,update_time=$3 where id = $1`;

export const SQL_UPDATE_COL_WIDTHS = `update ${TABLE_NAME} set col_widths=$2 where id = $1`;
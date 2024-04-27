import {buildDeleteSql, buildInsertSql, buildSelectSql, buildUpdateSql,buildTreeSql} from "../dao.util";

const TABLE_NAME = 'stats_macro_indicator';

export const PROPERTIES = ['id','text','pid','num','desc','create_time','update_time','creator','modified_by'];
export const INSERT_PROPERTIES = PROPERTIES.slice(1);

/**
 * macroIndicator 查询语句
 */
export const SQL_SELECT = buildSelectSql(TABLE_NAME,PROPERTIES);
/**
 * macroIndicator 主键查询语句
 */
export const SQL_FIND = `${SQL_SELECT} where id=$1`;
/**
 * macroIndicator 插入语句
 */
export const SQL_INSERT =  buildInsertSql(TABLE_NAME,INSERT_PROPERTIES);
/**
 * macroIndicator 更新语句
 */
export const SQL_UPDATE = buildUpdateSql(TABLE_NAME,INSERT_PROPERTIES);
/**
 * macroIndicator 删除语句
 */
export const SQL_REMOVE = buildDeleteSql(TABLE_NAME);
/**
 * macroIndicator 获取树语句
 */
export const SQL_TREE = buildTreeSql(TABLE_NAME,PROPERTIES,`pid is null or pid=''`);
/**
 * macroIndicator 获取同级别下一个节点
 */
export const SQL_FIND_NEXT = `${SQL_SELECT} where pid=$1 and num>=$2 order by num limit 2`;
/**
 * macroIndicator 获取同级别上一个节点
 */
export const SQL_FIND_PREV = `${SQL_SELECT} where pid=$1 and num<=$2 order by num desc limit 2`;
/**
 * macroIndicator 顶级获取同级别下一个节点
 */
export const SQL_TOP_FIND_NEXT = `${SQL_SELECT} where (pid is null or pid ='') and num>=$1 order by num limit 2`;
/**
 * macroIndicator 顶级获取同级别上一个节点
 */
export const SQL_TOP_FIND_PREV = `${SQL_SELECT} where (pid is null or pid ='') and num<=$1 order by num desc limit 2`;
/**
 * macroIndicator 更新名称
 */
export const SQL_UPDATE_TEXT = `update ${TABLE_NAME} set text=$2 where id = $1`;
/**
 * macroIndicator 节点移动更新父级和序号
 */
export const SQL_UPDATE_MOVE = `update ${TABLE_NAME} set pid=$2,num=$3 where id = $1`;
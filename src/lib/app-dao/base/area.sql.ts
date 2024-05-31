import {buildDeleteSql, buildInsertSql, buildSelectSql, buildUpdateSql,buildTreeSql} from "../dao.util";

const TABLE_NAME = 'youi_geo_area';
const TABLE_GEO_JSON = 'youi_geo_json';

export const PROPERTIES = ['id','pid','num','text','full_text','area_level','create_time','update_time','creator','modified_by'];
export const INSERT_PROPERTIES = PROPERTIES;

/**
 * area 查询语句
 */
export const SQL_SELECT = buildSelectSql(TABLE_NAME,PROPERTIES);
/**
 * area 主键查询语句
 */
export const SQL_FIND = `${SQL_SELECT} where id=$1`;
/**
 * area 插入语句
 */
export const SQL_INSERT =  buildInsertSql(TABLE_NAME,INSERT_PROPERTIES);
/**
 * area 更新语句
 */
export const SQL_UPDATE = buildUpdateSql(TABLE_NAME,INSERT_PROPERTIES);
/**
 * area 删除语句
 */
export const SQL_REMOVE = buildDeleteSql(TABLE_NAME);
/**
 * area 获取树语句
 */
export const SQL_TREE = buildTreeSql(TABLE_NAME,PROPERTIES,`pid=$1`,'','level<$2');
/**
 * area 获取同级别下一个节点
 */
export const SQL_FIND_NEXT = `${SQL_SELECT} where pid=$1 and num>=$2 order by num limit 2`;
/**
 * area 获取同级别上一个节点
 */
export const SQL_FIND_PREV = `${SQL_SELECT} where pid=$1 and num<=$2 order by num desc limit 2`;
/**
 * area 顶级获取同级别下一个节点
 */
export const SQL_TOP_FIND_NEXT = `${SQL_SELECT} where (pid is null or pid ='') and num>=$1 order by num limit 2`;
/**
 * area 顶级获取同级别上一个节点
 */
export const SQL_TOP_FIND_PREV = `${SQL_SELECT} where (pid is null or pid ='') and num<=$1 order by num desc limit 2`;
/**
 * area 更新名称
 */
export const SQL_UPDATE_TEXT = `update ${TABLE_NAME} set text=$2 where id = $1`;
/**
 * area 节点移动更新父级和序号
 */
export const SQL_UPDATE_MOVE = `update ${TABLE_NAME} set pid=$2,num=$3 where id = $1`;


/**
 * 区域图层
 */
export const SQL_AREA_FIND_GEO_JSON = `select geo_json from ${TABLE_GEO_JSON} where area_id=$1 order by version desc limit 1`;

/**
 *
 */
export const SQL_AREA_INSERT_GEO_JON = `insert into ${TABLE_GEO_JSON}(area_id,geo_json,version) values($1,$2,$3)`;
/**
 *
 */
export const SQL_AREA_UPDATE_GEO_JON = `update ${TABLE_GEO_JSON} set geo_json=$3,version=$2 where area_id=$1`;

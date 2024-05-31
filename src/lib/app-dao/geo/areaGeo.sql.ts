import {buildInsertSql, buildSelectSql, buildUpdateSql} from "../dao.util";

const TABLE_NAME = 'youi_geo_json';

export const PROPERTIES = ['area_id','geo_json','version','rect','center','create_time','update_time','creator','modified_by'];

export const SQL_SELECT = buildSelectSql(TABLE_NAME,PROPERTIES);

export const SQL_FIND = `${SQL_SELECT} where area_id=$1`;

export const SQL_INSERT =  buildInsertSql(TABLE_NAME,PROPERTIES);

export const SQL_UPDATE = buildUpdateSql(TABLE_NAME,PROPERTIES.slice(1),PROPERTIES[0]);

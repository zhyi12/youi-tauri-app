import {appDataBase} from "../../app-dao/dao.util";
import {findByPager} from "../service.util";
import {
    PROPERTIES,
    INSERT_PROPERTIES,
    SQL_FIND,SQL_FIND_BY_GROUP,
    SQL_SELECT,
    SQL_INSERT,
    SQL_REMOVE,
    SQL_UPDATE
} from "../../app-dao/dmp/dashboard.sql";

import type {Pager} from "../../app-dao/dao";
import type {Dashboard} from "../../app-entity/dmp/dashboard";

/**
 * 主键查询
 * @param id
 */
export async function findDashboard(id:number|string):Promise<Dashboard|null>{
    const dashboards = await appDataBase.select<Dashboard[]>(SQL_FIND,[id]);
    if(dashboards.length){
        return dashboards[0];
    }
    return null;
}

/**
 * 主键查询
 * @param id
 */
export async function findByGroup(group_id:number):Promise<[Dashboard]>{
    const dashboards = await appDataBase.select<Dashboard[]>(SQL_FIND_BY_GROUP,[group_id]);
    return dashboards;
}

/**
 * 插入数据看板
 * @param Dashboard
 */
export async function insertDashboard(dashboard:Dashboard):Promise<number|string>{
    // eslint-disable-next-line @typescript-eslint/ban-ts-comment
    // @ts-ignore
    const bindValues= INSERT_PROPERTIES.map(prop=>dashboard[prop]||'');
    //
    const result = await appDataBase.execute(SQL_INSERT,bindValues);
    return result.lastInsertId;
}

/**
 * 更新数据看板
 * @param Dashboard
 */
export async function updateDashboard(dashboard:Dashboard):Promise<number>{
    // eslint-disable-next-line @typescript-eslint/ban-ts-comment
    // @ts-ignore
    const bindValues= PROPERTIES.map(prop=>dashboard[prop]||'');
    //
    const result = await appDataBase.execute(SQL_UPDATE,bindValues);
    return result.rowsAffected;
}

/**
 * 删除数据看板
 * @param Dashboard
 */
export async function removeDashboard(id:number):Promise<number>{
    const result = await appDataBase.execute(SQL_REMOVE,[id]);
    return result.rowsAffected;
}


/**
 * 分页查询
 * @param pager
 * @param dashboard
 */
export async function findDashboardByPager(pager:Pager,dashboard?:Dashboard) {
    return await findByPager(pager,SQL_SELECT,dashboard)
}
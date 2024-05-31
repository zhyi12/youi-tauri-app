import {appDataBase} from "../../app-dao/dao.util";

import {
    PROPERTIES,
    INSERT_PROPERTIES,
    SQL_INSERT,
    SQL_REMOVE,
    SQL_UPDATE,
    SQL_UPDATE_TEXT,
    SQL_UPDATE_MOVE,
    SQL_FIND,
    SQL_TREE,
    SQL_FIND_PREV,
    SQL_TOP_FIND_PREV,
    SQL_FIND_NEXT,
    SQL_TOP_FIND_NEXT
} from "../../app-dao/dmp/dashboardGroup.sql";

import type {DashboardGroup} from "../../app-entity/dmp/dashboardGroup";

/**
 * 主键查询
 * @param id
 */
export async function findDashboardGroup(id:number|string):Promise<DashboardGroup|null>{
    const dashboardGroups = await appDataBase.select<DashboardGroup[]>(SQL_FIND,[id]);
    if(dashboardGroups.length){
        return dashboardGroups[0];
    }
    return null;
}

/**
 * 获取下一个节点
 * @param id
 * @param pid
 * @param num
 */
export async function findNextNode(id:number,pid:number|string|undefined,num:number):Promise<DashboardGroup|null>{
    let dashboardGroups;
    if(pid){
        //非顶级查找下一个节点
        const result = await appDataBase.select<DashboardGroup[]>(SQL_FIND_NEXT,[pid,num]);
        dashboardGroups = [...result];
    }else{
        //顶级查找下一个节点
        const result = await appDataBase.select<DashboardGroup[]>(SQL_TOP_FIND_NEXT,[num]);
        dashboardGroups = [...result];
    }

    if(dashboardGroups.length){
        dashboardGroups = dashboardGroups.filter((DashboardGroup)=>DashboardGroup.id != id);
        return dashboardGroups[0];
    }
    return null;
}

/**
 * 获取下一个节点
 * @param id
 * @param pid
 * @param num
 */
export async function findPrevNode(id:number,pid:number|string|undefined,num:number):Promise<DashboardGroup|null>{
    let dashboardGroups;
    if(pid){
        const result = await appDataBase.select<DashboardGroup[]>(SQL_FIND_PREV,[pid,num]);
        dashboardGroups = [...result];
    }else{
        const result = await appDataBase.select<DashboardGroup[]>(SQL_TOP_FIND_PREV,[num]);
        dashboardGroups = [...result];
    }

    if(dashboardGroups.length){
        dashboardGroups = dashboardGroups.filter((dashboardGroup)=>dashboardGroup.id != id);
        return dashboardGroups[0];
    }
    return null;
}

/**
 * 树查询
 * @param id
 */
export async function findDashboardGroupTree():Promise<DashboardGroup[]>{
    const dashboardGroups = await appDataBase.select<DashboardGroup[]>(SQL_TREE,[]);
    return dashboardGroups;
}

/**
 * 插入数据看版分组
 * @param DashboardGroup
 */
export async function insertDashboardGroup(dashboardGroup:DashboardGroup):Promise<number|string>{
    // eslint-disable-next-line @typescript-eslint/ban-ts-comment
    // @ts-ignore
    const bindValues= INSERT_PROPERTIES.map(prop=>dashboardGroup[prop]||'');
    //
    const result = await appDataBase.execute(SQL_INSERT,bindValues);
    return result.lastInsertId;
}

/**
 * 更新插入数据看版分组
 * @param DashboardGroup
 */
export async function updateDashboardGroup(dashboardGroup:DashboardGroup):Promise<number>{
    // eslint-disable-next-line @typescript-eslint/ban-ts-comment
    // @ts-ignore
    const bindValues= PROPERTIES.map(prop=>dashboardGroup[prop]||'');
    //
    const result = await appDataBase.execute(SQL_UPDATE,bindValues);
    return result.rowsAffected;
}

/**
 * 重命名
 * @param id
 * @param text
 */
export async function updateDashboardGroupText(id:number,text:string):Promise<number>{
    const result = await appDataBase.execute(SQL_UPDATE_TEXT,[id,text]);
    return result.rowsAffected;
}

/**
 * 节点移动后更新pid及num
 * @param id
 * @param pid
 * @param num
 */
export async function updateDashboardGroupMove(id:number,pid:number|undefined,num:number):Promise<number>{
    const result = await appDataBase.execute(SQL_UPDATE_MOVE,[id,pid,num]);
    return result.rowsAffected;
}

/**
 * 删除数据看版分组
 * @param DashboardGroup
 */
export async function removeDashboardGroup(id:number):Promise<number>{
    const result = await appDataBase.execute(SQL_REMOVE,[id]);
    return result.rowsAffected;
}

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
} from "../../app-dao/base/area.sql";

import type {Area} from "../../app-entity/base/area";

const AREA_REG = /(^[0-9]{2})([0-9]{2})([0-9]{2})([0-9]{3})([0-9]{3}$)/g;

export const AREA_LEVEL_COUNTRY = 1;//国家
export const AREA_LEVEL_PROVINCE = 2;//省
export const AREA_LEVEL_CITY = 3;//市
export const AREA_LEVEL_COUNTY= 4;//县
export const AREA_LEVEL_STREET = 5;//街道
export const AREA_LEVEL_VILLAGE = 6;//居委会

/**
 * 主键查询
 * @param id
 */
export async function findArea(id:number|string):Promise<Area|null>{
    const areas = await appDataBase.select<Area[]>(SQL_FIND,[id]);
    if(areas.length){
        return areas[0];
    }
    return null;
}

/**
 * 获取下一个节点
 * @param id
 * @param pid
 * @param num
 */
export async function findNextNode(id:number,pid:number|string|undefined,num:number):Promise<Area|null>{
    let areas;
    if(pid){
        //非顶级查找下一个节点
        const result = await appDataBase.select<Area[]>(SQL_FIND_NEXT,[pid,num]);
        areas = [...result];
    }else{
        //顶级查找下一个节点
        const result = await appDataBase.select<Area[]>(SQL_TOP_FIND_NEXT,[num]);
        areas = [...result];
    }

    if(areas.length){
        areas = areas.filter((Area)=>Area.id != id);
        return areas[0];
    }
    return null;
}

/**
 * 获取下一个节点
 * @param id
 * @param pid
 * @param num
 */
export async function findPrevNode(id:number,pid:number|string|undefined,num:number):Promise<Area|null>{
    let areas;
    if(pid){
        const result = await appDataBase.select<Area[]>(SQL_FIND_PREV,[pid,num]);
        areas = [...result];
    }else{
        const result = await appDataBase.select<Area[]>(SQL_TOP_FIND_PREV,[num]);
        areas = [...result];
    }

    if(areas.length){
        areas = areas.filter((area)=>area.id != id);
        return areas[0];
    }
    return null;
}

/**
 * 树查询
 * @param id
 */
export async function findAreaTree(pid,maxLevel):Promise<Area[]>{
    const areas = await appDataBase.select<Area[]>(SQL_TREE,[pid,maxLevel]);
    return areas;
}

/**
 * 插入行政区划
 * @param Area
 */
export async function insertArea(area:Area):Promise<number|string>{
    // eslint-disable-next-line @typescript-eslint/ban-ts-comment
    // @ts-ignore
    const bindValues= INSERT_PROPERTIES.map(prop=>area[prop]||'');
    //
    const result = await appDataBase.execute(SQL_INSERT,bindValues);
    return result.lastInsertId;
}

/**
 * 更新插入行政区划
 * @param Area
 */
export async function updateArea(area:Area):Promise<number>{
    // eslint-disable-next-line @typescript-eslint/ban-ts-comment
    // @ts-ignore
    const bindValues= PROPERTIES.map(prop=>area[prop]||'');
    //
    const result = await appDataBase.execute(SQL_UPDATE,bindValues);
    return result.rowsAffected;
}

/**
 * 重命名
 * @param id
 * @param text
 */
export async function updateAreaText(id:number,text:string):Promise<number>{
    const result = await appDataBase.execute(SQL_UPDATE_TEXT,[id,text]);
    return result.rowsAffected;
}

/**
 * 节点移动后更新pid及num
 * @param id
 * @param pid
 * @param num
 */
export async function updateAreaMove(id:string,pid:string|undefined,num:number):Promise<number>{
    const result = await appDataBase.execute(SQL_UPDATE_MOVE,[id,pid,num]);
    return result.rowsAffected;
}

/**
 * 删除行政区划
 * @param Area
 */
export async function removeArea(id:string):Promise<number>{
    const result = await appDataBase.execute(SQL_REMOVE,[id]);
    return result.rowsAffected;
}

/**
 * 获取行政级别
 * 1-国家 2-省 3-市 4-区 5-街道 6-居委会
 * @param areaId
 */
export function findAreaLevel(areaId:string){
    const prefixes = areaPrefixes(areaId);
    return prefixes.length+1;
}

/**
 * 返回行政区划代码前缀
 * @param areaId
 */
export function findAreaPrefix(areaId:string) {
    return areaPrefixes(areaId).join('');
}
/**
 *
 * @param areaId
 */
export function areaPrefixes(areaId:string) {
    const parts = areaId.split(AREA_REG).filter(s=>!!s);
    let sliceIndex = parts.length;
    for(let i=0;i<parts.length;i++){
        if(parseInt(parts[parts.length-i-1])!==0){
            sliceIndex=parts.length-i;
            break;
        }
    }
    return parts.slice(0,sliceIndex);
}
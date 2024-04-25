import {
    appDataBase
} from "../../app-dao/dao.util";

import {
    PROPERTIES,
    INSERT_PROPERTIES,
    SQL_FIND,
    SQL_SELECT,
    SQL_INSERT,
    SQL_REMOVE,
    SQL_UPDATE
} from "../../app-dao/base/desktopItem.sql";

import {findByPager} from "../service.util";

import type {Pager} from "../../app-dao/dao";
import type {DesktopItem} from "../../app-entity/base/desktopItem";


/**
 * 主键查询
 * @param id
 */
export async function findDesktopItem(id:number|string):Promise<DesktopItem|null>{
    const desktopItems = await appDataBase.select<DesktopItem[]>(SQL_FIND,[id]);
    if(desktopItems.length){
        return desktopItems[0];
    }
    return null;
}

/**
 * 插入桌面项
 * @param DesktopItem
 */
export async function insertDesktopItem(desktopItem:DesktopItem):Promise<number|string>{
    // eslint-disable-next-line @typescript-eslint/ban-ts-comment
    // @ts-ignore
    const bindValues= INSERT_PROPERTIES.map(prop=>desktopItem[prop]||'');
    //
    const result = await appDataBase.execute(SQL_INSERT,bindValues);
    return result.lastInsertId;
}

/**
 * 更新桌面项
 * @param DesktopItem
 */
export async function updateDesktopItem(desktopItem:DesktopItem):Promise<number>{
    // eslint-disable-next-line @typescript-eslint/ban-ts-comment
    // @ts-ignore
    const bindValues= PROPERTIES.map(prop=>desktopItem[prop]||'');
    //
    const result = await appDataBase.execute(SQL_UPDATE,bindValues);
    return result.rowsAffected;
}

/**
 * 删除桌面项
 * @param DesktopItem
 */
export async function removeDesktopItem(id:number):Promise<number>{
    const result = await appDataBase.execute(SQL_REMOVE,[id]);
    return result.rowsAffected;
}


/**
 * 分页查询
 * @param pager
 * @param desktopItem
 */
export async function findByPager(pager:Pager,desktopItem?:DesktopItem) {
    return findByPager(pager,SQL_SELECT,desktopItem)
}
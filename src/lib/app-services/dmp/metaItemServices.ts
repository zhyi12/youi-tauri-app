import {appDataBase} from "../../app-dao/dao.util";

import {
    PROPERTIES,
    INSERT_PROPERTIES,
    SQL_FIND,
    SQL_SELECT,
    SQL_INSERT,
    SQL_REMOVE,
    SQL_UPDATE
} from "../../app-dao/dmp/metaItem.sql";
import {findByPager} from "../service.util";

import type {Pager} from "../../app-dao/dao";
import type {MetaItem} from "../../app-entity/dmp/metaItem";

/**
 * 主键查询
 * @param id
 */
export async function findMetaItem(id:number|string):Promise<MetaItem|null>{
    const metaItems = await appDataBase.select<MetaItem[]>(SQL_FIND,[id]);
    if(metaItems.length){
        return metaItems[0];
    }
    return null;
}

/**
 * 插入桌面项
 * @param MetaItem
 */
export async function insertMetaItem(metaItem:MetaItem):Promise<number|string>{
    // eslint-disable-next-line @typescript-eslint/ban-ts-comment
    // @ts-ignore
    const bindValues= INSERT_PROPERTIES.map(prop=>metaItem[prop]||'');
    //
    const result = await appDataBase.execute(SQL_INSERT,bindValues);
    return result.lastInsertId;
}

/**
 * 更新桌面项
 * @param MetaItem
 */
export async function updateMetaItem(metaItem:MetaItem):Promise<number>{
    // eslint-disable-next-line @typescript-eslint/ban-ts-comment
    // @ts-ignore
    const bindValues= PROPERTIES.map(prop=>metaItem[prop]||'');
    //
    console.log(SQL_UPDATE,bindValues)
    const result = await appDataBase.execute(SQL_UPDATE,bindValues);
    return result.rowsAffected;
}

/**
 * 删除桌面项
 * @param MetaItem
 */
export async function removeMetaItem(id:number):Promise<number>{
    const result = await appDataBase.execute(SQL_REMOVE,[id]);
    return result.rowsAffected;
}


/**
 * 分页查询
 * @param pager
 * @param metaItem
 */
export async function findMetaItemByPager(pager:Pager,metaItem?:MetaItem) {
    return await findByPager(pager,SQL_SELECT,metaItem)
}
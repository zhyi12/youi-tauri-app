import {appDataBase} from "../../app-dao/dao.util";
import {findByPager} from "../service.util";
import {
    PROPERTIES,
    INSERT_PROPERTIES,
    SQL_FIND,
    SQL_SELECT,
    SQL_INSERT,
    SQL_REMOVE,
    SQL_UPDATE
} from "../../app-dao/datamacro/macroGroup.sql";

import type {Pager} from "../../app-dao/dao";
import type {MacroGroup} from "../../app-entity/datamacro/macroGroup";

/**
 * 主键查询
 * @param id
 */
export async function findMacroGroup(id:number|string):Promise<MacroGroup|null>{
    const macroGroups = await appDataBase.select<MacroGroup[]>(SQL_FIND,[id]);
    if(macroGroups.length){
        return macroGroups[0];
    }
    return null;
}

/**
 * 插入宏观分组
 * @param MacroGroup
 */
export async function insertMacroGroup(macroGroup:MacroGroup):Promise<number|string>{
    // eslint-disable-next-line @typescript-eslint/ban-ts-comment
    // @ts-ignore
    const bindValues= INSERT_PROPERTIES.map(prop=>macroGroup[prop]||'');
    //
    const result = await appDataBase.execute(SQL_INSERT,bindValues);
    return result.lastInsertId;
}

/**
 * 更新宏观分组
 * @param MacroGroup
 */
export async function updateMacroGroup(macroGroup:MacroGroup):Promise<number>{
    // eslint-disable-next-line @typescript-eslint/ban-ts-comment
    // @ts-ignore
    const bindValues= PROPERTIES.map(prop=>macroGroup[prop]||'');
    //
    const result = await appDataBase.execute(SQL_UPDATE,bindValues);
    return result.rowsAffected;
}

/**
 * 删除宏观分组
 * @param MacroGroup
 */
export async function removeMacroGroup(id:number):Promise<number>{
    const result = await appDataBase.execute(SQL_REMOVE,[id]);
    return result.rowsAffected;
}


/**
 * 分页查询
 * @param pager
 * @param macroGroup
 */
export async function findMacroGroupByPager(pager:Pager,macroGroup?:MacroGroup) {
    return await findByPager(pager,SQL_SELECT,macroGroup)
}
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
} from "../../app-dao/datamacro/macroGroupItem.sql";

import type {MacroGroupItem} from "../../app-entity/datamacro/macroGroupItem";

/**
 * 主键查询
 * @param id
 */
export async function findMacroGroupItem(id:number|string):Promise<MacroGroupItem|null>{
    const macroGroupItems = await appDataBase.select<MacroGroupItem[]>(SQL_FIND,[id]);
    if(macroGroupItems.length){
        return macroGroupItems[0];
    }
    return null;
}

/**
 * 获取下一个节点
 * @param id
 * @param pid
 * @param num
 */
export async function findNextNode(id:number,pid:number|string|undefined,num:number,group_id:number):Promise<MacroGroupItem|null>{
    let macroGroupItems;
    if(pid){
        //非顶级查找下一个节点
        const result = await appDataBase.select<MacroGroupItem[]>(SQL_FIND_NEXT,[pid,num]);
        macroGroupItems = [...result];
    }else{
        //顶级查找下一个节点
        const result = await appDataBase.select<MacroGroupItem[]>(SQL_TOP_FIND_NEXT,[num,group_id]);
        macroGroupItems = [...result];
    }

    if(macroGroupItems.length){
        macroGroupItems = macroGroupItems.filter((MacroGroupItem)=>MacroGroupItem.id != id);
        return macroGroupItems[0];
    }
    return null;
}

/**
 * 获取下一个节点
 * @param id
 * @param pid
 * @param num
 */
export async function findPrevNode(id:number,pid:number|string|undefined,num:number,group_id:number):Promise<MacroGroupItem|null>{
    let macroGroupItems;
    if(pid){
        const result = await appDataBase.select<MacroGroupItem[]>(SQL_FIND_PREV,[pid,num]);
        macroGroupItems = [...result];
    }else{
        const result = await appDataBase.select<MacroGroupItem[]>(SQL_TOP_FIND_PREV,[num,group_id]);
        macroGroupItems = [...result];
    }

    if(macroGroupItems.length){
        macroGroupItems = macroGroupItems.filter((macroGroupItem)=>macroGroupItem.id != id);
        return macroGroupItems[0];
    }
    return null;
}

/**
 * 树查询
 * @param id
 */
export async function findMacroGroupItemTree(group_id:number):Promise<MacroGroupItem[]>{
    const macroGroupItems = await appDataBase.select<MacroGroupItem[]>(SQL_TREE,[group_id]);
    return macroGroupItems;
}

/**
 * 插入分组项
 * @param MacroGroupItem
 */
export async function insertMacroGroupItem(macroGroupItem:MacroGroupItem):Promise<number|string>{
    // eslint-disable-next-line @typescript-eslint/ban-ts-comment
    // @ts-ignore
    const bindValues= INSERT_PROPERTIES.map(prop=>macroGroupItem[prop]||'');
    //
    const result = await appDataBase.execute(SQL_INSERT,bindValues);
    return result.lastInsertId;
}

/**
 * 更新插入分组项
 * @param MacroGroupItem
 */
export async function updateMacroGroupItem(macroGroupItem:MacroGroupItem):Promise<number>{
    // eslint-disable-next-line @typescript-eslint/ban-ts-comment
    // @ts-ignore
    const bindValues= PROPERTIES.map(prop=>macroGroupItem[prop]||'');
    //
    const result = await appDataBase.execute(SQL_UPDATE,bindValues);
    return result.rowsAffected;
}

/**
 * 重命名
 * @param id
 * @param text
 */
export async function updateMacroGroupItemText(id:number,text:string):Promise<number>{
    const result = await appDataBase.execute(SQL_UPDATE_TEXT,[id,text]);
    return result.rowsAffected;
}

/**
 * 节点移动后更新pid及num
 * @param id
 * @param pid
 * @param num
 */
export async function updateMacroGroupItemMove(id:number,pid:number|undefined,num:number):Promise<number>{
    const result = await appDataBase.execute(SQL_UPDATE_MOVE,[id,pid,num]);
    return result.rowsAffected;
}

/**
 * 删除分组项
 * @param MacroGroupItem
 */
export async function removeMacroGroupItem(id:number):Promise<number>{
    const result = await appDataBase.execute(SQL_REMOVE,[id]);
    return result.rowsAffected;
}

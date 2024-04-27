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
} from "../../app-dao/datamacro/macroIndicator.sql";

import type {MacroIndicator} from "../../app-entity/datamacro/macroIndicator";

/**
 * 主键查询
 * @param id
 */
export async function findMacroIndicator(id:number|string):Promise<MacroIndicator|null>{
    const macroIndicators = await appDataBase.select<MacroIndicator[]>(SQL_FIND,[id]);
    if(macroIndicators.length){
        return macroIndicators[0];
    }
    return null;
}

/**
 * 获取下一个节点
 * @param id
 * @param pid
 * @param num
 */
export async function findNextNode(id:number,pid:number|string|undefined,num:number):Promise<MacroIndicator|null>{
    let macroIndicators;
    if(pid){
        //非顶级查找下一个节点
        const result = await appDataBase.select<MacroIndicator[]>(SQL_FIND_NEXT,[pid,num]);
        macroIndicators = [...result];
    }else{
        //顶级查找下一个节点
        const result = await appDataBase.select<MacroIndicator[]>(SQL_TOP_FIND_NEXT,[num]);
        macroIndicators = [...result];
    }

    if(macroIndicators.length){
        macroIndicators = macroIndicators.filter((macroIndicator)=>macroIndicator.id != id);
        return macroIndicators[0];
    }
    return null;
}

/**
 * 获取下一个节点
 * @param id
 * @param pid
 * @param num
 */
export async function findPrevNode(id:number,pid:number|string|undefined,num:number):Promise<MacroIndicator|null>{
    let macroIndicators;
    if(pid){
        const result = await appDataBase.select<MacroIndicator[]>(SQL_FIND_PREV,[pid,num]);
        macroIndicators = [...result];
    }else{
        const result = await appDataBase.select<MacroIndicator[]>(SQL_TOP_FIND_PREV,[num]);
        macroIndicators = [...result];
    }

    if(macroIndicators.length){
        macroIndicators = macroIndicators.filter((macroIndicator)=>macroIndicator.id != id);
        return macroIndicators[0];
    }
    return null;
}

/**
 * 树查询
 * @param id
 */
export async function findMacroIndicatorTree():Promise<MacroIndicator[]>{
    const macroIndicators = await appDataBase.select<MacroIndicator[]>(SQL_TREE,[]);
    return macroIndicators;
}

/**
 * 插入宏观指标
 * @param MacroIndicator
 */
export async function insertMacroIndicator(macroIndicator:MacroIndicator):Promise<number>{
    // eslint-disable-next-line @typescript-eslint/ban-ts-comment
    // @ts-ignore
    const bindValues= INSERT_PROPERTIES.map(prop=>macroIndicator[prop]||'');
    //
    const result = await appDataBase.execute(SQL_INSERT,bindValues);
    return result.lastInsertId;
}

/**
 * 更新插入宏观指标
 * @param MacroIndicator
 */
export async function updateMacroIndicator(macroIndicator:MacroIndicator):Promise<number>{
    // eslint-disable-next-line @typescript-eslint/ban-ts-comment
    // @ts-ignore
    const bindValues= PROPERTIES.map(prop=>macroIndicator[prop]||'');
    //
    const result = await appDataBase.execute(SQL_UPDATE,bindValues);
    return result.rowsAffected;
}

/**
 * 重命名
 * @param id
 * @param text
 */
export async function updateMacroIndicatorText(id:number,text:string):Promise<number>{
    const result = await appDataBase.execute(SQL_UPDATE_TEXT,[id,text]);
    return result.rowsAffected;
}

/**
 * 节点移动后更新pid及num
 * @param id
 * @param pid
 * @param num
 */
export async function updateMacroIndicatorMove(id:number,pid:number|undefined,num:number):Promise<number>{
    const result = await appDataBase.execute(SQL_UPDATE_MOVE,[id,pid,num]);
    return result.rowsAffected;
}

/**
 * 删除宏观指标
 * @param MacroIndicator
 */
export async function removeMacroIndicator(id:number):Promise<number>{
    const result = await appDataBase.execute(SQL_REMOVE,[id]);
    return result.rowsAffected;
}


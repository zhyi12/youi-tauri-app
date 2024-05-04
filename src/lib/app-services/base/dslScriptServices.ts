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
    SQL_TOP_FIND_NEXT,
    SQL_FIND_CONTENT,
    SQL_UPDATE_CONTENT,
    SQL_UPDATE_COL_WIDTHS
} from "../../app-dao/base/dslScript.sql";

import type {DslScript,DslScriptContent} from "../../app-entity/base/dslScript";

/**
 * 主键查询
 * @param id
 */
export async function findDslScript(id:number|string):Promise<DslScript|null>{
    const dslScripts = await appDataBase.select<DslScript[]>(SQL_FIND,[id]);
    if(dslScripts.length){
        return dslScripts[0];
    }
    return null;
}

/**
 * 获取下一个节点
 * @param id
 * @param pid
 * @param num
 */
export async function findNextNode(id:number,pid:number|string|undefined,num:number):Promise<DslScript|null>{
    let dslScripts;
    if(pid){
        //非顶级查找下一个节点
        const result = await appDataBase.select<DslScript[]>(SQL_FIND_NEXT,[pid,num]);
        dslScripts = [...result];
    }else{
        //顶级查找下一个节点
        const result = await appDataBase.select<DslScript[]>(SQL_TOP_FIND_NEXT,[num]);
        dslScripts = [...result];
    }

    if(dslScripts.length){
        dslScripts = dslScripts.filter((DslScript)=>DslScript.id != id);
        return dslScripts[0];
    }
    return null;
}

/**
 * 获取下一个节点
 * @param id
 * @param pid
 * @param num
 */
export async function findPrevNode(id:number,pid:number|string|undefined,num:number):Promise<DslScript|null>{
    let dslScripts;
    if(pid){
        const result = await appDataBase.select<DslScript[]>(SQL_FIND_PREV,[pid,num]);
        dslScripts = [...result];
    }else{
        const result = await appDataBase.select<DslScript[]>(SQL_TOP_FIND_PREV,[num]);
        dslScripts = [...result];
    }

    if(dslScripts.length){
        dslScripts = dslScripts.filter((dslScript)=>dslScript.id != id);
        return dslScripts[0];
    }
    return null;
}

/**
 * 树查询
 * @param id
 */
export async function findDslScriptTree():Promise<DslScript[]>{
    const dslScripts = await appDataBase.select<DslScript[]>(SQL_TREE,[]);
    return dslScripts;
}

/**
 * 插入DSL脚本
 * @param DslScript
 */
export async function insertDslScript(dslScript:DslScript):Promise<number|string>{
    // eslint-disable-next-line @typescript-eslint/ban-ts-comment
    // @ts-ignore
    const bindValues= INSERT_PROPERTIES.map(prop=>dslScript[prop]||'');
    //
    const result = await appDataBase.execute(SQL_INSERT,bindValues);
    return result.lastInsertId;
}

/**
 * 更新插入DSL脚本
 * @param DslScript
 */
export async function updateDslScript(dslScript:DslScript):Promise<number>{
    // eslint-disable-next-line @typescript-eslint/ban-ts-comment
    // @ts-ignore
    const bindValues= PROPERTIES.map(prop=>dslScript[prop]||'');
    //
    const result = await appDataBase.execute(SQL_UPDATE,bindValues);
    return result.rowsAffected;
}

/**
 * 重命名
 * @param id
 * @param text
 */
export async function updateDslScriptText(id:number,text:string):Promise<number>{
    const result = await appDataBase.execute(SQL_UPDATE_TEXT,[id,text]);
    return result.rowsAffected;
}

/**
 * 节点移动后更新pid及num
 * @param id
 * @param pid
 * @param num
 */
export async function updateDslScriptMove(id:number,pid:number|undefined,num:number):Promise<number>{
    const result = await appDataBase.execute(SQL_UPDATE_MOVE,[id,pid,num]);
    return result.rowsAffected;
}

/**
 * 删除DSL脚本
 * @param DslScript
 */
export async function removeDslScript(id:number):Promise<number>{
    const result = await appDataBase.execute(SQL_REMOVE,[id]);
    return result.rowsAffected;
}


/**
 * 主键查询
 * @param id
 */
export async function findDslScriptContent(id:number|string):Promise<DslScriptContent|null>{
    const dslScripts = await appDataBase.select<DslScriptContent[]>(SQL_FIND_CONTENT,[id]);
    if(dslScripts.length){
        return dslScripts[0];
    }
    return null;
}


export async function updateDslScriptContent(id:number,content:string):Promise<number>{
    const result = await appDataBase.execute(SQL_UPDATE_CONTENT,[id,content,new Date().getTime()]);
    return result.rowsAffected;
}

/**
 * 更新列宽度
 * @param id
 * @param colWidths
 */
export async function updateColWidths(id:number,colWidths:number[]):Promise<number>{
    const result = await appDataBase.execute(SQL_UPDATE_COL_WIDTHS,[id,colWidths.join()]);
    return result.rowsAffected;
}
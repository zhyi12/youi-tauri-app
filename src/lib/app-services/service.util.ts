
import {
    appDataBase,
    buildConditions,
    buildConditionsSql,
    buildLimitParamValues,
    wrapSqlCount, wrapSqlLimit
} from "../app-dao/dao.util";

import type {TreeItem} from "../youi/index.d";
import type {TreeObject} from "../app-entity/entity";
import type {Pager} from "../app-dao/dao";
import {findTreeNode, removeTreeNode} from "../youi/util/tree.util";

/**
 * 通用分页查询
 * @param pager
 * @param selectSql
 * @param entity
 */
export async function findByPager<T>(pager:Pager,selectSql,entity?:T) {
    const conditions = buildConditions(entity);
    const querySql = conditions.length?`${selectSql} where ${buildConditionsSql(conditions)}`:selectSql;
    let bindValues:any[] = conditions.map(condition=>condition.value);
    //获取记录总数
    const countResult = await appDataBase.select<{count:number}[]>(wrapSqlCount(querySql),bindValues);
    let totalCount = 0;
    if(countResult){
        totalCount = countResult[0].count;
    }
    //分页参数
    bindValues = bindValues.concat(buildLimitParamValues(pager));

    const records = await appDataBase.select<T[]>(wrapSqlLimit(querySql,conditions.length),bindValues);

    return {
        totalCount,
        records
    }
}

/**
 *
 */
export function levelListToTree<T extends TreeObject>(list:T[],nodeProcessor?:(TreeItem,T)=>void):TreeItem[]{
    if(list.length === 0 )return [];

    const treeNodes:TreeItem[] = [];
    const levelMaps:Record<String,TreeItem> = {};
    const topLevel = list[0].level;

    for(let i=0;i<list.length;i++){
        const obj = list[i];
        const treeItem:TreeItem = {
            id:obj.id.toString(),
            text:obj.text,
            datas:{
                dataId:obj.id,
                num:obj.num
            }
        }

        if(nodeProcessor){
            nodeProcessor(treeItem,obj);
        }

        const levelKey = buildLevelKey(obj.level||1);

        levelMaps[levelKey] = treeItem;

        if(topLevel === obj.level){
            treeNodes.push(treeItem);
        }else{
            const parentLevelKey = buildLevelKey((obj.level||1)-1);
            const parentNode:TreeItem = levelMaps[parentLevelKey];
            if(parentNode){
                parentNode.children = parentNode.children||[];
                parentNode.children.push(treeItem);
            }
        }
    }

    return treeNodes
}



/**
 *
 */
export function moveTreeNode(treeNodes:TreeItem[],source:TreeItem,target:TreeItem,position:string,newNum:number){
    //
    const sourceTreeNode = findTreeNode(treeNodes,source.id);
    // sourceTreeNode.datas.num = newNum;
    // removeTreeNode(treeNodes,source.id,null);
    // const pathNodes = findPathNodes(treeNodes,target.id,[]);
    //
    // if('before' === position || 'after' === position){
    //     if(pathNodes.length>1){
    //         const parent = pathNodes[pathNodes.length-2];
    //         parent.children = parent.children ||[];
    //         parent.children.push(sourceTreeNode);
    //     }else{
    //         treeNodes.push(sourceTreeNode);
    //     }
    // }else{
    //     const parent = pathNodes.pop();
    //     parent.children = parent.children ||[];
    //     parent.children.push(sourceTreeNode);
    // }
}

function buildLevelKey(level:number) {
    return `L${level}`;
}
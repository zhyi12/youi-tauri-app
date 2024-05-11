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
    SQL_TOP_FIND_NEXT, SQL_FIND_BY_METADATA
} from "../../app-dao/metadata/surveyPlanTree.sql";

import type {MetaNode} from "../../app-entity/metadata/surveyPlanTree";

/**
 *
 * @param object_name
 * @param object_id
 */
export async function findByMetaObject(object_name,object_id):Promise<MetaNode|undefined>{
    const metaNodes = await appDataBase.select<MetaNode[]>(SQL_FIND_BY_METADATA,[object_name,object_id]);
    if(metaNodes.length){
        return metaNodes[0]
    }
}

export async function findOrCreatePlanNode(plan_id,plan_text){
    const planNode = await findByMetaObject('plan',plan_id);
    if(!planNode){
        const time = new Date().getTime();
        const newPlanNode:MetaNode = <MetaNode>{
            object_id: plan_id, object_name: "plan",
            text: plan_text,
            num:1,
            create_time:time,
            update_time:time
        };
        const id = await insertMetaNode(newPlanNode);
        return  {...newPlanNode,id };
    }else{
        return planNode
    }
}
/**
 * 主键查询
 * @param id
 */
export async function findMetaNode(id:number|string):Promise<MetaNode|null>{
    const MetaNodes = await appDataBase.select<MetaNode[]>(SQL_FIND,[id]);
    if(MetaNodes.length){
        return MetaNodes[0];
    }
    return null;
}

/**
 * 获取下一个节点
 * @param id
 * @param pid
 * @param num
 */
export async function findNextNode(id:number,pid:number|string|undefined,num:number):Promise<MetaNode|null>{
    let MetaNodes;
    if(pid){
        //非顶级查找下一个节点
        const result = await appDataBase.select<MetaNode[]>(SQL_FIND_NEXT,[pid,num]);
        MetaNodes = [...result];
    }else{
        //顶级查找下一个节点
        const result = await appDataBase.select<MetaNode[]>(SQL_TOP_FIND_NEXT,[num]);
        MetaNodes = [...result];
    }

    if(MetaNodes.length){
        MetaNodes = MetaNodes.filter((MetaNode)=>MetaNode.id != id);
        return MetaNodes[0];
    }
    return null;
}

/**
 * 获取下一个节点
 * @param id
 * @param pid
 * @param num
 */
export async function findPrevNode(id:number,pid:number|string|undefined,num:number):Promise<MetaNode|null>{
    let MetaNodes;
    if(pid){
        const result = await appDataBase.select<MetaNode[]>(SQL_FIND_PREV,[pid,num]);
        MetaNodes = [...result];
    }else{
        const result = await appDataBase.select<MetaNode[]>(SQL_TOP_FIND_PREV,[num]);
        MetaNodes = [...result];
    }

    if(MetaNodes.length){
        MetaNodes = MetaNodes.filter((MetaNode)=>MetaNode.id != id);
        return MetaNodes[0];
    }
    return null;
}

/**
 * 树查询
 * @param id
 */
export async function findMetaNodeTree({rootId}):Promise<MetaNode[]>{
    const MetaNodes = await appDataBase.select<MetaNode[]>(SQL_TREE,[rootId]);
    return MetaNodes;
}

/**
 * 插入制度树
 * @param MetaNode
 */
export async function insertMetaNode(MetaNode:MetaNode):Promise<number|string>{
    // eslint-disable-next-line @typescript-eslint/ban-ts-comment
    // @ts-ignore
    const bindValues= INSERT_PROPERTIES.map(prop=>MetaNode[prop]||'');
    //
    const result = await appDataBase.execute(SQL_INSERT,bindValues);
    return result.lastInsertId;
}

/**
 * 更新插入制度树
 * @param MetaNode
 */
export async function updateMetaNode(MetaNode:MetaNode):Promise<number>{
    // eslint-disable-next-line @typescript-eslint/ban-ts-comment
    // @ts-ignore
    const bindValues= PROPERTIES.map(prop=>MetaNode[prop]||'');
    //
    const result = await appDataBase.execute(SQL_UPDATE,bindValues);
    return result.rowsAffected;
}

/**
 * 重命名
 * @param id
 * @param text
 */
export async function updateMetaNodeText(id:number,text:string):Promise<number>{
    const result = await appDataBase.execute(SQL_UPDATE_TEXT,[id,text]);
    return result.rowsAffected;
}

/**
 * 节点移动后更新pid及num
 * @param id
 * @param pid
 * @param num
 */
export async function updateMetaNodeMove(id:number,pid:number|undefined,num:number):Promise<number>{
    const result = await appDataBase.execute(SQL_UPDATE_MOVE,[id,pid,num]);
    return result.rowsAffected;
}

/**
 * 删除制度树
 * @param MetaNode
 */
export async function removeMetaNode(id:number):Promise<number>{
    const result = await appDataBase.execute(SQL_REMOVE,[id]);
    return result.rowsAffected;
}

/**
 * 查找统计制度树关联的元数据，未找到则插入并更新树节点
 * @param id
 */
export async function findMetaObject<T>(metaNode:MetaNode):Promise<T|null>{
    let metaObject = null;
    const tableName = `stats_survey_${metaNode.object_name ==='refTable'?'table':metaNode.object_name}`;

    if(metaNode.object_id){
        const findSql = `select * from ${tableName} where id = $1`;
        const result = await appDataBase.select<T[]>(findSql,[metaNode.object_id]);
        metaObject = result.length?result[0]:null;
    }else{
        //
        const insertSql = `insert into ${tableName}(text) values ($1)`;
        const result = await appDataBase.execute(insertSql,[metaNode.text]);
        if(result.lastInsertId && typeof result.lastInsertId === 'number'){
            metaObject = <T>{id:result.lastInsertId,text:metaNode.text};
            await updateMetaNode( {...metaNode,object_id:result.lastInsertId})
        }
    }
    return metaObject;
}

/**
 * 更新统计制度树的元数据
 * @param metadata
 */
export async function updateMetaObject<T>(nodeId:number,objectName:string,metadata:IMetaObject):Promise<number>{
    if(metadata.id || metadata.id === 0){
        //更新树节点的文本
        await updateMetaNodeText(nodeId,metadata.text);

        const tableName = `stats_survey_${objectName}`;
        //生成insert语句
        const properties = Object.keys(metadata).filter(property=>property!=='id');
        const updateSql = `update ${tableName} set ${properties.map((property,idx)=>`${property}=$${idx+2}`).join()}  where id = $1`;
        const bindValues = [metadata.id].concat(properties.map(property=>metadata[property]));
        //更新元数据
        const result = await appDataBase.execute(updateSql,bindValues);
        return result.rowsAffected;
    }
    return 0;
}
import {createTreeStore} from "../tree.store";
import type {TreeItem} from "../../youi/index.d";
import type {MetaNode} from "../../app-entity/metadata/surveyPlanTree";

import {
    findMetaNode,
    findMetaNodeTree,
    findPrevNode,
    findNextNode,
    insertMetaNode,
    updateMetaNode,
    removeMetaNode,
    updateMetaNodeMove,
    updateMetaNodeText,
} from "../../app-services/metadata/surveyPlanTreeServices";
import {levelListToTree} from "../../app-services/service.util";

export const createStore = () => {
    return createTreeStore<MetaNode>({
        async fetch(params?): Promise<TreeItem[]> {
            const metaNodes = await findMetaNodeTree(params);
            return levelListToTree(metaNodes,(treeNode,metaNode)=>{
                Object.assign(treeNode,toTreeItem(metaNode))
            });
        },

        async findNode(id:number):Promise<TreeItem>{
            const surveyPlanTree = await findMetaNode(id);
            return toTreeItem(surveyPlanTree);
        },

        async findPrev(id,pid,num):Promise<TreeItem>{
            const surveyPlanTree = await findPrevNode(id,pid,num);
            return toTreeItem(surveyPlanTree);
        },

        async findNext(id,pid,num):Promise<TreeItem>{
            const surveyPlanTree = await findNextNode(id,pid,num);
            return toTreeItem(surveyPlanTree);
        },

        async insert(surveyPlanTree):Promise<TreeItem>{
            const insertId = await insertMetaNode(surveyPlanTree);
            Object.assign(surveyPlanTree,{id:insertId});
            return toTreeItem(surveyPlanTree);
        },

        async update(surveyPlanTree):Promise<TreeItem>{
            await updateMetaNode(surveyPlanTree);
            return toTreeItem(surveyPlanTree);
        },

        async rename(id,text):Promise<void>{
            await updateMetaNodeText(id,text);
        },

        async updateNum(id,pid,num):Promise<void>{
            await updateMetaNodeMove(id,pid,num);
        },

        async remove(id:number):Promise<void>{
            await removeMetaNode(id);
        }
    });
}

export const treeStore = createStore();

/**
 *
 * @param surveyPlanTree
 */
function toTreeItem(metaNode:MetaNode):TreeItem|null{
    if(!metaNode)return null;
    return {
        id:metaNode.id+'',
        text:metaNode.text,
        group:metaNode.object_name,
        icon:`meta_${metaNode.object_name}`,
        datas:{num:metaNode.num,pid:metaNode.pid,object_name:metaNode.object_name,object_id:metaNode.object_id}
    }
}
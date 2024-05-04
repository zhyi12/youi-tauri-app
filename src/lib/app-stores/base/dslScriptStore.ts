import {createTreeStore} from "../tree.store";
import type {TreeItem} from "../../youi/index.d";
import type {DslScript} from "../../app-entity/base/dslScript";

import {
    findDslScript,
    findDslScriptTree,
    findPrevNode,
    findNextNode,
    insertDslScript,
    updateDslScript,
    removeDslScript,
    updateDslScriptMove,
    updateDslScriptText,
} from "../../app-services/base/dslScriptServices";
import {levelListToTree} from "../../app-services/service.util";

export const createStore = () => {
    return createTreeStore<DslScript>({
        async fetch(params?): Promise<TreeItem[]> {
            const dslScripts = await findDslScriptTree();
            return levelListToTree(dslScripts);
        },

        async findNode(id:number):Promise<TreeItem>{
            const dslScript = await findDslScript(id);
            return toTreeItem(dslScript);
        },

        async findPrev(id,pid,num):Promise<TreeItem>{
            const dslScript = await findPrevNode(id,pid,num);
            return toTreeItem(dslScript);
        },

        async findNext(id,pid,num):Promise<TreeItem>{
            const dslScript = await findNextNode(id,pid,num);
            return toTreeItem(dslScript);
        },

        async insert(dslScript):Promise<TreeItem>{
            const insertId = await insertDslScript(dslScript);
            Object.assign(dslScript,{id:insertId});
            return toTreeItem(dslScript);
        },

        async update(dslScript):Promise<TreeItem>{
            await updateDslScript(dslScript);
            return toTreeItem(dslScript);
        },

        async rename(id,text):Promise<void>{
            await updateDslScriptText(id,text);
        },

        async updateNum(id,pid,num):Promise<void>{
            await updateDslScriptMove(id,pid,num);
        },

        async remove(id:number):Promise<void>{
            await removeDslScript(id);
        }
    });
}

export const treeStore = createStore();

/**
 *
 * @param dslScript
 */
function toTreeItem(dslScript:DslScript):TreeItem|null{
    if(!dslScript)return null;
    return {
        id:dslScript.id+'',
        text:dslScript.text,
        datas:{num:dslScript.num,pid:dslScript.pid}
    }
}
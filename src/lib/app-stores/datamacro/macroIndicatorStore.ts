import {createTreeStore} from "../tree.store";
import type {TreeItem} from "../../youi/index.d";
import type {MacroIndicator} from "../../app-entity/macrodata/macroIndicator";

import {
    findMacroIndicator,
    findMacroIndicatorTree,
    findPrevNode,
    findNextNode,
    insertMacroIndicator,
    removeMacroIndicator,
    updateMacroIndicatorMove,
    updateMacroIndicatorText
} from "../../app-services/datamacro/macroIndicatorServices";
import {levelListToTree} from "../../app-services/service.util";

export const createStore = () => {
    return createTreeStore<MacroIndicator>({
        async fetch(params?): Promise<TreeItem[]> {
            const macroIndicators = await findMacroIndicatorTree();
            return levelListToTree(macroIndicators);
        },

        async findNode(id:number):Promise<TreeItem>{
            const macroIndicator = await findMacroIndicator(id);
            return toTreeItem(macroIndicator);
        },

        async findPrev(id,pid,num):Promise<TreeItem>{
            const macroIndicator = await findPrevNode(id,pid,num);
            return toTreeItem(macroIndicator);
        },

        async findNext(id,pid,num):Promise<TreeItem>{
            const macroIndicator = await findNextNode(id,pid,num);
            return toTreeItem(macroIndicator);
        },

        async insert(macroIndicator):Promise<TreeItem>{
            const insertId = await insertMacroIndicator(macroIndicator);
            Object.assign(macroIndicator,{id:insertId});
            return toTreeItem(macroIndicator);
        },

        async rename(id,text):Promise<void>{
            await updateMacroIndicatorText(id,text);
        },

        async updateNum(id,pid,num):Promise<void>{
            await updateMacroIndicatorMove(id,pid,num);
        },

        async remove(id:number):Promise<void>{
            await removeMacroIndicator(id);
        }
    });
}

export const treeStore = createStore();

/**
 *
 * @param macroIndicator
 */
function toTreeItem(macroIndicator:MacroIndicator):TreeItem|null{
    if(!macroIndicator)return null;
    return {
        id:macroIndicator.id+'',
        text:macroIndicator.text,
        datas:{num:macroIndicator.num,pid:macroIndicator.pid}
    }
}

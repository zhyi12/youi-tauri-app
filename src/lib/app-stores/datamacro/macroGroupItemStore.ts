import {createTreeStore} from "../tree.store";
import type {TreeItem} from "../../youi/index.d";
import type {MacroGroupItem} from "../../app-entity/datamacro/macroGroupItem";

import {
    findMacroGroupItem,
    findMacroGroupItemTree,
    findPrevNode,
    findNextNode,
    insertMacroGroupItem,
    updateMacroGroupItem,
    removeMacroGroupItem,
    updateMacroGroupItemMove,
    updateMacroGroupItemText,
} from "../../app-services/datamacro/macroGroupItemServices";
import {levelListToTree} from "../../app-services/service.util";

export const createStore = () => {
    return createTreeStore<MacroGroupItem>({
        async fetch(params?): Promise<TreeItem[]> {
            const macroGroupItems = await findMacroGroupItemTree(params.group_id);
            return levelListToTree(macroGroupItems);
        },

        async findNode(id:number):Promise<TreeItem>{
            const macroGroupItem = await findMacroGroupItem(id);
            return toTreeItem(macroGroupItem);
        },

        async findPrev(id,pid,num,group_id):Promise<TreeItem>{
            const macroGroupItem = await findPrevNode(id,pid,num,group_id);
            return toTreeItem(macroGroupItem);
        },

        async findNext(id,pid,num,group_id):Promise<TreeItem>{
            const macroGroupItem = await findNextNode(id,pid,num,group_id);
            return toTreeItem(macroGroupItem);
        },

        async insert(macroGroupItem):Promise<TreeItem>{
            const insertId = await insertMacroGroupItem(macroGroupItem);
            Object.assign(macroGroupItem,{id:insertId});
            return toTreeItem(macroGroupItem);
        },

        async update(macroGroupItem):Promise<TreeItem>{
            await updateMacroGroupItem(macroGroupItem);
            return toTreeItem(macroGroupItem);
        },

        async rename(id,text):Promise<void>{
            await updateMacroGroupItemText(id,text);
        },

        async updateNum(id,pid,num):Promise<void>{
            await updateMacroGroupItemMove(id,pid,num);
        },

        async remove(id:number):Promise<void>{
            await removeMacroGroupItem(id);
        }
    });
}

export const treeStore = createStore();

/**
 *
 * @param macroGroupItem
 */
function toTreeItem(macroGroupItem:MacroGroupItem):TreeItem|null{
    if(!macroGroupItem)return null;
    return {
        id:macroGroupItem.id+'',
        text:macroGroupItem.text,
        datas:{num:macroGroupItem.num,pid:macroGroupItem.pid}
    }
}
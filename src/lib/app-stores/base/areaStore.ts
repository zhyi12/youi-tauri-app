import {createTreeStore} from "../tree.store";
import type {TreeItem} from "../../youi/index.d";
import type {Area} from "../../app-entity/base/area";

import {
    findArea,
    findAreaTree,
    findPrevNode,
    findNextNode,
    insertArea,
    updateArea,
    removeArea,
    updateAreaMove,
    updateAreaText,
} from "../../app-services/base/areaServices";
import {levelListToTree} from "../../app-services/service.util";

export const createStore = () => {
    return createTreeStore<Area>({
        async fetch(params?): Promise<TreeItem[]> {
            const areas = await findAreaTree(params.pid,params.maxLevel);
            const rootArea = await findArea(params.pid);

            return [{
                ...toTreeItem(rootArea),
                children:levelListToTree(areas)
            }];
        },

        async findNode(id:number):Promise<TreeItem>{
            const area = await findArea(id);
            return toTreeItem(area);
        },

        async findPrev(id,pid,num):Promise<TreeItem>{
            const area = await findPrevNode(id,pid,num);
            return toTreeItem(area);
        },

        async findNext(id,pid,num):Promise<TreeItem>{
            const area = await findNextNode(id,pid,num);
            return toTreeItem(area);
        },

        async insert(area):Promise<TreeItem>{
            const insertId = await insertArea(area);
            Object.assign(area,{id:insertId});
            return toTreeItem(area);
        },

        async update(area):Promise<TreeItem>{
            await updateArea(area);
            return toTreeItem(area);
        },

        async rename(id,text):Promise<void>{
            await updateAreaText(id,text);
        },

        async updateNum(id,pid,num):Promise<void>{
            await updateAreaMove(id,pid,num);
        },

        async remove(id:number):Promise<void>{
            await removeArea(id);
        }
    });
}

export const treeStore = createStore();

/**
 *
 * @param area
 */
function toTreeItem(area:Area):TreeItem|null{
    if(!area)return null;
    return {
        id:area.id+'',
        text:area.text,
        datas:{num:area.num,pid:area.pid}
    }
}
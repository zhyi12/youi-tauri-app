import {createTreeStore} from "../tree.store";
import type {TreeItem} from "../../youi/index.d";
import type {DashboardGroup} from "../../app-entity/dmp/dashboardGroup";

import {
    findDashboardGroup,
    findDashboardGroupTree,
    findPrevNode,
    findNextNode,
    insertDashboardGroup,
    updateDashboardGroup,
    removeDashboardGroup,
    updateDashboardGroupMove,
    updateDashboardGroupText,
} from "../../app-services/dmp/dashboardGroupServices";
import {levelListToTree} from "../../app-services/service.util";

export const createStore = () => {
    return createTreeStore<DashboardGroup>({
        async fetch(params?): Promise<TreeItem[]> {
            const dashboardGroups = await findDashboardGroupTree();
            return levelListToTree(dashboardGroups,(node,record)=>{
                Object.assign(node,toTreeItem(record))
            });
        },

        async findNode(id:number):Promise<TreeItem>{
            const dashboardGroup = await findDashboardGroup(id);
            return toTreeItem(dashboardGroup);
        },

        async findPrev(id,pid,num):Promise<TreeItem>{
            const dashboardGroup = await findPrevNode(id,pid,num);
            return toTreeItem(dashboardGroup);
        },

        async findNext(id,pid,num):Promise<TreeItem>{
            const dashboardGroup = await findNextNode(id,pid,num);
            return toTreeItem(dashboardGroup);
        },

        async insert(dashboardGroup):Promise<TreeItem>{
            const insertId = await insertDashboardGroup(dashboardGroup);
            Object.assign(dashboardGroup,{id:insertId});
            return toTreeItem(dashboardGroup);
        },

        async update(dashboardGroup):Promise<TreeItem>{
            await updateDashboardGroup(dashboardGroup);
            return toTreeItem(dashboardGroup);
        },

        async rename(id,text):Promise<void>{
            await updateDashboardGroupText(id,text);
        },

        async updateNum(id,pid,num):Promise<void>{
            await updateDashboardGroupMove(id,pid,num);
        },

        async remove(id:number):Promise<void>{
            await removeDashboardGroup(id);
        }
    });
}

export const treeStore = createStore();

/**
 *
 * @param dashboardGroup
 */
function toTreeItem(dashboardGroup:DashboardGroup):TreeItem|null{
    if(!dashboardGroup)return null;
    return {
        id:dashboardGroup.id+'',
        text:dashboardGroup.text,
        icon:'group',
        datas:{num:dashboardGroup.num,pid:dashboardGroup.pid}
    }
}
import {writable} from "svelte/store";

import {
    insertDashboard,
    updateDashboard,
    removeDashboard,
    findDashboardByPager,
    findByGroup
} from "../../app-services/dmp/dashboardServices";

import type {Dashboard} from "../../app-entity/dmp/dashboard";
import type {Pager} from "../../app-dao/dao";

/**
 *
 */
export const createStore = () => {
    const {subscribe, update, set} = writable<{records:Dashboard[],totalCount:number}>({records:[],totalCount:0});

    return {
        subscribe,
        /**
         * 获取数据
         */
        findByPager: async (pager:Pager,dashboard?:Dashboard) => {
             const result = await findDashboardByPager(pager,dashboard);
             return set(result);
        },
        /**
         *
         * @param groupId
         */
        fetchByGroup:async (groupId)=>{
            const result = await findByGroup(groupId);
            return set({records:result});
        },
        /**
         *
         * @param dashboard
         */
        addDashboard: async (dashboard:Dashboard)=>{
            const insertId = await insertDashboard(dashboard);
            Object.assign(dashboard,{id:insertId});
            //
            return update(pagerRecords=>{
                return {totalCount:pagerRecords.totalCount+1,records:[dashboard,...pagerRecords.records]};
            })
        },
        /**
         * 删除
         * @param id
         */
        removeDashboard:async (id:number) =>{
            const result = await removeDashboard(id);
            return update(pagerRecords=>{
                return {totalCount:pagerRecords.totalCount-1,records:pagerRecords.records.filter(o=>o.id !== id)};
            })
        },
        /**
         *
         * @param dashboard
         */
        updateDashboard:async (dashboard:Dashboard)=>{

            const result = await updateDashboard(dashboard);

            return update(pagerRecords=>{
                return {totalCount:pagerRecords.totalCount,
                    records:pagerRecords.records.map(o=>{
                        if(o.id == dashboard.id){
                            return dashboard;
                        }
                        return o;
                    })};
            })
        }
    }
}

/**
 *
 */
export const dashboardStore = createStore();
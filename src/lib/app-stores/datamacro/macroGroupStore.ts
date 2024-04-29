import {writable} from "svelte/store";

import {
    insertMacroGroup,
    updateMacroGroup,
    removeMacroGroup,
    findMacroGroupByPager
} from "../../app-services/datamacro/macroGroupServices";

import type {MacroGroup} from "../../app-entity/datamacro/macroGroup";
import type {Pager} from "../../app-dao/dao";

/**
 *
 */
export const createStore = () => {
    const {subscribe, update, set} = writable<{records:MacroGroup[],totalCount:number}>({records:[],totalCount:0});

    return {
        subscribe,
        /**
         * 获取数据
         */
        findByPager: async (pager:Pager,macroGroup?:MacroGroup) => {
             const result = await findMacroGroupByPager(pager,macroGroup);
             return set(result);
        },
        /**
         *
         * @param macroGroup
         */
        addMacroGroup: async (macroGroup:MacroGroup)=>{
            const insertId = await insertMacroGroup(macroGroup);
            Object.assign(macroGroup,{id:insertId});
            //
            return update(pagerRecords=>{
                return {totalCount:pagerRecords.totalCount+1,records:[macroGroup,...pagerRecords.records]};
            })
        },
        /**
         * 删除
         * @param id
         */
        removeMacroGroup:async (id:number) =>{
            const result = await removeMacroGroup(id);
            return update(pagerRecords=>{
                return {totalCount:pagerRecords.totalCount-1,records:pagerRecords.records.filter(o=>o.id !== id)};
            })
        },
        /**
         *
         * @param macroGroup
         */
        updateMacroGroup:async (macroGroup:MacroGroup)=>{

            const result = await updateMacroGroup(macroGroup);

            return update(pagerRecords=>{
                return {totalCount:pagerRecords.totalCount,
                    records:pagerRecords.records.map(o=>{
                        if(o.id == macroGroup.id){
                            return macroGroup;
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
export const macroGroupStore = createStore();
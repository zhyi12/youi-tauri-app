import {writable} from "svelte/store";

import {insertSurveyTable,updateSurveyTable,removeSurveyTable,findSurveyTableByPager} from "../../app-services/metadata/surveyTableServices";

import type {SurveyTable} from "../../app-entity/metadata/surveyTable";
import type {Pager} from "../../app-dao/dao";

/**
 *
 */
export const createStore = () => {
    const {subscribe, update, set} = writable<{records:SurveyTable[],totalCount:number}>({records:[],totalCount:0});

    return {
        subscribe,
        /**
         * 获取数据
         */
        findByPager: async (pager:Pager,surveyTable?:SurveyTable) => {
             const result = await findSurveyTableByPager(pager,surveyTable);
             return set(result);
        },
        /**
         *
         * @param surveyTable
         */
        addSurveyTable: async (surveyTable:SurveyTable)=>{
            const insertId = await insertSurveyTable(surveyTable);
            Object.assign(surveyTable,{id:insertId});
            //
            return update(pagerRecords=>{
                return {totalCount:pagerRecords.totalCount+1,records:[surveyTable,...pagerRecords.records]};
            })
        },
        /**
         * 删除
         * @param id
         */
        removeSurveyTable:async (id:number) =>{
            const result = await removeSurveyTable(id);
            return update(pagerRecords=>{
                return {totalCount:pagerRecords.totalCount-1,records:pagerRecords.records.filter(o=>o.id !== id)};
            })
        },
        /**
         *
         * @param surveyTable
         */
        updateSurveyTable:async (surveyTable:SurveyTable)=>{

            const result = await updateSurveyTable(surveyTable);

            return update(pagerRecords=>{
                return {totalCount:pagerRecords.totalCount,
                    records:pagerRecords.records.map(o=>{
                        if(o.id == surveyTable.id){
                            return surveyTable;
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
export const surveyTableStore = createStore();
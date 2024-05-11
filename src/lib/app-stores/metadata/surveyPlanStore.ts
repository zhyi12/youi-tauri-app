import {writable} from "svelte/store";

import {insertSurveyPlan,updateSurveyPlan,removeSurveyPlan,findSurveyPlanByPager} from "../../app-services/metadata/surveyPlanServices";

import type {SurveyPlan} from "../../app-entity/metadata/surveyPlan";
import type {Pager} from "../../app-dao/dao";

/**
 *
 */
export const createStore = () => {
    const {subscribe, update, set} = writable<{records:SurveyPlan[],totalCount:number}>({records:[],totalCount:0});

    return {
        subscribe,
        /**
         * 获取数据
         */
        findByPager: async (pager:Pager,surveyPlan?:SurveyPlan) => {
             const result = await findSurveyPlanByPager(pager,surveyPlan);
             return set(result);
        },
        /**
         *
         * @param surveyPlan
         */
        addSurveyPlan: async (surveyPlan:SurveyPlan)=>{
            const insertId = await insertSurveyPlan(surveyPlan);
            Object.assign(surveyPlan,{id:insertId});
            //
            return update(pagerRecords=>{
                return {totalCount:pagerRecords.totalCount+1,records:[surveyPlan,...pagerRecords.records]};
            })
        },
        /**
         * 删除
         * @param id
         */
        removeSurveyPlan:async (id:number) =>{
            const result = await removeSurveyPlan(id);
            return update(pagerRecords=>{
                return {totalCount:pagerRecords.totalCount-1,records:pagerRecords.records.filter(o=>o.id !== id)};
            })
        },
        /**
         *
         * @param surveyPlan
         */
        updateSurveyPlan:async (surveyPlan:SurveyPlan)=>{

            const result = await updateSurveyPlan(surveyPlan);

            return update(pagerRecords=>{
                return {totalCount:pagerRecords.totalCount,
                    records:pagerRecords.records.map(o=>{
                        if(o.id == surveyPlan.id){
                            return surveyPlan;
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
export const surveyPlanStore = createStore();
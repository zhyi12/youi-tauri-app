import {writable} from "svelte/store";

import {insertSurveyTask,updateSurveyTask,removeSurveyTask,findSurveyTaskByPager} from "../../app-services/metadata/surveyTaskServices";

import type {SurveyTask} from "../../app-entity/metadata/surveyTask";
import type {Pager} from "../../app-dao/dao";

/**
 *
 */
export const createStore = () => {
    const {subscribe, update, set} = writable<{records:SurveyTask[],totalCount:number}>({records:[],totalCount:0});

    return {
        subscribe,
        /**
         * 获取数据
         */
        findByPager: async (pager:Pager,surveyTask?:SurveyTask) => {
             const result = await findSurveyTaskByPager(pager,surveyTask);
             return set(result);
        },
        /**
         *
         * @param surveyTask
         */
        addSurveyTask: async (surveyTask:SurveyTask)=>{
            const insertId = await insertSurveyTask(surveyTask);
            Object.assign(surveyTask,{id:insertId});
            //
            return update(pagerRecords=>{
                return {totalCount:pagerRecords.totalCount+1,records:[surveyTask,...pagerRecords.records]};
            })
        },
        /**
         * 删除
         * @param id
         */
        removeSurveyTask:async (id:number) =>{
            const result = await removeSurveyTask(id);
            return update(pagerRecords=>{
                return {totalCount:pagerRecords.totalCount-1,records:pagerRecords.records.filter(o=>o.id !== id)};
            })
        },
        /**
         *
         * @param surveyTask
         */
        updateSurveyTask:async (surveyTask:SurveyTask)=>{

            const result = await updateSurveyTask(surveyTask);

            return update(pagerRecords=>{
                return {totalCount:pagerRecords.totalCount,
                    records:pagerRecords.records.map(o=>{
                        if(o.id == surveyTask.id){
                            return surveyTask;
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
export const surveyTaskStore = createStore();
import {writable} from "svelte/store";

import {insertSurveyProject,updateSurveyProject,removeSurveyProject,findSurveyProjectByPager} from "../../app-services/metadata/surveyProjectServices";

import type {SurveyProject} from "../../app-entity/metadata/surveyProject";
import type {Pager} from "../../app-dao/dao";

/**
 *
 */
export const createStore = () => {
    const {subscribe, update, set} = writable<{records:SurveyProject[],totalCount:number}>({records:[],totalCount:0});

    return {
        subscribe,
        /**
         * 获取数据
         */
        findByPager: async (pager:Pager,surveyProject?:SurveyProject) => {
             const result = await findSurveyProjectByPager(pager,surveyProject);
             return set(result);
        },
        /**
         *
         * @param surveyProject
         */
        addSurveyProject: async (surveyProject:SurveyProject)=>{
            const insertId = await insertSurveyProject(surveyProject);
            Object.assign(surveyProject,{id:insertId});
            //
            return update(pagerRecords=>{
                return {totalCount:pagerRecords.totalCount+1,records:[surveyProject,...pagerRecords.records]};
            })
        },
        /**
         * 删除
         * @param id
         */
        removeSurveyProject:async (id:number) =>{
            const result = await removeSurveyProject(id);
            return update(pagerRecords=>{
                return {totalCount:pagerRecords.totalCount-1,records:pagerRecords.records.filter(o=>o.id !== id)};
            })
        },
        /**
         *
         * @param surveyProject
         */
        updateSurveyProject:async (surveyProject:SurveyProject)=>{

            const result = await updateSurveyProject(surveyProject);

            return update(pagerRecords=>{
                return {totalCount:pagerRecords.totalCount,
                    records:pagerRecords.records.map(o=>{
                        if(o.id == surveyProject.id){
                            return surveyProject;
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
export const surveyProjectStore = createStore();
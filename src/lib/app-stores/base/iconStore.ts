import {writable} from "svelte/store";

import {insertIcon, updateIcon, removeIcon, findByPager} from "../../app-services/base/iconServices";

import type {Icon} from "../../app-entity/base/icon";
import type {Pager} from "../../app-dao/dao";

/**
 *
 */
export const createStore = () => {
    const {subscribe, update, set} = writable<{records:Icon[],totalCount:number}>({records:[],totalCount:0});

    return {
        subscribe,
        /**
         * 获取数据
         */
        findByPager: async (pager:Pager,icon?:Icon) => {
            const result = await findByPager(pager,icon);
            return set(result);
        },
        /**
         *
         * @param icon
         */
        addIcon: async (icon:Icon)=>{
            const insertId = await insertIcon(icon);
            Object.assign(icon,{id:insertId});
            //
            return update(pagerRecords=>{
                return {totalCount:pagerRecords.totalCount+1,records:[icon,...pagerRecords.records]};
            })
        },
        /**
         * 删除
         * @param id
         */
        removeIcon:async (id:number) =>{
            await removeIcon(id);
            return update(pagerRecords=>{
                return {totalCount:pagerRecords.totalCount-1,records:pagerRecords.records.filter(o=>o.id !== id)};
            })
        },
        /**
         *
         * @param icon
         */
        updateIcon:async (icon:Icon)=>{

            await updateIcon(icon);

            return update(pagerRecords=>{
                return {totalCount:pagerRecords.totalCount,
                    records:pagerRecords.records.map(o=>{
                        if(o.id == icon.id){
                            return icon;
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
export const iconStore = createStore();
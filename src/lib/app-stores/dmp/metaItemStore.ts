import {writable} from "svelte/store";

import {
    insertMetaItem,
    updateMetaItem,
    removeMetaItem,
    findMetaItemByPager as findByPager,
} from "../../app-services/dmp/metaItemServices";

import type {MetaItem} from "../../app-entity/dmp/metaItem";
import type {Pager} from "../../app-dao/dao";

/**
 *
 */
export const createStore = () => {
    const {subscribe, update, set} = writable<{records:MetaItem[],totalCount:number}>({records:[],totalCount:0});

    return {
        subscribe,
        /**
         * 获取数据
         */
        findByPager: async (pager:Pager,metaItem?:MetaItem) => {
             const result = await findByPager(pager,metaItem);
             return set(result);
        },
        /**
         *
         * @param metaItem
         */
        addMetaItem: async (metaItem:MetaItem)=>{
            const insertId = await insertMetaItem(metaItem);
            Object.assign(metaItem,{id:insertId});
            //
            return update(pagerRecords=>{
                return {totalCount:pagerRecords.totalCount+1,records:[metaItem,...pagerRecords.records]};
            })
        },
        /**
         * 删除
         * @param id
         */
        removeMetaItem:async (id:number) =>{
            await removeMetaItem(id);
            return update(pagerRecords=>{
                return {totalCount:pagerRecords.totalCount-1,records:pagerRecords.records.filter(o=>o.id !== id)};
            })
        },
        /**
         *
         * @param metaItem
         */
        updateMetaItem:async (metaItem:MetaItem)=>{

            await updateMetaItem(metaItem);

            return update(pagerRecords=>{
                return {totalCount:pagerRecords.totalCount,
                    records:pagerRecords.records.map(o=>{
                        if(o.id == metaItem.id){
                            return metaItem;
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
export const metaItemStore = createStore();
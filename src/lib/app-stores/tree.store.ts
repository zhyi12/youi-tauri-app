import {writable} from "svelte/store";
import {findTreeNode, removeTreeNode} from "../youi";
import type {TreeItem} from "../youi/index.d";
import type {TreeStore} from "./store";
import type {ITreeService} from "../app-services/service";

/**
 *
 */
export const createTreeStore = <T> (service:ITreeService<T>) => {

    const {subscribe, update, set} = writable(<TreeStore>{nodes:[],expandedIds:[]});

    /**
     *
     * @param record
     */
    const addChild = async (record:T) => {
        const node = await service.insert(record);
        Object.assign(record,{id:node.id});
        //更新num
        return update(model=>{
            const needUpdateNum = record.num === 1 || !record.num;
            let maxNum = 0;
            if(record.pid){
                let parent = findTreeNode(model.nodes,record.pid);
                if(parent){
                    parent.children = parent.children||[];
                    if(needUpdateNum){
                        maxNum = parent.children.reduce((a,v)=>Math.max(a,v.datas.num),1);
                    }
                    parent.children.push(node);
                    model.expandedIds.push(parent.id);
                }
            }else{
                if(needUpdateNum){
                    maxNum = model.nodes.reduce((a,v)=>Math.max(a,v.datas.num),1);
                }
                model.nodes.push(node);
            }

            if(needUpdateNum && maxNum>0){
                //更新界节点面数据
                node.datas.num = maxNum+0.5;
                //持久化更新
                service.updateNum(node.id,record.pid,node.datas.num).then(r=>r);
            }
            return model;
        });
    };
    /**
     *
     * @param refNode
     */
    const calculatePrevNum = async(refNode:TreeItem)=>{
        const prevNode = await service.findPrev(refNode.id,refNode.datas.pid,refNode.datas.num);
        let num = refNode.datas.num;
        if(!prevNode){
            num = num - 0.005;
        }else{
            num= (refNode.datas.num + prevNode.datas.num)/2;
        }
        return num;
    };
    /**
     *
     * @param refNode
     */
    const calculateNextNum =async (refNode:TreeItem) => {
        const nextNode = await service.findNext(refNode.id,refNode.datas.pid,refNode.datas.num);
        let num = refNode.datas.num;
        if(!nextNode){
            num = num + 0.005;
        }else{
            num= (refNode.datas.num + nextNode.datas.num)/2;
        }
        return num;
    }
    /**
     *
     * @param refId
     * @param record
     */
    const insertBefore = async(refId:number,record:T)=>{
        const refNode = await service.findNode(refId);
        if(refNode){
            const num = await calculatePrevNum(refNode);
            Object.assign(record,{num,pid:refNode.datas.pid});
            await addChild(record);
        }
    };

    const insertAfter = async(refId:number,record:T)=>{
        const refNode = await service.findNode(refId);
        if(refNode){
            const num = await calculateNextNum(refNode);
            Object.assign(record,{num,pid:refNode.datas.pid});
            await addChild(record);
        }
    };

    return {
        subscribe,

        /**
         *
         * @param params
         */
        fetch:async (params)=>{
            const nodes:TreeItem[] = await service.fetch(params);
            return set({nodes,expandedIds:[]})
        },

        addChild,

        insertBefore,

        insertAfter,
        /**
         * 重命名
         * @param id
         */
        rename:async(id:number,text:string)=>{
            await service.rename(id,text);
            return update(model=>{
                const node = findTreeNode(model.nodes,id.toString());
                if(node){
                    Object.assign(node,{text});
                }
                return model;
            });
        },
        /**
         * 移动节点
         * @param fromId
         * @param targetId
         * @param position
         */
        move:async(fromId:number,targetId:number,position:string)=>{
            const targetNode = await service.findNode(targetId);
            if(targetNode){
                let newNum;
                let newPid;
                if('child' === position){
                    newPid = targetId;
                }else if('before' === position){
                    newPid = targetNode.datas.pid;
                    const refNode = await service.findNode(targetId);
                    newNum = await calculatePrevNum(refNode);
                }else if('after' === position){
                    newPid = targetNode.datas.pid;
                    const refNode = await service.findNode(targetId);
                    newNum = await calculateNextNum(refNode);
                }

                return update(model=>{
                    const parent = newPid?findTreeNode(model.nodes,newPid):null;
                    // 从父节点查找最大的num
                    if(!newNum){
                        if(newPid){
                            if(parent){
                                parent.children = parent.children||[];
                                newNum = parent.children.reduce((a,v)=>Math.max(a,v.datas.num),1);
                            }
                        }else{
                            newNum = model.nodes.reduce((a,v)=>Math.max(a,v.datas.num),1);
                        }
                        newNum = newNum||1 + 0.05;
                    }
                    // 更新数据库
                    service.updateNum(fromId,newPid,newNum||1).then(_=>_);
                    // 更新模型
                    const sourceNode = findTreeNode(model.nodes,fromId.toString());
                    // 删除从模型移动的节点
                    removeTreeNode(model.nodes,fromId.toString());
                    // 更新模型节点移动后属性
                    Object.assign(sourceNode.datas,{pid:newPid,num:newNum});
                    // 移动节点重新加入模型
                    if(parent){
                        parent.children = parent.children||[];
                        parent.children.push(sourceNode);
                    }else{
                        model.nodes.push(sourceNode);
                    }
                    return model;
                })
            }
        },
        /**
         * 删除节点
         * @param id
         */
        removeNode:async (id:number)=>{
            await service.remove(id);
            return update(model=>{
                removeTreeNode(model.nodes,id.toString());
                return model;
            });
        }
    }
}
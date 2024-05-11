import {writable} from "svelte/store";
import {findPathNodes, findTreeNode, removeTreeNode} from "../youi";
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
    const addChild = async (record:T,groupId?:number|string) => {
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

            model.activeId = node.id.toString();
            return model;
        });
    };
    /**
     *
     * @param refNode
     */
    const calculatePrevNum = async(refNode:TreeItem,groupId?:number|string)=>{
        const prevNode = await service.findPrev(refNode.id,refNode.datas.pid,refNode.datas.num,groupId);
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
    const calculateNextNum = async (refNode:TreeItem,groupId?:number|string) => {
        const nextNode = await service.findNext(refNode.id,refNode.datas.pid,refNode.datas.num,groupId);
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
    const insertBefore = async(refId:number,record:T,groupId?:number|string)=>{
        const refNode = await service.findNode(refId);
        if(refNode){
            const num = await calculatePrevNum(refNode,groupId);
            Object.assign(record,{num,pid:refNode.datas.pid});
            await addChild(record);
        }
    };

    const insertAfter = async(refId:number,record:T,groupId?:number|string)=>{
        const refNode = await service.findNode(refId);
        if(refNode){
            const num = await calculateNextNum(refNode,groupId);
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
        fetch:async (params,activeId?:string,expandedIds?:string[])=>{
            const nodes:TreeItem[] = await service.fetch(params);
            if(!expandedIds)expandedIds = [];
            if(!expandedIds.length && nodes.length.length===1){
                expandedIds = [nodes[0].id];
            }
            return set({nodes,expandedIds,activeId})
        },

        addChild,

        insertBefore,

        insertAfter,
        /**
         * 重命名
         * @param id
         */
        rename:async(id:number,text:string,onlyShow?:boolean)=>{
            if(!onlyShow){
                await service.rename(id,text);
            }
            return update(model=>{
                const node = findTreeNode(model.nodes,id.toString());
                if(node){
                    Object.assign(node,{text});
                }
                model.activeId = id.toString();
                return model;
            });
        },
        update:async(record:T)=>{
            const node = await service.update(record);
            return update(model=>{
                const oldNode = findTreeNode(model.nodes,record.id.toString());
                if(oldNode){
                    Object.assign(oldNode,node);
                }
                model.activeId = node.id.toString();
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
        removeNode:async (id:number,afterRemove)=>{
            await service.remove(id);
            return update(model=>{
                const pathNodes = findPathNodes(model.nodes,id.toString());
                let levelNums;
                if(pathNodes.length>1){
                    const parent = pathNodes[pathNodes.length - 2];
                    levelNums = parent.children.map(n=>({num:n.datas.num,id:n.id}));
                    //删除节点
                    parent.children =  parent.children.filter(n=>n.id != id);
                }else{
                    levelNums = model.nodes.map(n=>({num:n.datas.num,id:n.id}));
                    //删除节点
                    model.nodes = model.nodes.filter(n=>n.id != id);
                }

                levelNums.sort();

                const node = pathNodes.pop();

                if(levelNums.length > 1){
                    //
                    let refId;
                    const index = levelNums.map(l=>l.id).indexOf(node.id);
                    if(index === levelNums.length-1){
                        refId = levelNums[levelNums.length-2].id;
                    }else{
                        refId = levelNums[index+1].id;
                    }
                    pathNodes.push({id:refId});
                }

                const redirectPaths = pathNodes.map(n=>n.id);

                if(afterRemove){
                    afterRemove(node,redirectPaths);
                }

                model.activeId = redirectPaths[redirectPaths.length-1];

                return model;
            });
        }
    }
}
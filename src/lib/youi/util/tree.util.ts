
import type TreeItem from '../tree/Tree.svelte';

/**
 * 查找节点
 * @param children
 * @param nodeId
 */
export function findTreeNode(children:TreeItem[],nodeId:string){
    for(let i=0;i<children.length;i++){
        const item = children[i];
        if(item.id == nodeId){
            return item;
        }else if(Array.isArray(item.children)){
            const next_item = findTreeNode(item.children,nodeId);
            if(next_item){
                return next_item;
            }
        }
    }
    return null;
}

/**
 * 删除节点
 * @param children
 * @param nodeId
 * @param parentNode
 * @returns {null|*}
 */
export function removeTreeNode(children:TreeItem[],nodeId:string,parentNode?:TreeItem):TreeItem|null{
    for(let i=0;i<children.length;i++){
        const item = children[i];
        if(item.id == nodeId){
            children.splice(i,1);
            return parentNode;
        }else if(Array.isArray(item.children)){
            let parent = removeTreeNode(item.children,nodeId,item);
            if(parent){
                return parent;
            }
        }
    }
    return null;
}

/**
 *
 * @param children
 * @param nodeId
 * @returns {*[]}
 */
export function findPathNodes(children:TreeItem[],nodeId:string,parentPathNodes?:TreeItem[]){
    parentPathNodes = parentPathNodes||[];
    let pathNodes = [];
    for(let i=0;i<children.length;i++){
        const item = children[i];
        if(item.id == nodeId){
            return [...parentPathNodes,item];
        }else if(item.children){
            const itemAncestors = findPathNodes(item.children,nodeId,[...parentPathNodes,item]);
            if(itemAncestors.length){
                return itemAncestors;
            }
        }
    }
    return pathNodes;
}
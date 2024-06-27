import type {FilterNode} from "./Filter";

export const FILTER_CONTEXT = "FilterContext"

export const FILTER_OPS = [
    {name:'eq',text:'等于',symbol:'='},
    {name:'neq',text:'不等于',symbol:'!='},
    {name:'lt',text:'小于',symbol:'<'},
    {name:'lte',text:'小于等于',symbol:'<='},
    {name:'gt',text:'大于',symbol:'>'},
    {name:'gte',text:'大于等于',symbol:'>='},
    {name:'start_with',text:'以字符开始',symbol:''},
    {name:'end_with',text:'以字符结束',symbol:''},
    {name:'like',text:'模糊匹配',symbol:''},
    {name:'is_null',text:'空值',symbol:''},
    {name:'is_not_null',text:'非空',symbol:''},
    {name:'in',text:'IN',symbol:''},
];

export const FILTER_OP_MAP:Map<string,string> = FILTER_OPS.reduce((m,v)=>{
    m.set(v.name,v.symbol||v.text)
    return m;
},new Map<string,string>());

/**
 * 通过层级生成树
 * @param items
 */
export function toTree(items:FilterNode[]) {
    const treeNodes = [];
    let levelNodeMap = {};
    for(let i=0;i<items.length;i++){
        const node = {...items[i]};
        const level = node.level||1;
        if(i === 0 || level === 1){
            treeNodes.push(node);
            levelNodeMap = {}
            levelNodeMap[`L1`] = node;
        }else if(level>1 && i>0){
            let parent;
            for(let l=level-1;l>=0;l--){
                if(levelNodeMap[`L${l}`]){
                    parent = levelNodeMap[`L${l}`];
                    break;
                }
            }
            if(parent){
                parent.items = parent.items||[];
                node.level = parent.level+1;
                parent.items.push(node);
                levelNodeMap[`L${level}`] = node;
            }
        }
    }
    return treeNodes;
}

export function findTreeItem(items:FilterNode[],nodeId:string){
    for(let i=0;i<items.length;i++){
        const item = items[i];
        if(item.id == nodeId){
            return item;
        }else if(Array.isArray(item.items)){
            const next_item = findTreeItem(item.items,nodeId);
            if(next_item){
                return next_item;
            }
        }
    }
    return null;
}

<script lang="ts">
    import {createEventDispatcher, setContext} from "svelte";
    import {writable} from "svelte/store";
    import {FILTER_CONTEXT, findTreeItem, toTree} from "./helper";
    import Connection from "./Connection.svelte";
    import {Icon,plusIcon,indexOf, uuid} from "../../youi";
    import type {TreeItem} from "../../youi";

    const dispatch = createEventDispatcher();
    /**
     * 条件、连接项集合
     */
    export let items = [];

    /**
     * 可选列
     */
    export let columns = [];

    /**
     *
     */
    export let refValueItems:TreeItem = [];

    export let icons = (_name)=>undefined;

    export let loadValueItems =({_id,_property,_dataType})=>[];

    const selectItems = writable(columns);
    const valueItems = writable(refValueItems);
    const collapseConnectionIds = writable([]);
    const activeId = writable();

    const addRootCondition = () => {
        const newCondition = {id:uuid(),nodeType:'Item',property:'',operator:'eq',value:[],level:2};
        if(items.length === 0){
            items = [{id:uuid(),nodeType:'And',level:1},newCondition];
        }else{
            items.push(newCondition);
            items = items;
        }
        dispatch('change',{
            type:'addCondition'
        });
    }
    /**
     *
     * @param connectId
     * @param connect
     * @param refId
     */
    const addCondition = (connectId,connect,refId)=>{
        // 连接项
        const connectIndex = indexOf(items,connectId);
        const refIndex = indexOf(items,refId);
        if(connectIndex>-1 && refIndex>-1){
            const connectItem = items[connectIndex];
            const refItem = items[refIndex];
            const newCondition = {
                id:uuid(),
                nodeType:'Item',
                operator:'eq',
                property:'',
                value:[],
                level:connectItem.level+1
            }
            if(connectItem.nodeType.toLowerCase() === connect){
                //相同连接，直接追加条件
                items.splice(refIndex,0,newCondition)
            }else{
                //不同连接，创建新连接,ref和newCondition移动到新连接下
                if(refItem.nodeType!=='Item'){
                    //相关项为连接的处理
                    for(let i=refIndex+1;i<items.length;i++){
                        if(items[i].level>refItem.level){
                            items[i].level = items[i].level+1;
                        }else{
                            break;
                        }
                    }
                }
                refItem.level = refItem.level+1;

                items.splice(refIndex,0,
                    {id:uuid(),nodeType:connect==='and'?'And':'Or',level:connectItem.level+1},
                    {...newCondition,level:connectItem.level+2 },
                );
            }
            dispatch('change',{
                type:'addCondition'
            });
        }
    };

    setContext(FILTER_CONTEXT,{
        activeId,
        selectItems,
        valueItems,
        collapseConnectionIds,
        toggleConnect:(connectId)=>{
            const connectIndex = indexOf(items,connectId);
            items[connectIndex].nodeType = items[connectIndex].nodeType === 'And'?'Or':'And';

            dispatch('change',{
                type:'toggle',
                id:connectId,
                value:items[connectIndex].nodeType
            });
        },
        addCondition,
        removeCondition:(connectId,itemId)=>{
            const connectIndex = indexOf(items,connectId);
            const itemIndex = indexOf(items,itemId);
            const connectNode = findTreeItem(treeItems,connectId);
            const item = {...items[itemIndex]};

            if(connectIndex > 0 && connectNode.items.length === 2){
                const otherNode = connectNode.items.find(n=>n.id !== itemId);
                const otherIndex = indexOf(items,otherNode.id);
                //删除前只有两个条件时，删除连接节点，剩余的条件上移一级
                items.splice(connectIndex,3,{...items[otherIndex],level:otherNode.level-1});
            }else{
                items.splice(itemIndex,1);
            }

            dispatch('change',{
                type:'remove',
                item
            });
        },

        updateCondition:(id,property,value,oldValue)=>{
            const itemIndex = indexOf(items,id);

            const item = items[itemIndex];
            if(item && value !== oldValue){
                item[property] = value;
                dispatch('change',{
                    type:'update',
                    property,
                    value,
                    oldValue
                });
                items = items;
            }
        },
        updateProperty:(id,dataType,value,oldValue)=>{
            const itemIndex = indexOf(items,id);
            const item = items[itemIndex];

            item.dataType = dataType;
            Object.assign(item,{
                dataType,
                property:value
            });
            dispatch('change',{
                type:'update',
                property:'property',
                value,
                oldValue
            });
            items = items;
        },
        loadValueItems,
        icons
    });

    $:treeItems = toTree([...items||[]]);
    $:selectItems.set(columns);
    $:valueItems.set(refValueItems);

</script>

<div class="p-2 mr-8">
    {#if treeItems.length}
        <Connection connect={treeItems[0].nodeType.toLowerCase()}
                    id={treeItems[0].id}
                    root={true}
                    items={treeItems[0].items}/>
    {/if}
    <div data-tip="增加过滤条件"
         on:click={()=>addRootCondition()}
         class="tooltip float-left py-1 bg-blue-50 w-8 h-8 hover:text-red-500 rounded-full">
        <div class="cursor-pointer text-center caret ">
            <Icon scale={0.8} data={plusIcon}/>
        </div>
    </div>
</div>
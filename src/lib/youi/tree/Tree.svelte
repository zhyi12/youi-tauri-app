<script lang="ts">

    import {createEventDispatcher, setContext,tick} from "svelte";
    import {writable} from "svelte/store";
    import TreeNodeList from "./TreeNodeList.svelte";
    import {mouse} from "../mouse/mouse";
    import {dropping} from "../mouse/dropping";
    import classnames from "../util/classname";
    import type {TreeItem} from "./tree.d";
    import {findTreeNode} from "../util/tree.util";

    let className = '';

    export { className as class };

    export let children:TreeItem[] = [];
    export let selectedIds:string[] = [];
    export let expandedIds:string[] = [];
    export let hoverId:string = undefined;

    export let draggable = true;

    export let droppable = true;

    export let acceptDropping = undefined;

    export let isDropping = false;

    /**
     * 是否节点排序
     */
    export let sorted = true;
    /**
     * 是否多选
     */
    export let multiple = false;

    // 节点渲染
    export let nodeRender:()=>any = undefined;

    /**
     * 当前定位的节点ID
     */
    let activeId:string = undefined;
    const selectedNodeIds = writable(selectedIds);
    const expandedNodeIds = writable(expandedIds);
    const dispatch = createEventDispatcher();
    const activeNodeId = writable(activeId);
    const hoverNodeId = writable(hoverId);

    const treeConfig = writable({
        sorted,
        multiple,
        nodeRender
    });

    setContext("Tree", {
        activeNodeId,
        hoverNodeId,
        expandedNodeIds,
        selectedNodeIds,
        treeConfig,
        selectNode:({id})=>{
            if(multiple){
                //多选
            }else{
                //单选
                selectedIds = [id];
            }
        },
        toggleNode:({id})=>{
            if(expandedIds.includes(id)){
                expandedIds = expandedIds.filter(selectedId=>selectedId !== id);
            }else{
                expandedIds = [...expandedIds,id];
            }
        }
    });
    //
    $: selectedNodeIds.set(selectedIds);
    $: expandedNodeIds.set(expandedIds);
    $: hoverNodeId.set(hoverId);
    $: classes = classnames("youi-tree p-1 text-sm relative",className);

    let dragPosition = {x:0,y:0};
    let dragItem = undefined;
    let dragging = false;
    let dragContent = {html:''};
    let currentDropDom = undefined;
    let currentDragDom = undefined;

    /**
     *
     * @param event
     */
    const mouseStart = (event) => {
        let dragDom = event.target.closest('li');
        if(dragDom){
            const dragId = dragDom.getAttribute('id');
            dragItem = findTreeNode(children,dragId);
            dragContent = {html:dragItem?.text};
            currentDragDom = dragDom;
            isDropping = droppable;
        }
    }

    /**
     *
     */
    const mouseDrag = (event) => {
        //计算位置
        if(draggable && dragItem){
            dragPosition = {
                x:event.pageX+5,
                y:event.pageY+5
            };
        }

        if(droppable && isDropping){
            let dropDom = event.target.closest('li');
            if(dropDom && (!currentDropDom || dropDom != currentDropDom)){
                releaseDropping();
                currentDropDom = dropDom;
                if(currentDragDom){
                    const dropId = dropDom.getAttribute('id');
                    const selector = `li[id="${dropId}"]`;
                    const isSame = `${dropId}` == `${dragItem.id}`;
                    if(isSame){
                        return;
                    }else if(!selector,currentDragDom.querySelector(selector)){
                        return;
                    }
                }
                //
                dropDom.dispatchEvent(new CustomEvent('dropping',{detail:dragItem,
                    bubbles: true,
                    cancelable: true }));
            }

            //计算插入位置
            if(isDropping && dropDom && dropDom.classList.contains('dropping')){
                if(event.offsetY<10){
                    dropDom.classList.remove('dropping-after');
                    dropDom.classList.add('dropping-before');
                }else if(event.offsetY>20){
                    dropDom.classList.remove('dropping-before');
                    dropDom.classList.add('dropping-after');
                }else{
                    dropDom.classList.remove('dropping-before','dropping-after');
                }
            }
        }
    }

    const mouseStop = () => {
        if(dragItem){

        }

        if( droppable && isDropping && currentDropDom){
            if(dragItem){
                const position = currentDropDom.classList.contains('dropping-before')?'before':
                    (currentDropDom.classList.contains('dropping-after')?'after':'child');
                dispatch('move',{position,from:dragItem.id,to:currentDropDom.getAttribute('id')});
            }
            releaseDropping();
        }
        dragPosition = undefined;
        dragItem = undefined;
        currentDropDom = undefined;
        currentDragDom = undefined;
        isDropping = false;
    }

    /**
     * 释放dropping
     */
    function releaseDropping() {
        if(currentDropDom){
            currentDropDom.dispatchEvent(new CustomEvent('releaseDropping',{detail:{},bubbles:true}));
        }
    }

</script>

<div class={classes}
     use:dropping={{droppable,acceptDropping}}
     use:mouse={draggable?{mouseStart,mouseDrag,mouseStop,delay:50}:null}>
    <ul>
        <TreeNodeList root={true} {children} level={0}/>
    </ul>

    {#if draggable && dragItem}
    <ul class="tree-dragging-helper dragging-backup"

    >
        <li class="absolute" style={`left:${dragPosition.x}px;top:${dragPosition.y}px`}>
            {@html dragContent.html}
        </li>
    </ul>
    {/if}
</div>

<style>
    .tree-dragging-helper{
        pointer-events: none;
        position: fixed;
        top: 0;
        right: 0;
        bottom: 0;
        left: 0;
        margin: 0;
        display: grid;
        height: 100%;
        max-height: none;
        width: 100%;
        max-width: none;
        justify-items: center;
        padding: 0;
        overscroll-behavior: contain;
        z-index: 999;
        background-color: transparent;
        color: inherit;
        transition-duration: .2s;
        transition-timing-function: cubic-bezier(0,0,.2,1);
        transition-property: transform,opacity,visibility;
        overflow-y: hidden;
    }
</style>

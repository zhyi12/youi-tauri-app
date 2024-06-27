<script>
    import {Icon, uuid,uncheckIcon,checkedIcon} from "../../youi";
    import CircleAddButton from "./CircleAddButton.svelte";

    export let orders = [];

    export let icons = (_name)=>undefined;
    /**
     * 可选列
     */
    export let columns = [];

    const addOrder = () => {
        orders.push({
            id:uuid(),
            property:'',
            descending:false
        });
        orders = orders;
    }

    const toggleDescending = (order) => {
        order.descending = !order.descending;
        orders = orders;
    }

    const updateProperty = (order,item) => {
        order.property = item.name;
        orders = orders;
    }

    $:textMap = columns.reduce((m,v)=>m.set(v.name,v.text),new Map());
</script>

<div>
    {#each orders as order}
        <div class="flex float-left border rounded-md h-8">
            <div class="dropdown">
                <div tabindex="0" role="button" class="text-green-700 min-w-24 h-8 p-1">
                    {textMap.get(order.property)||order.property}
                </div>
                <ul class="menu fixed dropdown-content w-80 bg-base-100 rounded border overflow-x-hidden z-[2]">
                    {#each columns as item}
                        <li title={item.text} on:mousedown={()=>updateProperty(order,item)}>
                            <a class:active={item.name === order.property}>
                                <Icon data={icons(item.dataType)}/>
                                {item.text}
                            </a>
                        </li>
                    {/each}
                </ul>
            </div>
            <div data-tip="是否逆序" class="p-1 tooltip cursor-pointer w-8 text-center" on:click={()=>toggleDescending(order)}>
                <Icon data={order.descending === true?checkedIcon:uncheckIcon}/>
            </div>
        </div>
    {/each}

    <CircleAddButton title="增加排序" on:click={()=>addOrder()}/>
</div>
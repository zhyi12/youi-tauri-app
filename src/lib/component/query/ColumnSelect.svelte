<script>
    import {Icon} from "../../youi";
    import {createEventDispatcher} from "svelte";

    const dispatch = createEventDispatcher();

    export let text = '';

    export let property = '';

    export let columns = [];

    export let icons = (_name)=>undefined;

    export let offset = 0;

    let hovering = false;

    const select = (item) => {
        dispatch('select',{...item});
    }

    $:translateX = offset*-1-100;

</script>

<div class="dropdown dropdown-up"
     on:mouseenter={()=>hovering = true}
     on:mouseleave={()=>hovering = false}
>
    <div tabindex="0" role="button" class="select-text">
        {text}
    </div>
    {#if hovering}
        <ul class="fixed dropdown-content select-content" style={`transform: translateX(${translateX}px);`}>
            {#each columns as item}
                <li title={item.text} on:mousedown={()=>select(item)}>
                    <a class:active={item.name === property}>
                        <Icon data={icons(item.dataType)}/>
                        {item.text}
                    </a>
                </li>
            {/each}
        </ul>
    {/if}
</div>

<style lang="scss">
    .select-text{
      @apply text-green-700 min-w-16 h-8 p-1 flex items-center whitespace-nowrap;
    }

    .select-content{
      @apply menu w-48 bg-base-100 rounded border overflow-x-hidden z-[2];
    }
</style>
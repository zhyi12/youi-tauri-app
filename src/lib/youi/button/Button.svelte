<script context="module" lang="ts">

    export interface ButtonInfo{
        name:string,
        icon?:string,
        text?:string,
        title?:string,
        tooltipPosition?:string
    }

</script>

<script>

    import classnames from "../util/classname";
    import Tooltip from "../tooltip/Tooltip.svelte";

    let className = '';
    export { className as class };

    export let title = '';

    export let disabled = false;

    export let tooltipPosition = 'bottom';

    $: classes = classnames('btn', className,disabled);
</script>

<Tooltip {title} class={`tooltip-${tooltipPosition}`}>
    <button {disabled} class={classes} on:click>
        <slot/>
    </button>
</Tooltip>



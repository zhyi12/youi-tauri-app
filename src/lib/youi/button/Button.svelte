<script>

    import classnames from "../util/classname";
    import Tooltip from "../tooltip/Tooltip.svelte";

    let className = '';
    export { className as class };

    export let title = '';

    export let tooltipPosition = 'bottom';

    $: classes = classnames('btn', className);
</script>

<Tooltip {title} class={`tooltip-${tooltipPosition}`}>
    <button class={classes} on:click>
        <slot/>
    </button>
</Tooltip>



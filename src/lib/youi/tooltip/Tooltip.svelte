<script>
    import classnames from "../util/classname";
    let className = '';
    export { className as class };

    export let title = '';

    $: classes = classnames("tooltip", className);

</script>

{#if title}
    <div class={classes} data-tip={title}>
        <slot/>
    </div>
{/if}
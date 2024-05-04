<script>

    import classnames from '../util/classname';

    let className = '';
    export { className as class };

    export let nav = false;

    let component;

    $: classes = classnames('youi-dropdown dropdown');

</script>

<div {...$$restProps} class={classes} bind:this={component}>
    <slot />
</div>

<!--<div class="dropdown">-->
<!--    <div tabindex="0" role="button" class="btn m-1">Click</div>-->
<!--    <ul tabindex="0" class="dropdown-content z-[1] menu p-2 shadow bg-base-100 rounded-box w-52">-->
<!--        <li><a>Item 1</a></li>-->
<!--        <li><a>Item 2</a></li>-->
<!--    </ul>-->
<!--</div>-->
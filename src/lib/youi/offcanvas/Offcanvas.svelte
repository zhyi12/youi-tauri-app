<script>
    import classnames from "../util/classname";
    import {uuid} from "../util/uuid";

    let className = '';
    export { className as class };

    /**
     * uuid
     */
    export let id = `offcanvas-${uuid()}`;

    /**
     * 是否显示
     * @type {boolean}
     */
    export let isOpen = false;

    export let zIndex = 999;

    /**
     * 内容区域样式
     * @type {string}
     */
    export let bodyClass = 'p-0 w-10/12';

    export let contentClass = '';

    $: classes = classnames(`drawer drawer-end z-[${zIndex}]`, className);

</script>

<div class={classes}>
    <input {id} type="checkbox" checked={isOpen} class="drawer-toggle"/>
    <div class="drawer-side">
        <label for={id} aria-label="close sidebar" on:mousedown={()=>isOpen = false} class="drawer-overlay"></label>
        <div class:min-h-full={true}
             class:bg-white={true}
             class={bodyClass}>
            <div class={contentClass}>
                <slot/>
            </div>
        </div>
    </div>
</div>

<script lang="ts">

    import classnames from "../util/classname";
    import UnorderedList from "./UnorderedList.svelte";
    import OrderedList from "./OrderedList.svelte";
    import RecursiveListItem from "./RecursiveListItem.svelte";

    let className = '';
    export {className as class};

    export let icons = ()=>undefined;

    /**
     * @typedef {{ text?: string; href?: string; html?: string; }} RecursiveListNode
     * @restProps {ul | ol}
     */

    /**
     * Specify the children to render
     * @type {Array<RecursiveListNode & { children?: RecursiveListNode[]; }>}
     */
    export let children = [];

    /**
     * item样式
     * @type {string}
     */
    export let itemClass = '';

    /**
     * Specify the type of list to render
     * @type {"unordered" | "ordered" | "ordered-native"}
     */
    export let type = "unordered";

    export let level = 1;

    $: classes = classnames("list", className);

</script>

<svelte:component
        this="{type === 'unordered' ? UnorderedList : OrderedList}"
        native="{type === 'ordered-native'}"
        class={classes}
>
    {#each children as child}
        {#if Array.isArray(child.children)}
            <RecursiveListItem {itemClass} {...{
                id: child.id,
                name: child.name,
                text: child.text,
                html: child.html,
                href: child.href,
                icon: child.icon,
                level:level,
                icons
            }}>
                <svelte:self {...{itemClass,icons,level:level+1,children: child.children}} type="{type}"/>
            </RecursiveListItem>
        {:else}
            <RecursiveListItem {itemClass} {...{
                id: child.id,
                name: child.name,
                text: child.text,
                html: child.html,
                href: child.href,
                icon: child.icon,
                level:level,
                icons
            }}/>
        {/if}
    {/each}
</svelte:component>

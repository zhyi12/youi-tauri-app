<script>
    import { createEventDispatcher, setContext } from 'svelte';
    import { writable } from 'svelte/store';
    import TabHeader from './TabHeader.svelte';
    import classnames from "../util/classname";
    import {uuid} from "../util/uuid";

    const dispatch = createEventDispatcher();

    let className = '';
    export { className as class };

    export let id = `tabs-${uuid()}`;
    //lifted bordered
    export let headerStyle = 'lifted';

    let contentId = writable(id);
    const activeTabId = writable();
    setContext('tabContent', {
        contentId,
        activeTabId,
        setActiveTab: (tabId) => {
            activeTabId.set(tabId);
            dispatch('tab', tabId);
        }
    });

    $: classes = classnames('youi-tab flex flex-col flex-1', className);
</script>

<div {...$$restProps} class={classes}>
    <TabHeader class={`tabs-${headerStyle}`}>
        <slot/>
    </TabHeader>
    <slot/>
</div>
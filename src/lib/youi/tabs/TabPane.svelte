<script lang="ts">
    import { getContext ,onMount} from 'svelte';
    import classnames from "../util/classname";

    let className = '';
    export { className as class };
    export let active = false;
    export let disabled = false;
    export let tab = undefined;
    export let tabId = undefined;

    const tabs = getContext('tabs');
    const { contentId,activeTabId, setActiveTab } = getContext('tabContent');

    $: tabOpen = $activeTabId === tabId;

    $: classes = classnames('tab-content flex-1 overflow-auto border-base-300', className);

    onMount(() => {
        if (active) setActiveTab(tabId);
    });
</script>

{#if tabs}
    <label class:tab-active={tabOpen} class:text-rose-600={tabOpen} class="tab" for={tabId}>
        {tab}
    </label>
{:else}
    <input type="radio" id={tabId}
           name={`${$contentId}_tab`} on:change={(e)=>{
               if(e.target.checked){
                   setActiveTab(tabId)
               }
           }}
           checked={active} role="tab" class="tab h-0 absolute hidden"/>
    <div {...$$restProps} class={classes}>
        <slot />
    </div>
{/if}

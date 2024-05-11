<script>

    import {Button,Icon} from "../youi";
    import {APP_ICONS} from "../app-icons/icons";
    import {backwardIcon,forwardIcon} from "../app-icons";
    import {getContext} from "svelte";

    const {canBack,canForward} = getContext("AppContext");

    export let navPaths = [];

    let backCount = 0;

    const goBack = () => {
        window.history.back();
        backCount++;
        $canForward = true;
        if(backCount>=window.history.length-1){
            $canBack = false;
        }
    }

    const goForward = ()=>{
        window.history.forward();
        backCount--;
        if(backCount <= 0){
            $canForward = false;
        }
    }

</script>

<Button class="btn-ghost btn-xs p-1" disabled={!$canBack} title="后退" on:click={()=>goBack()}>
    <Icon data={backwardIcon} class="w-4 h-4"/>
</Button>
<Button class="btn-ghost btn-xs p-1 " disabled={!$canForward} on:click={()=>goForward()} title="前进">
    <Icon data={forwardIcon} class="w-4 h-4"/>
</Button>
<div class="text-sm breadcrumbs">

    <ul>
        {#each navPaths as path,index}
            {#if path.text}
                <li>
                    <a href={path.href}>
                        {#if APP_ICONS[path.icon]}
                            <Icon data={APP_ICONS[path.icon]} class="w-4 h-4"/>
                        {/if}
                        {path.text}
                    </a>
                </li>
            {/if}
        {/each}

    </ul>
</div>
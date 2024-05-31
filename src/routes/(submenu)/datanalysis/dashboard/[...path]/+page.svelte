<script>

    import {afterNavigate} from "$app/navigation";
    import {createStore} from "$lib/app-stores/dmp/dashboardStore";

    let store = createStore();

    export let data;

    afterNavigate(({from})=>{
        if(from && data.record.id){
            store.fetchByGroup(data.record.id)
        }
    })
</script>

<div class="grid grid-cols-4 gap-4">
    {#if $store.records}
        {#each $store.records as record}
            <div class="card w-64 glass m-3">
                <figure>
                    <img src="" alt="封面!"/>
                </figure>
                <div class="card-body">
                    <a href={`${data.baseUri}-window-designer-${record.id}`} class="card-title">{record.text}</a>
                </div>
            </div>
        {/each}
    {/if}
</div>
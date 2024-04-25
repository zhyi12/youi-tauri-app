<script>

    import classnames from "../util/classname";
    import {uuid} from "../util/uuid";

    let className = '';
    export { className as class };

    export let id = `modal-${uuid()}`;

    export let isOpen = false;

    let container = undefined;

    $: classes = classnames('modal', className);

    $: if(isOpen === true){
        container && container.showModal();
    }

</script>

<dialog {id} class={classes}
        on:close={()=>{
            isOpen = false;
        }}
        bind:this={container}>
    <div class="modal-box">
        <slot/>
    </div>
    <form method="dialog" class="modal-backdrop">
        <button>close</button>
    </form>
</dialog>
<script lang="ts">
    import {createEventDispatcher, onMount} from "svelte";

    const dispatch = createEventDispatcher();

    export let x = -1;
    export let y = 0;
    export let width = 100;
    export let height = 20;

    export let value;

    let openValue = '';
    let inputDom = undefined;

    $:if(x>-1 && inputDom){
        inputDom.focus()
    }

    const handle_input_keydown = (event) => {
        const {keyCode} = event;
        if(13 === keyCode && !event.shiftKey){
            close();
            event.preventDefault();
        }

        //左右
        if(37 === keyCode){
            if(event.target.selectionEnd>0){
                event.stopPropagation()
            }
        }else if(39 === keyCode){
            const end = (value+'').length;
            if(event.target.selectionEnd < end){
                event.stopPropagation()
            }
        }
    }

    const close = () => {
        dispatch('close',{value});
        x=-1000;
        y=-1000;
        width = 0;
        height = 0;
    }

    onMount(()=>{

    })
</script>

<div class="youi-editor"
     style:left={`${x}px`}
     style:top={`${y}px`}
     style:width={`${width}px`}
     style:height={`${height}px`}
     on:mousedown={(event)=>{
         event.stopPropagation();
     }}
>
    <input type="hidden" bind:value/>
    <textarea class="leading-4 p-0.5" style:height={`${height}px`} style:width={`${width}px`} bind:value
              on:keydown={handle_input_keydown} bind:this={inputDom}
              on:blur={()=>close()}></textarea>
</div>

<style>
  .youi-editor{
    position: absolute;
    border:1px solid #0b5ed7;
    min-height: 20px;
    min-width: 40px;
    left: 0px;
    top:0px;
  }
</style>
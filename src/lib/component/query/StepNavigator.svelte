<script lang="ts">

    import type {IStep} from "./DataQuery.d";

    import {createEventDispatcher} from "svelte";
    import {Input,Icon, listIcon, removeIcon, renameIcon,plusIcon} from "../../youi";
    import {STEP_MENUS} from "./helper";

    const dispatch = createEventDispatcher();

    export let steps:IStep[] = [];

    export let activeIndex = 0;

    export let icons = (_name)=>undefined;

    let hoverIndex = -1;

    let renameIndex = -1;

    let oldName = '';

    let renameInput;

    /**
     *
     * @param step
     * @param index
     */
    const gotoStep = (step,index) => {
        dispatch('step',{step,index});
    }

    const removeStep = (step,index) => {
        dispatch('remove',{step,index,isLast:index === steps.length-1});
    }

    const addStep = (step,index) => {
        dispatch('add',{step,index});
    }

    const rename = (step,index) => {
        renameIndex = index;
        oldName = step.text;
    }

    const nameChanged = (step,index) => {
        dispatch('renamed',{step,index,oldName});
    }

    $:if(renameInput){
        renameInput.focus()
    }

</script>

<ul class="steps steps-vertical text-lg w-full content-start gap-y-1">
    {#each steps as step,index}
        <li class="step w-full relative hover:bg-sky-100 mb-1 p-1"
            class:bg-sky-100={index === activeIndex}
            on:mouseenter={()=>hoverIndex = index}
            on:mouseleave={()=>hoverIndex = -1}
            on:click={()=>gotoStep(step,index)}
            on:dblclick={()=>rename(step,index)}
            class:step-primary={index<=activeIndex}>
            <div class="step-row tooltip tooltip-bottom w-full" data-tip={step.text}
                 class:text-green-700={index === activeIndex}>
                <Icon class="w-4 h-4 text-center" data={icons(step.name)||listIcon}/>
                <span class="pl-0.5 max-w-28 inline-block overflow-hidden whitespace-nowrap">
                    {#if renameIndex === index}
                        <Input on:blur={()=>renameIndex = -1}
                               on:change={()=>nameChanged(step,index)}
                                bind:inner={renameInput} on:click={e=>e.stopPropagation()} bind:value={step.text} />
                    {:else}
                        {step.text}
                    {/if}
                </span>
                {#if renameIndex !== index && index>0 && (hoverIndex === index || activeIndex === index)}
                    <div class="absolute right-0 bg-gray-100 rounded-md">
                        <span class:hover:text-green-500={true} on:click|stopPropagation={()=>rename(step,index)}>
                            <Icon data={renameIcon}/>
                        </span>
                        <span class="mt-[-2px] inline-block z-[2] w-4 h-4"
                              class:hover:text-red-500={true} on:click|stopPropagation={()=>removeStep(step,index)}>
                            <Icon scale={0.96} class="w-4" data={removeIcon}/>
                        </span>
                    </div>
                {/if}
            </div>

            {#if (hoverIndex === index || index === steps.length-1)}
                {@const isLast = index === steps.length-1}
                <div on:click|stopPropagation
                     class:last={isLast}
                     class="step-add">
                    <div class="dropdown w-30" class:dropdown-top={index>4}>
                        <div tabindex="0" role="button" class="add-toggle">
                            <Icon scale={isLast?1.2:0.75} data={plusIcon}/>
                        </div>
                        <ul class="menu fixed dropdown-content w-36 z-[5] bg-white border rounded">
                            {#each STEP_MENUS as menu}
                                <li on:mousedown={()=>addStep({...menu},index)}>
                                    <a>
                                        <Icon class="w-4" data={icons(menu.name.toLowerCase())||listIcon}/>
                                        {menu.text}
                                    </a>
                                </li>
                            {/each}
                        </ul>
                    </div>
                </div>
            {/if}
        </li>
    {/each}
    <li class="h-full">

    </li>
</ul>

<style lang="scss">

    .step{
      height: 4rem;
      .step-row{
        @apply flex flex-row leading-4 text-sm cursor-pointer;
      }
      .step-add{
        @apply z-[2] absolute leading-4 w-2 h-2 left-4 bottom-0 justify-center;

        .add-toggle{
          @apply w-4 h-4 text-center bg-blue-50 rounded-full hover:cursor-pointer hover:bg-blue-200;
        }

        &.last{
          @apply left-2 -bottom-2;
          .add-toggle{
            line-height: 1.8rem;
            @apply w-8 h-8 left-2
          }
        }
      }
    }
</style>
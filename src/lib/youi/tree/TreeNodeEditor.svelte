<script lang="ts">
    import {Tooltip,Icon,saveIcon,closeIcon,ellipsisHIcon} from "../index";

    export let id:number;

    export let text:string;

    export let hovered = false;

    export let selected = false;

    export let buttons = [];

    let newName = text;

    let renaming = false;
    let more = false;

    $:renameButton = buttons.filter(b=>b.name === 'rename')[0];

    $:showButtons = buttons.length>3?[...buttons.slice(0,2)]:buttons;
    $:moreButtons = buttons.length>3?buttons.slice(2):[];

    /**
     *
     */
    const doRename = () => {
        if(newName && text !== newName){
            renaming = false;
            if(renameButton && renameButton.action){
                renameButton.action(id,newName);
            }
        }
    }

    const handle_rename = (event) => {
        if(event.keyCode === 13 && newName){
            renaming = false;
            doRename();
        }
    }

    const applyButtonAction = (button) => {
        if(button.name === 'rename'){
            renaming = true;
        }else if(button.action){
            button.action(id,text);
        }
        more = false;
    }
</script>

{#if renaming}
    <input class="w-full" type="text" bind:value={newName}
           on:dblclick|stopPropagation
           on:mousedown|stopPropagation on:keydown={handle_rename}/>
    <Tooltip title="保存修改">
        <a class="p-1 hover:bg-gray-200 rounded-md" on:click|stopPropagation={()=>{doRename()}}>
            <Icon class="w-4 h-4" data={saveIcon}/>
        </a>
    </Tooltip>
    <Tooltip title="取消">
        <a class="p-1 hover:bg-gray-200 rounded-md" on:click|stopPropagation={()=>{renaming = false}}>
            <Icon class="w-4 h-4" data={closeIcon}/>
        </a>
    </Tooltip>
{:else}
    <div class="tooltip whitespace-nowrap" data-tip={text}>
        <div class="overflow-hidden whitespace-normal h-6">{text}</div>
    </div>
    {#if hovered || selected}
        <div class="flex-1">

        </div>
        <div class="self-end absolute right-2 bg-blue-50">
            {#each showButtons as button}
                <Tooltip title={button.title||button.text} class="tooltip-bottom">
                    <a class="p-1 hover:bg-gray-200 rounded-md" on:click|stopPropagation={()=>{applyButtonAction(button)}}>
                        <Icon class="w-4 h-4" data={button.icon}/>
                    </a>
                </Tooltip>
            {/each}

            {#if moreButtons.length}
                <div class="dropdown dropdown-end">
                    <Tooltip title="更多" class="tooltip-bottom">
                        <div tabindex="0" role="button" on:mousedown|stopPropagation on:dblclick|stopPropagation>
                            <Icon class="w-4 h-4 pr-2" data={ellipsisHIcon}/>
                        </div>
                    </Tooltip>
                    <ul tabindex="0" class="dropdown-content z-[1] menu p-2 shadow bg-gray-50 rounded-box w-52">
                        {#each moreButtons as button}
                            <li>
                                <a on:click|stopPropagation={()=>applyButtonAction(button)}>
                                    <Icon class="w-4 h-4" data={button.icon}/>
                                    {button.text}</a>
                            </li>
                        {/each}
                    </ul>
                </div>
            {/if}
        </div>
    {/if}
{/if}

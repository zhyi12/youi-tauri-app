<script lang="ts">
    import {Tooltip,Icon,saveIcon,closeIcon,ellipsisHIcon} from "../index";

    export let id:number;

    export let text:string;

    export let group:string = undefined;

    export let hovered = false;

    export let selected = false;

    export let buttons = [];

    // 存储初始值
    let orgText = text;

    let renaming = false;
    let more = false;

    $:nodeButtons = buttons.filter(b=>{
        if(group && b.groups && !b.groups.includes(group)){
            return false;
        }
        return true
    });

    $:renameButton = nodeButtons.filter(b=>b.name === 'rename')[0];

    $:showButtons = nodeButtons.length>3?[...nodeButtons.slice(0,2)]:nodeButtons;
    $:moreButtons = nodeButtons.length>3?nodeButtons.slice(2):[];

    /**
     *
     */
    const doRename = () => {
        if(text && text !== orgText){
            renaming = false;
            if(renameButton && renameButton.action){
                renameButton.action(id,text);
            }
        }
    }

    const handle_rename = (event) => {
        if(event.keyCode === 13 && text){
            renaming = false;
            doRename();
        }
    }

    const applyButtonAction = (button) => {
        if(button.name === 'rename'){
            renaming = true;
        }else if(button.action){
            button.action(id,text,group);
        }
        more = false;
    }
</script>

{#if renaming}
    <input class="w-full" type="text" bind:value={text}
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
    <div class="tooltip whitespace-nowrap tooltip-bottom" data-tip={text}>
        <div class="overflow-hidden h-6">{text}</div>
    </div>
    {#if hovered || selected}
        <div class="flex-1">

        </div>
        <div class="self-end absolute right-1 bg-blue-50 rounded-md">
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
                        <a tabindex="0" role="button" class="p-1 hover:bg-gray-200 rounded-md"
                             on:mousedown|stopPropagation
                             on:click|stopPropagation
                             on:dblclick|stopPropagation>
                            <Icon class="w-4 h-4" data={ellipsisHIcon}/>
                        </a>
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

<script lang="ts">
    import {Icon,listIcon} from "../youi";
    import type {MenuInfo} from "../app-entity/base/menu";
    import {APP_ICONS} from "../app-icons/icons";
    import Tooltip from "../youi/tooltip/Tooltip.svelte";

    export let menus:MenuInfo[] = [];

    /**
     * 当前路径所在的模块
     */
    export let activeModule:string = undefined;

</script>

<nav class="mt-8 truncate">
    <ul role="list" class="items-center flex flex-col space-y-1">
        <li>
            <Tooltip title="工作台" class="tooltip-bottom">
                <a href="/" class="leading-6 font-semibold text-sm p-3 rounded-md gap-x-3 flex text-gray-400 hover:text-sky-400"
                    class:bg-gray-800={!activeModule}
                    class:text-sky-400={!activeModule}
                >
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true"
                         class="shrink-0 w-6 h-6"><path stroke-linecap="round" stroke-linejoin="round" d="M2.25 12l8.954-8.955c.44-.439 1.152-.439 1.591 0L21.75 12M4.5 9.75v10.125c0 .621.504 1.125 1.125 1.125H9.75v-4.875c0-.621.504-1.125 1.125-1.125h2.25c.621 0 1.125.504 1.125 1.125V21h4.125c.621 0 1.125-.504 1.125-1.125V9.75M8.25 21h8.25"></path></svg>
                    <span class="hidden">Dashboard</span>
                </a>
            </Tooltip>
        </li>

        {#each menus as menu}
            <Tooltip title={menu.text} class="tooltip-bottom">
                <li>
                    <a href={menu.href}
                       class:bg-gray-800={activeModule === menu.name}
                       class:text-sky-400={activeModule === menu.name}
                       class="leading-6 font-semibold text-sm p-3 rounded-md gap-x-3 flex text-gray-400  hover:text-sky-400 hover:bg-gray-800">
                        <Icon data={APP_ICONS[menu.icon||menu.name]||listIcon} class="shrink-0 w-6 h-6"/>
                    </a>
                </li>
            </Tooltip>
        {/each}
    </ul>
</nav>
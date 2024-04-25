<script lang="ts">

    import {setContext} from "svelte";
    import {writable} from "svelte/store";

    import type {MenuInfo} from "../app-entity/base/menu";
    import RecursiveList from "../youi/list/RecursiveList.svelte";
    import {APP_ICONS} from "../app-icons/icons";

    export let submenu:MenuInfo = {id:'',text:'',name:'',children:[]};

    let selectedNodeId = writable("");

    setContext("ListTree",{
        selectedNodeId
    });

    /**
     * 当前页面的pathname
     */
    export let pathname = undefined;

    $: if(pathname){
        calculateActiveHref(submenu.children,pathname)
    }

    /**
     * 计算当前选择的菜单
     * @param menus
     * @param pathname
     */
    function calculateActiveHref(menus,pathname) {
        menus.some(menu=>{
            if (matchMenuHref(menu,pathname)){
                $selectedNodeId = menu.id;
                return false;
            }else if(menu.children){
                calculateActiveHref(menu.children,pathname);
            }
        });
    }

    function matchMenuHref(menu,pathname) {
        if(menu.href === pathname){
            return true;
        }
        return false;
    }

    const icons = (icon)=>{
        return APP_ICONS[icon] || APP_ICONS['list']
    }
</script>

<RecursiveList {icons}
               class="menu w-52 rounded-box space-y-1"
               itemClass="pt-1.5 pb-1.5 mt-0.5"
               children={submenu.children}>
</RecursiveList>
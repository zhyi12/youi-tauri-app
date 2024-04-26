<script lang="ts">
    import SubMenu from "$lib/app-page/SubMenu.svelte";
    import type { LayoutData } from './$types';

    export let data: LayoutData;

</script>
<!-- 子模块菜单 -->
<aside class="border-gray-200 w-52 hidden fixed lg:block lg:left-20 top-12 border-r h-dvh">
    <SubMenu submenu={data.submenu} pathname={data.pathname}/>
</aside>
<div class="lg:pl-52 flex-1 flex flex-col">
    <slot/>
</div>

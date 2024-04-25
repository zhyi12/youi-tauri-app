<script>
    import {Icon,homeIcon,folderOIcon,hddOIcon} from "../youi";
    import PageNav from "./PageNav.svelte";

    export let navPaths = [];


</script>

<div class="shadow items-center h-12 flex shrink-0 pl-2">
    <div class="text-sm breadcrumbs">
        <PageNav {navPaths}/>
<!--        <ul>-->
<!--            <li>-->
<!--                <a>-->
<!--                    <Icon data={homeIcon} class="w-4 h-4"/>-->
<!--                    首页-->
<!--                </a>-->
<!--            </li>-->
<!--            <li>-->
<!--                <a>-->
<!--                    <Icon data={hddOIcon} class="w-4 h-4"/>-->
<!--                    云盘-->
<!--                </a>-->
<!--            </li>-->
<!--            <li>-->
<!--              <span class="inline-flex gap-1 items-center">-->
<!--                <Icon data={folderOIcon} class="w-4 h-4"/>文件夹-->
<!--              </span>-->
<!--            </li>-->
<!--        </ul>-->
    </div>
</div>
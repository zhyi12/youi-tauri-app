<script>
    import {Icon, plusIcon} from "../../youi";
    export let title = '新增';
</script>

<div data-tip={title}
     on:click
     class="tooltip tooltip-bottom float-left bg-blue-50 w-8 h-8 cursor-pointer hover:text-red-500 rounded-full flex items-center justify-center">
    <Icon scale={0.8} data={plusIcon}/>
</div>
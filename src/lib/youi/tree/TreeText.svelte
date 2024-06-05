<script lang="ts">
    export let level = 1;
    export let indent = 0;
    export let selected = false;
</script>

<a style={`padding-left:${(level)*16+indent}px`}
   class:bg-blue-100={selected}
   class:hover:bg-blue-50={selected?false:true}
   on:click|stopPropagation
   on:mouseenter
   on:mouseleave
   class="inline-block w-full py-0.5 whitespace-nowrap leading-6 relative flex rounded-md hover:cursor-pointer mt-0" on:dblclick>
    <slot/>
</a>
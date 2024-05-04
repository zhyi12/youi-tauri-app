
<script>
    export let active = false;
</script>

<li>
    <a class:active on:click on:mousedown>
        <slot/>
    </a>
</li>
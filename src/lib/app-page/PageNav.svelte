<script>

    import {Icon} from "../youi";
    import {APP_ICONS} from "../app-icons/icons";

    export let navPaths = [];

</script>

<ul>

    {#each navPaths as path,index}
        <li>
            <a href={path.href}>
                {#if APP_ICONS[path.icon]}
                    <Icon data={APP_ICONS[path.icon]} class="w-4 h-4"/>
                {/if}
                {path.text}
            </a>
        </li>
    {/each}

</ul>
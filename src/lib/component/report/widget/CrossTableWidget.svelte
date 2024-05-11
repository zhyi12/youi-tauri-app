<script>
    import {CrossTable} from "../../../youi";

    let className = '';
    export { className as class };

    export let id = undefined;

    export let name = undefined;

    export let slaveRows = 1;

    export let mainColumns = 1;

    export let footerRows;

    export let data = [];

    export let colModels = [];

    export let mergedCells = [];

    export let active = false;

    export let editable = false;

    export let scrollTop = 0;

    export let scrollLeft = 0;

    export let showYScroll = false;

    export let contentHeight = undefined;

    export let tableHeight;

    let selections = [];

    $:if(active === false){
        selections = [];
    }

</script>

<CrossTable bind:tableHeight class={className}
        {data} {colModels} {mergedCells} {slaveRows} {mainColumns} {footerRows}
            {selections} {scrollLeft} {scrollTop} {editable}
            {contentHeight}
            showXScroll = {false}
            {showYScroll}
            on:selection-stop
            on:col-resize
            on:edited
            on:focus
            on:table-height
/>


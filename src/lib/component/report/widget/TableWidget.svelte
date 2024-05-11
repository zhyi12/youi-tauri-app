<script>
    import {Grid} from "../../../youi/index";

    export let id = undefined;

    export let name = undefined;

    export let data = [];

    export let colModels = [];

    export let mergedCells = [];

    export let active = false;

    export let scrollLeft = 0;


    /**
     * Number of frozen rows
     */
    export let frozenRows = 0;

    /**
     * Number of frozen columns
     */
    export let frozenColumns = 0;

    export let editable = false;

    let selections = [];

    $:if(active === false){
        selections = [];
    }

</script>

<Grid {selections} {data} {colModels} {mergedCells} {scrollLeft}
      {frozenRows} {frozenColumns} {editable}
      showYScroll={false}
      on:selection-stop
      on:col-resize
/>
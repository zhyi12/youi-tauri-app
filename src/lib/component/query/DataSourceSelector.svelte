<script>
    import {TabContent, TabPane,List} from "../../youi";
    import {execute} from "../../tauri/dsl";
    import FieldUpload from "../../tauri/FieldUpload.svelte";
    import {DATATYPE_ICONS} from "../../app-icons";

    export let columns = [];

    export let uri  = '';

    export let reader = '';

    export let selectedColumnNames = [];

    let activeId;

    const icons = (name) => {
        return DATATYPE_ICONS[name];
    }

    const handle_tab = ({detail}) => {
        activeId = detail;
    }

    const handle_upload = ({detail}) => {
        selectedColumnNames = [];
        let path = detail.value;
        if(path.endsWith('.csv')){
            reader = 'CsvReader';
        }
        execute(`read_csv_header("${path}")`,[]).then(result=>{
            columns = result.map(item=>({
                name:item.name,
                text:item.name,
                dataType:item.dataType
            }));
        });
    }

    $:items = columns.map((c,index)=>({
        id:c.name,
        name:c.name,
        text:c.text,
        group:c.dataType,
        datas:{num:index}
    }));
</script>

<TabContent on:tab={handle_tab} class="h-full">
    <TabPane tab="制度数据源" tabId="statsDs" active={true}>
        stats
    </TabPane>
    <TabPane tab="文件数据源" tabId="fileDs">
        <div class="h-full flex flex-col p-2">
            <!--    激活状态显示，切换到其他tab时实时销毁    -->
            {#if activeId === "fileDs"}
                <FieldUpload extensions={["csv"]} on:change={handle_upload} bind:value={uri}/>
                <List class="flex-1 overflow-auto border"
                      bind:selectedIds={selectedColumnNames} {icons} multiple={true} items={items}/>
            {/if}
        </div>
    </TabPane>
</TabContent>

<script lang="ts">

    import {derived} from "svelte/store";
    import {Toolbar,Button,Icon,listIcon} from "$lib/youi";
    import Report from "$lib/component/report/Report.svelte";
    import {createReportStore} from "$lib/component/report/store";
    import type {ReportModel} from "$lib/component/report/Report.d";
    import {undoIcon,redoIcon} from "../../../../../lib/app-icons";

    let data = [
        {text:'省级区划名称',verticalAlign:'middle'},{text:'地级区划数',verticalAlign:'middle'},{corner:true},{text:'县级区划数',verticalAlign:'middle'},{corner:true},{},{},{},{text:'乡级区划数'},{corner:true},{},{},
        {},{},{text:'#地级市'},{},{text:'#市辖区'},{text:'#县级市'},{text:'#县'},{text:'#自治县'},{},{text:'#镇'},{text:'#乡'},{text:'#街道'},
        {},{},{},{},{},{},{},{},{},{},{},{},
        {text:'全国',letterSpacing:94},{text:100},{text:200},{text:300},{text:400},{text:500},{text:600},{text:700},{text:800},{text:900},{text:1000},{text:1100},
        {},{},{},{},{},{},{},{},{},{},{},{},
        {text:'北京市',letterSpacing:40},{},{},{},{},{},{},{},{},{},{},{},
        {text:'天津市',letterSpacing:40},{},{},{},{},{},{},{},{},{},{},{},
        {text:'河北省',letterSpacing:40},{},{},{},{},{},{},{},{},{},{},{},
        {text:'山西省',letterSpacing:40},{},{},{},{},{},{},{},{},{},{},{},
        {text:'内蒙古自治区',letterSpacing:8},{},{},{},{},{},{},{},{},{},{},{},
        {},{},{},{},{},{},{},{},{},{},{},{},
        {text:'辽宁省',letterSpacing:40},{},{},{},{},{},{},{},{},{},{},{},
        {text:'吉林省',letterSpacing:40},{},{},{},{},{},{},{},{},{},{},{},
        {text:'黑龙江省',letterSpacing:22},{},{},{},{},{},{},{},{},{},{},{},
        {},{},{},{},{},{},{},{},{},{},{},{},
        {text:'上海市',letterSpacing:40},{},{},{},{},{},{},{},{},{},{},{},
        {text:'江苏省',letterSpacing:40},{},{},{},{},{},{},{},{},{},{},{},
        {text:'浙江省',letterSpacing:40},{},{},{},{},{},{},{},{},{},{},{},
        {text:'安徽省',letterSpacing:40},{},{},{},{},{},{},{},{},{},{},{},
        {text:'福建省',letterSpacing:40},{},{},{},{},{},{},{},{},{},{},{},
        {text:'江西省',letterSpacing:40},{},{},{},{},{},{},{},{},{},{},{},
        {text:'山东省',letterSpacing:40},{},{},{},{},{},{},{},{},{},{},{},
        {},{},{},{},{},{},{},{},{},{},{},{},
        {text:'河南省',letterSpacing:40},{},{},{},{},{},{},{},{},{},{},{},
        {text:'湖北省',letterSpacing:40},{},{},{},{},{},{},{},{},{},{},{},
        {text:'湖南省',letterSpacing:40},{},{},{},{},{},{},{},{},{},{},{},
        {text:'广东省',letterSpacing:40},{},{},{},{},{},{},{},{},{},{},{},
        {text:'广西壮族自治区',letterSpacing:4},{},{},{},{},{},{},{},{},{},{},{},
        {text:'海南省',letterSpacing:40},{},{},{},{},{},{},{},{},{},{},{},
        {},{},{},{},{},{},{},{},{},{},{},{},
        {text:'重庆市',letterSpacing:40},{},{},{},{},{},{},{},{},{},{},{},
        {text:'四川省',letterSpacing:40},{},{},{},{},{},{},{},{},{},{},{},
        {text:'贵州省',letterSpacing:40},{},{},{},{},{},{},{},{},{},{},{},
        {text:'云南省',letterSpacing:40},{},{},{},{},{},{},{},{},{},{},{},
        {text:'西藏自治区',letterSpacing:12},{},{},{},{},{},{},{},{},{},{},{},
        {},{},{},{},{},{},{},{},{},{},{},{},
        {text:'陕西省',letterSpacing:40},{},{},{},{},{},{},{},{},{},{},{},
        {text:'甘肃省',letterSpacing:40},{},{},{},{},{},{},{},{},{},{},{},
        {text:'青海省',letterSpacing:40},{},{},{},{},{},{},{},{},{},{},{},
        {text:'宁夏回族自治区',letterSpacing:4},{},{},{},{},{},{},{},{},{},{},{},
        {text:'新建维吾尔族自治区'},{},{},{},{},{},{},{},{},{},{},{},
        {text:''},{},{},{},{},{},{},{},{},{},{},{},
        {text:''},{},{},{},{},{},{},{},{},{},{},{},
    ];

    let colModels = [{width:130},{},{},{},{},{width:100},{},{},{},{},{},{}];
    let mergedCells = [
      {startRow:0, endRow:1, startCol:0, endCol:0},
      {startRow:0, endRow:1, startCol:1, endCol:1},
      {startRow:0, endRow:1, startCol:3, endCol:3},
      {startRow:0, endRow:1, startCol:8, endCol:8},
      {startRow:0, endRow:0, startCol:4, endCol:7},
      {startRow:0, endRow:0, startCol:9, endCol:11},
    ];

    const widgets = [
        {
            id:"w0",
            name:"Table",
            colModels:[{width:789},{},{},{}],
            frozenColumns:1,
            data:[{text: '1-1 全国行政区划',fontSize:16,fontStyle:'bold'},{},{},{},{},{},{},{},{text:'单位：个'},{},{},{}]
        },
        {
            id:"w1",
            name:"CrossTable",
            colModels,
            mergedCells,
            data,
            slaveRows:2,
            mainColumns:1,
            footerRows:2
        }
    ];

    const reportStore = createReportStore();
    const model:ReportModel = {widgets};
    reportStore.setModel(model)

    /**
     * 撤销、重做的响应式处理
     */
    $: canUndo = derived([reportStore],()=>reportStore.canUndo());
    $: canRedo = derived([reportStore],()=>reportStore.canRedo());
    $: dirty = derived([reportStore],()=>reportStore.isDirty());

    /**
     * 调整列宽度
     */
    const handle_col_resize = ({detail}) => {
        const {widgetId,columnIndex,value} = detail;
        reportStore.resizeColumn(widgetId,columnIndex,value);
    }

    const handle_edited = ({detail}) => {
        const {widgetId,rowIndex,columnIndex,text} = detail;
        reportStore.updateText(widgetId,rowIndex,columnIndex,text);
    }

</script>

<div class="flex-1 flex h-0 overflow-hidden">
    <div class="flex-1 flex flex-col">
        <Toolbar>
            <Button title="撤销" disabled={!$canUndo} on:click={()=>reportStore.undo()}>
                <Icon data={undoIcon}/>
            </Button>
            <Button title="重做" disabled={!$canRedo} on:click={()=>reportStore.redo()}>
                <Icon data={redoIcon}/>
            </Button>
            <Button title="设计">
                <Icon data={listIcon}/>
            </Button>
        </Toolbar>
        <Report widgets={$reportStore.widgets} editable={true}
                on:col-resize={handle_col_resize}
                on:edited={handle_edited}
        />
    </div>
</div>

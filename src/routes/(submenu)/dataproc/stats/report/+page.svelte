<script lang="ts">
    import {CrossTable} from "$lib/youi";

    let data = [
        {text:'省级区划名称'},{text:'地级区划数'},{corner:true},{text:'县级区划数'},{corner:true},{},{},{},{text:'乡级区划数'},{corner:true},{},{},
        {},{},{text:'#地级市'},{},{text:'#市辖区'},{text:'#县级市'},{text:'#县'},{text:'#自治县'},{},{text:'#镇'},{text:'#乡'},{text:'#街道'},
        {},{},{},{},{},{},{},{},{},{},{},{},
        {text:'全国',letterSpacing:94},{},{},{},{},{},{},{},{},{},{},{},
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
    ];

    let colModels = [{width:130},{},{},{},{},{},{},{},{},{},{},{}];
    let mergedCells = [
      {startRow:0, endRow:1, startCol:0, endCol:0},
      {startRow:0, endRow:1, startCol:1, endCol:1},
      {startRow:0, endRow:1, startCol:3, endCol:3},
      {startRow:0, endRow:1, startCol:8, endCol:8},
      {startRow:0, endRow:0, startCol:4, endCol:7},
      {startRow:0, endRow:0, startCol:9, endCol:11},
    ];

</script>

<div class="flex-1 flex h-0 overflow-hidden">
    <CrossTable {data} {colModels} {mergedCells} slaveRows={2}/>
</div>

<script>
    import {Toolbar,Button,Icon,saveIcon} from "$lib/youi";
    import MapBox from "$lib/component/mapbox/MapBox.svelte";
    import {blankStyle} from "$lib/component/mapbox/map-styles";
    import {tdtIcon} from "$lib/app-icons";

    import AreaBoundaryLayer from './_AreaBoundaryLayer.svelte'
    import SubBoundaryLayer from './_SubBoundaryLayer.svelte'
    import {afterNavigate} from "$app/navigation";
    import {findGeoJson} from "$lib/app-services/geo/areaGeoJsonServices";
    import {findSubGeoJson} from "$lib/tauri/geo";

    export let data;

    let zoom = 5;

    let center = {lng:110.1553,lat:16.0632};

    // 边界
    let boundary = undefined;

    // 下级边界
    let subBoundary = undefined;

    let map ;

    afterNavigate(({from})=>{
        if(data.nodeId && from){
            // 加载行政区划
            findGeoJson(data.nodeId,{district:data.district,tdtKey:data.appConfig.tdtKey}).then(geojson=>{
                boundary = geojson;
            })

            //findSubGeoJson
            findSubGeoJson(data.nodeId).then(r=>{
                subBoundary = r;
            });
        }
        boundary = null;
    })
</script>

<Toolbar>
    <Button title="从天地图更新">
        <Icon data={tdtIcon}/>
    </Button>
</Toolbar>

<MapBox class="flex-1" accessToken={data.appConfig.mapBoxKey} bind:this={map}
        {center} style = {blankStyle}
        {zoom} pitch={40}
        options={{ scrollZoom: true ,bearing:0}}
>
    <AreaBoundaryLayer areaId={data.nodeId} {boundary}/>
    <SubBoundaryLayer areaId={data.nodeId} boundary={subBoundary}/>
</MapBox>
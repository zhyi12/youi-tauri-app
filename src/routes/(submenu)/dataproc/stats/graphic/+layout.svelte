<script>

    // import {getConfigValue} from "$lib/util/page.util";
    import MapBox from "$lib/component/mapbox/MapBox.svelte";
    import {tdtStyle} from "$lib/component/mapbox/map-styles";
    // import {findSonTreeGeoJson} from "$lib/app-services/geo/geoJsonServices";
    import BoundaryLayer from "$lib/component/mapbox/BoundaryLayer.svelte";
    import {onMount} from "svelte";

    export let data;

    const tdtKey = 'da289cb6acc5479ea6ab4b225042d8a4';
    const mapboxKey = 'pk.eyJ1Ijoiemh5aTEyIiwiYSI6ImNsNjgyYWNzZDNxaHgzam8xbTJ0b2JtdGUifQ.FOHj9ZueVLkivDvj9fHPZA';
    const areaId = '';

    //天地图图层
    let style = tdtStyle(tdtKey);
    //-87.61694, 41.86625
    let center = {lng:113,lat:34};
    let map = null;
    let geoJsonList = [];


    /**
     * 生成规则多边形顶点个数
     * @param pointCount  顶点个数 3 => 三角形
     * @param start 顶点起始角度 调整图形的方向
     */
    function polygonPath(pointCount, start) {
        const step = (Math.PI * 2) / pointCount;
        const line = [];
        for (let i = 0; i < pointCount; i++) {
            line.push(step * i + (start * Math.PI) / 12);
        }
        const path = line.map((t) => {
            const x = Math.sin(t + Math.PI / 4);
            const y = Math.cos(t + Math.PI / 4);
            return [center.lng+x, center.lat+y, 0];
        });
        path.push(path[0]);
        return path;
    }

    function drawPie() {
        const count = 60;
        const paths = polygonPath(count,0);
        const center = [(paths[0][0]+paths[30][0])/2,(paths[0][1]+paths[30][1])/2];

        let part1 = paths.slice(0,10);
        part1.push(center);

        return [paths]
    }

    onMount(()=>{
        drawPie()


    })

    const geoJson = {
        "features": [
            {
                "type": "Feature",
                "properties": {
                    "level": 1,
                    "name": "Bird Exhibit",
                    "height": 500000,
                    "base_height": 0,
                    "color": "orange"
                },
                "geometry": {
                    "coordinates": [
                        drawPie()
                    ],
                    "type": "MultiPolygon"
                },
                "id": "06e8fa0b3f851e3ae0e1da5fc17e111e"
            },
        ],
        "type": "FeatureCollection"
    }

</script>

<MapBox class="flex-1" accessToken={mapboxKey}
        bind:this={map} {center} {style}
        zoom = {5} pitch={45}
        on:click={e=>{
            //
        }}

        options={{ scrollZoom: true }}
>

    <BoundaryLayer type="fill-extrusion"
                   color={(geoJson.properties && geoJson.properties.color)?geoJson.properties.color:''}
                   radius={(geoJson.properties && geoJson.properties.radius)?geoJson.properties.radius:2}
                   lineWidth={2}
                   lineColor={(geoJson.properties && geoJson.properties.color)?geoJson.properties.color:undefined}
                   name={`layer_${(geoJson.properties && geoJson.properties.id)?geoJson.properties.id:2}`} {geoJson}/>
</MapBox>

<slot/>
<script>

    import {contextKey} from "./mapbox";
    import {getContext, onMount} from "svelte";

    const { getMap, getMapbox } = getContext(contextKey)

    const map = getMap()

    export let id = undefined;

    onMount(() => {
        if(map){
            map.addSource(id,{
                type:'vector',
                "scheme":"tms",
                "tiles": ["http://localhost:8080/geoserver/gwc/service/tms/1.0.0/ne%3Acn@EPSG%3A900913@pbf/{z}/{x}/{y}.pbf"],
                crs: {
                    type: 'name',
                    properties: {
                        name: 'EPSG:4326'
                    }
                }
            });

            map.addLayer({
                id: 'tile-layer',
                type: 'line',
                source: id,
                "source-layer":'cn',
                paint: {
                    "line-color":"#a1aad0",
                    "line-width":1
                }
            });
        }

    })
</script>


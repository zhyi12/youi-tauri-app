<script>
    import { getContext } from 'svelte'
    import {contextKey} from "$lib/component/mapbox/mapbox";

    const { getMap, getMapbox } = getContext(contextKey)
    const map = getMap()
    const mapbox = getMapbox()

    export let boundary = undefined;

    export let areaId = '';

    let sourceId = 'boundary';
    let layerId = `layer_${sourceId}`;

    let layer;
    let source;
    
    const drawBoundary = () => {
        if(layer && source){
            map.removeLayer(layerId);
            map.removeSource(sourceId);
        }

        source = map.addSource(sourceId,{
            type: 'geojson',
            data:boundary
        });

        layer = map.addLayer({
            id:layerId,
            type: 'line',
            source: sourceId,
            paint:{
                'line-color':['case', ['boolean', ['feature-state', 'hover'], false], 'red','#63aad0'],
                'line-width':1,
            }
        });
        // 地图自适应
        if('MultiPolygon' === boundary.type){
            let coordinates;
            if(areaId === '460000000000'){
                coordinates = boundary.coordinates.flat().reduce((acc,v)=>{
                    if(v.length>acc[0]){
                        return [v.length,v];
                    }else {
                        return acc;
                    }
                },[0,{}])[1];
            }else{
                coordinates = boundary.coordinates.flat().flat();
            }

            const bounds = new mapboxgl.LngLatBounds(
                coordinates[0],
                coordinates[0]
            );

            for (const coord of coordinates) {
                bounds.extend(coord);
            }

            map.fitBounds(bounds, {
                padding: 20,
                pitch:0,
            });
        }
    }

    const clearBoundary = () => {
        if(layer && source){
            map.removeLayer(layerId);
            map.removeSource(sourceId);
            layer = undefined;
            source = undefined;
        }
    }
    
    $: boundary && drawBoundary();

    $: if(boundary == null){
        clearBoundary()
    }

</script>
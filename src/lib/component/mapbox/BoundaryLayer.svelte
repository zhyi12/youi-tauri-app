<script>
    import {createEventDispatcher, getContext, onMount} from "svelte";
    import {contextKey} from "./mapbox";

    const { getMap, getMapbox } = getContext(contextKey)
    const map = getMap()
    const mapbox = getMapbox();
    const dispatch = createEventDispatcher();

    export let name = 'geo_json_layer';
    export let color = '#fefefe';
    export let hoverColor = '';
    export let geoJson = undefined;
    export let lineWidth = 2;
    export let lineColor = '#000';
    export let type ='line';
    export let paint = undefined;
    export let radius = 3;

    const paints = {
        'line':{
            'line-color':['case', ['boolean', ['feature-state', 'hover'], false], 'red',lineColor||'#000'],
            'line-width':lineWidth||1,
        },
        'fill':{
            'fill-color': ['get', 'color'],
            'fill-outline-color': 'black',
            'fill-opacity':['case', ['boolean', ['feature-state', 'hover'], false], 0.9,0.5],
        },
        'circle':{
            "circle-radius": radius,
            "circle-color":['case', ['boolean', ['feature-state', 'hover'], false],hoverColor||color||'#333ea9',color||'#333ea9'],
        },
        'fill-extrusion':{
            'fill-extrusion-color': [
                "interpolate",
                ["exponential",1],
                ["get", "height"],
                //分类颜色设置（必须按顺序）
                100000000,
                '#5ea5e0',
                260000000,
                '#1b2c81',
                261000000,
                '#8a44b0'
            ],
            'fill-extrusion-height': [
                "interpolate", ["linear"], ["zoom"],4, 0,14.05, ["get", "height"]
            ],
            'fill-extrusion-base': [
                "interpolate", ["linear"], ["zoom"],4, 0,14.05, ["get", "base_height"]
            ],
            'fill-extrusion-opacity': 1         //透明度
        }
    }

    let layer;
    let source;

    let hoverId = '';

    const setGeoJson = () => {
        //更新geojson
        if(layer && source){
            clearLayer();
            addLayer();
        }
    }

    const clearLayer = () => {
        if(map && layer){
            try{
                if(map.getStyle().layers.filter(({id})=>id===layer.id).length){
                    //移除事件
                    map.off('mousemove',layer.id,layerMousemove);
                    map.off('mouseleave',layer.id,layerMouseleave);
                    map.off('click',layer.id,layerClick);

                    map.removeLayer(layer.id);
                    map.removeSource(layer.source);
                }
            }catch (e){
                console.warn(e);
            }
        }
    }

    const layerMousemove = (e)=>{
        if(e.features && e.features[0] && e.features[0].id){
            hoverId = e.features[0].id;
            map.setFeatureState({ source: name, id: hoverId }, { hover: true });
        }else{
            hoverId = '';
        }
    }

    const layerMouseleave = () => {
        if(hoverId){
            map.setFeatureState({ source: name, id: hoverId }, { hover: false })
            hoverId = '';
        }
    }

    const layerClick = (e) => {
        dispatch('layerClick',{...e,name});
    }

    $:geoJson && setGeoJson();

    $:color && changeColor();

    function changeColor() {
        if(map && layer){
            if('line' === type){
                map.setPaintProperty(layer.id,'line-color',color);
            }
        }
    }

    const addLayer = () => {
        if(map && geoJson.features && geoJson.features.length) {
            let timestamp = new Date().getTime();
            let sourceId = `s_${name}_${timestamp}`;
            let layerId = `l_${name}_${timestamp}`
            map.addSource(sourceId, {
                type: 'geojson',
                data: geoJson
            });
            map.addLayer({
                id: layerId,
                type,
                source: sourceId,
                layout:{},
                paint:paint||paints[type]
            });
            const {layers,sources} = map.getStyle();
            if(layers && sources){
                layer = layers.filter(({id})=>id == layerId).pop();
                source = sources[sourceId];
            }

            map.on('mousemove',layerId,layerMousemove);
            map.on('mouseleave',layerId,layerMouseleave)
            if(geoJson.properties && geoJson.properties.clickable){
                map.on('click',layerId,layerClick)
            }
        }
    }

    onMount(() => {
        if(geoJson.features && geoJson.features.length){
            addLayer();
            return ()=>{
                if(layer && source){
                    clearLayer();
                }
            }
        }
    });

</script>
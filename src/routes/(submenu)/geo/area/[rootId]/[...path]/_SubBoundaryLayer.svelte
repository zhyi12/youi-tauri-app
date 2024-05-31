<script>
    import {getContext, onMount} from 'svelte'
    import {contextKey} from "$lib/component/mapbox/mapbox";
    import * as THREE from "three";
    import Pie from "../../../../../../lib/component/mapbox/chart/pie";
    import {createModelTransform,createProjectionMatrix} from "../../../../../../lib/component/mapbox/chart/projection";
    import {processChartData} from "../../../../../../lib/tauri/geo";

    const { getMap, getMapbox } = getContext(contextKey)
    const map = getMap()
    const mapbox = getMapbox()

    export let boundary = undefined;

    export let areaId = '';

    let sourceId = 'sub_boundary';
    let layerId = `sub_layer_${sourceId}`;
    let textLayerId = `sub_text_layer_${sourceId}`;

    let barSourceId = `bar_${sourceId}`;
    let barLayerId = `bar_${layerId}`;

    let layer;
    let textLayer;
    let source;

    /**
     * 根据点数据生成矩形面数据
     * @param {点数据对象} properties object
     * @param {偏移量} offset number
     */
    function createSquaredByPoint(smx,smy,offset){
        const offset_int = parseFloat(offset);
        return [[smx,smy],[smx,smy + offset_int],[smx - offset_int,smy + offset_int],[smx - offset_int,smy],[smx,smy]];
    }

    const colors = ['#DE5347','#3DCE3D','#0080FF','#A473EA'];

    const createPie = (chartData) => {
        const pie = new Pie({
            items: chartData.values.map((value,index)=>{
                return {
                    value,
                    color:colors[index]
                }
            }),
            // text: (item)=> 'value-' + item.value
        });
        pie.rotation.x = Math.PI / 2;
        return pie;
    }

    const drawBoundary = () => {
        clearBoundary();

        source = map.addSource(sourceId,{
            type: 'geojson',
            data:boundary
        });
        // 子行政区划图层
        layer = map.addLayer({
            id:layerId,
            type: 'fill',
            source: sourceId,
            'paint': {
                'fill-color': '#c7d1ee',
                'fill-opacity': [
                    'case',
                    ['boolean', ['feature-state', 'hover'], false],
                    1,
                    0.5
                ],
                'fill-outline-color': 'black',
            }
        });

        // 添加symbol图层以显示子行政区划文本
        textLayer = map.addLayer({
            id: textLayerId,
            type: "symbol",
            source: sourceId,
            "layout": {
                "text-field": "{name}",
                "text-font": ["Open Sans Semibold"],
                "symbol-spacing": 600,
                "text-size": 10,
                "symbol-placement":'point'
            },
            paint: {
                "text-color": "#000", // 文本颜色
                "text-halo-color": "#FFF", // 文本描边颜色
                "text-halo-width": 1, // 文本描边宽度
            },
        });
        let points = boundary.features.map(f=>{
            if(f.properties.center){
                const {lng,lat} = JSON.parse(f.properties.center);
                return {
                    x:lng,
                    y:lat,
                    translate_x:0,
                    translate_y:0,
                    values:[]
                }
            }
        }).filter(f=>!!f);
        // let hoveredPolygonId;
        // map.on('mousemove', layerId, (e) => {
        //     if (e.features.length > 0) {
        //         if (hoveredPolygonId !== null) {
        //             map.setFeatureState(
        //                 { source: sourceId, id: hoveredPolygonId },
        //                 { hover: false }
        //             );
        //         }
        //         hoveredPolygonId = e.features[0].properties.areaId;
        //         map.setFeatureState(
        //             { source: sourceId, id: hoveredPolygonId },
        //             { hover: true }
        //         );
        //     }
        // });

        // When the mouse leaves the state-fill layer, update the feature state of the
        // previously hovered feature.
        // map.on('mouseleave', layerId, () => {
        //     if (hoveredPolygonId !== null) {
        //         map.setFeatureState(
        //             { source: sourceId, id: hoveredPolygonId },
        //             { hover: false }
        //         );
        //     }
        //     hoveredPolygonId = null;
        // });


        processChartData(points).then(pieData=>{
            let chartData = pieData.map(data=>{
                data.values = [Math.random()*10,Math.random()*10,Math.random()*10];
                return data;
            });
            console.log(chartData)
            addChartLayer(chartData)
        });
    }

    /**
     *
     */
    const addChartLayer = (chartData)=>{
        // transformation parameters to position, rotate and scale the 3D model onto the map
        const modelTransform = createModelTransform(mapboxgl,chartData[0].x,chartData[0].y);

        map.addLayer({
            id: barLayerId,
            type: 'custom',
            renderingMode: '3d',
            onAdd: function (map, gl) {
                this.camera = new THREE.PerspectiveCamera(45, 1, 0.1, 1000);

                this.scene = new THREE.Scene();

                this.map = map;

                this.renderer = new THREE.WebGLRenderer({
                    canvas: map.getCanvas(),
                    context: gl,
                    antialias: true
                });

                this.scene.add(new THREE.AmbientLight(0x404040)); // soft white light
                // 上光源
                const directionalLight = new THREE.DirectionalLight(0xffffff, 0.8);
                directionalLight.position.set(0, 100, 20)
                this.scene.add(directionalLight);
                const directionalLight3 = directionalLight.clone()
                directionalLight3.position.set(0, 100, -20)
                this.scene.add(directionalLight3);
                // 下光源
                const directionalLight2 = directionalLight.clone()
                directionalLight2.position.set(0, -100, 0)
                this.scene.add(directionalLight2);

                for(let i=0;i<chartData.length;i++){
                    const pie = createPie(chartData[i]);
                    const group = new THREE.Group();
                    pie.translateX(chartData[i].translate_x*-1);
                    pie.translateY(chartData[i].translate_y+2500);
                    group.add(pie);
                    this.scene.add( group );
                }

                this.renderer.autoClear = false;
            },
            render: function (gl, matrix) {
                this.camera.projectionMatrix = createProjectionMatrix(matrix,modelTransform);
                this.renderer.resetState();
                this.renderer.render(this.scene, this.camera);
                this.map.triggerRepaint();
            }
        })
    }

    const clearBoundary = () => {

        if(layer && source){
            map.removeLayer(textLayerId);
            map.removeLayer(layerId);
            map.removeLayer(barLayerId);
            map.removeSource(sourceId);
            // map.removeSource(barSourceId);
            layer = undefined;
            source = undefined;
        }

    }

    $: boundary && drawBoundary();

    $: if(boundary == null){
        clearBoundary()
    }

    onMount(()=>{
        map.on('click', layerId, (e) => {
            new mapboxgl.Popup()
                .setLngLat(e.lngLat)
                .setHTML(JSON.stringify(e.features[0].properties.areaId))
                .addTo(map);
        });
    })

</script>
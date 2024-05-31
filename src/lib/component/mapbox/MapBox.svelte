<script>
    import { setContext, onDestroy, createEventDispatcher } from 'svelte'
    import { contextKey } from './mapbox.js'
    import action from './map-action.js'
    import { EventQueue } from './queue.js'
    // import {classnames} from "../../youi/util/classname";
    import {load} from "../util/dom";
    let className = '';
    /*  样式 */
    export { className as class };

    export let editingGeo = undefined;

    export let map = undefined;
    export let version = 'v2.14.1'
    export let center = [ 0, 0 ]
    export let zoom = 9
    export let pitch = 0
    export let zoomRate = 1
    export let wheelZoomRate = 1
    export let options = {}
    export let accessToken
    export let customStylesheetUrl = false
    export let style = 'mapbox://styles/mapbox/streets-v11'

    const dispatch = createEventDispatcher()
    $:classes = 'youi-mapbox';
    // $:classes = classnames("youi-mapbox",className);

    setContext(contextKey, {
        getMap: () => map,
        getMapbox: () => mapbox
    })

    let container
    let mapbox
    let draw
    let unbindDraw;

    const optionsWithDefaults = Object.assign({
        accessToken,
        container,
        style,
        center,
        zoom,
        zoomRate,
        wheelZoomRate,
        pitch,
        version,
        customStylesheetUrl,
        crs: 'EPSG:900913',
        map
    }, options)

    const queue = new EventQueue()

    const handle_draw_update = (e) => {
        dispatch('draw',e);
    }

    function init ({ detail }) {
        map = detail.map
        mapbox = detail.mapbox
        queue.start(map)
        dispatch('ready')

    }

    function initDraw() {
        let resources = [
            { type: 'script', attr: 'src', value: `https://api.mapbox.com/mapbox-gl-js/plugins/mapbox-gl-draw/v1.4.2/mapbox-gl-draw.js`, id: 'byk-gl-js-draw' },
            { type: 'link', attr: 'href', value: `https://api.mapbox.com/mapbox-gl-js/plugins/mapbox-gl-draw/v1.0.9/mapbox-gl-draw.css`, id: 'byk-gl-draw-css' }
        ];

        load(resources,()=>{
            if(map && !draw){
                draw = new window.MapboxDraw({
                    displayControlsDefault: false,
                    controls: {
                        polygon: true,
                        trash: true
                    },
                    defaultMode: editingGeo.features?'simple_select':'draw_polygon'
                });
                map.addControl(draw);
                map.on('draw.create',handle_draw_update);
                map.on('draw.update',handle_draw_update);
                map.on('draw.delete',handle_draw_update);
            }

            if(draw && editingGeo.features){
                //
                editingGeo.features.forEach(feature=>{
                    draw.add(feature);
                })
            }
        });
    }

    function destroyDraw() {
        if(map && draw){
            map.removeControl(draw);
            draw = undefined;
        }
    }

    onDestroy(() => {
        queue.stop()
        map = undefined
        draw = undefined
    })

    export function fitBounds (bbox, data = {}) {
        queue.send('fitBounds', [ bbox, data ])
    }

    export function flyTo (destination, data = {}) {
        queue.send('flyTo', [ destination, data ])
    }

    export function resize () {
        queue.send('resize')
    }

    export function setCenter (coords, data = {}) {
        queue.send('setCenter', [ coords, data ])
    }

    export function setZoom (value, data = {}) {
        queue.send('setZoom', [ value, data ])
    }

    export function addControl (control, position = 'top-right') {
        queue.send('addControl', [ control, position ])
    }

    export function getMap () {
        return map
    }

    export function getMapbox () {
        return mapbox
    }


    $: zoom && setZoom(zoom)
    $: center && setCenter(center)
    $: editingGeo && (editingGeo.features ? initDraw():destroyDraw())
    $: resize();
</script>

<div class={classes}
        use:action={optionsWithDefaults}
        on:ready={init}
        on:recentre
        on:dragend
        on:click
        on:zoomstart
        on:zoom
        on:zoomend
        on:drag
>
    {#if map}
        <slot></slot>
    {/if}
</div>

<style>
    div {
        width: 100%;
        height: 100%;
    }
</style>
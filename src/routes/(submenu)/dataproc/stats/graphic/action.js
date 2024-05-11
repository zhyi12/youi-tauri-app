import {Scene, RasterLayer,PointLayer} from '@antv/l7';
import {Map} from '@antv/l7-maps';
import { Choropleth } from '@antv/l7plot';

export default function action(node, options = {}) {

    const scene = new Scene({
        id: node.getAttribute('id'),
        map: new Map({
            pitch: 66.02383,
            style: 'dark',
            center: [121.400257, 31.25287],
            zoom: 14.55,
            rotation: 134.95,
            // center: [120.19382669582967, 30.258134],
            // zoom: 3,
        }),
    });


    scene.on('loaded', () => {

        fetch('https://gw.alipayobjects.com/os/basement_prod/893d1d5f-11d9-45f3-8322-ee9140d288ae.json')
            .then((res) => res.json())
            .then((data) => {
                const pointLayer = new PointLayer({})
                    .source(data, {
                        parser: {
                            type: 'json',
                            x: 'longitude',
                            y: 'latitude',
                        },
                    })
                    //.animate(true)
                    .active(true)
                    .shape('name', ['cylinder', 'triangleColumn', 'hexagonColumn', 'squareColumn'])
                    .size('unit_price', (h) => {
                        return [6, 6, h / 500];
                    })
                    .color('name', ['#739DFF', '#61FCBF', '#FFDE74', '#FF896F']);

                scene.addLayer(pointLayer);

            });


    });

}
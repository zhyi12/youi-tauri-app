export const cnStyle = () => {

    return {
        version: 8,
        sources: {
            "raster-tiles":{
                type: "raster",
                tiles: ["http://192.168.173.1:8080/geoserver/gwc/service/tms/1.0.0/ne%3Acn@EPSG%3A900913@pbf/{z}/{x}/{y}.pbf"],
                tileSize: 256
            }
        },
        layers: [{
            "id": "tdt-vec-tiles",
            "type": "raster",
            "source": "raster-tiles",
            minzoom: 0,
            maxzoom: 22
        }]
    }
}
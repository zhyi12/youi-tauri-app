import * as THREE from "three";

const modelAltitude = 0;
const modelRotate = [Math.PI / 2, 0, 0];

/**
 * three 坐标转换
 * @param mapboxgl
 * @param lng
 * @param lat
 */
export const createModelTransform = (mapboxgl,lng,lat) => {
    const modelAsMercatorCoordinate = mapboxgl.MercatorCoordinate.fromLngLat([lng,lat], modelAltitude);
    return {
        translateX: modelAsMercatorCoordinate.x,
        translateY: modelAsMercatorCoordinate.y,
        translateZ: modelAsMercatorCoordinate.z,
        rotateX: modelRotate[0],
        rotateY: modelRotate[1],
        rotateZ: modelRotate[2],
        /* Since the 3D model is in real world meters, a scale transform needs to be
         * applied since the CustomLayerInterface expects units in MercatorCoordinates.
         */
        scale: modelAsMercatorCoordinate.meterInMercatorCoordinateUnits()
    };
}

/**
 *
 * @param matrix
 * @param modelTransform
 */
export const createProjectionMatrix = (matrix,modelTransform) => {
    const rotationX = new THREE.Matrix4().makeRotationAxis(new THREE.Vector3(1, 0, 0), modelTransform.rotateX);
    const rotationY = new THREE.Matrix4().makeRotationAxis(new THREE.Vector3(0, 1, 0), modelTransform.rotateY);
    const rotationZ = new THREE.Matrix4().makeRotationAxis(new THREE.Vector3(0, 0, 1), modelTransform.rotateZ);
    const m = new THREE.Matrix4().fromArray(matrix);
    const l = new THREE.Matrix4()
        .makeTranslation(modelTransform.translateX, modelTransform.translateY, modelTransform.translateZ)
        .scale(new THREE.Vector3(modelTransform.scale, -modelTransform.scale, modelTransform.scale))
        .multiply(rotationX)
        .multiply(rotationY)
        .multiply(rotationZ);
    return m.multiply(l);
}


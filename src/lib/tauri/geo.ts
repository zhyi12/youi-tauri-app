import { invoke } from "@tauri-apps/api/core";
import { Store } from '@tauri-apps/plugin-store';

const subGeoStore = new Store("store.subGeo");
/**
 *
 * @param script
 */
export async function mvt(geojson: string): Promise<[]> {
    const result = await invoke('plugin:geo|mvt', { geojson});
}

export async function to_geojson(geom: string): Promise<String> {
    const result = await invoke('plugin:geo|to_geojson', { geom});
    return result;
}

/**
 *
 * @param chartData
 */
export async function processChartData(chartData:[]) {
    const result = await invoke('plugin:geo|process_chart_data', { chartData});
    return result;
}
/**
 *
 * @param id
 */
export async function findSubGeoJson(id: string): Promise<any> {
    let subGeoJson;
    if (await subGeoStore.has(id)){
        subGeoJson = await subGeoStore.get<String>(id);
    }else{
        subGeoJson = await invoke<String>('plugin:geo|find_sub_geojson', {id});
        await subGeoStore.set(id,subGeoJson);
        await subGeoStore.save();
    }
    try {
        return JSON.parse(subGeoJson);
    }catch (e) {
        return null;
    }
}
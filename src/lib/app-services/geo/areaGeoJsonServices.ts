import {PROPERTIES,SQL_FIND, SQL_UPDATE,SQL_INSERT} from "../../app-dao/geo/areaGeo.sql";
import {appDataBase} from "../../app-dao/dao.util";

import type {AreaGeoJson} from "../../app-entity/geo/areaGeoJson";
import {fetch} from "@tauri-apps/plugin-http";
import {to_geojson} from "../../tauri/geo";
import {AREA_LEVEL_COUNTY, findAreaLevel} from "../base/areaServices";


/**
 *
 * @param area
 */
export async function findGeoJson(area_id:string,options?):Promise<any>{
    const result = await appDataBase.select<AreaGeoJson[]>(SQL_FIND,[area_id]);
    let geo_json_str = '';
    if(result.length){
        geo_json_str = result[0].geo_json;
    }
    if(!geo_json_str && findAreaLevel(area_id)<=AREA_LEVEL_COUNTY){
        const time = new Date().getTime();
        const searched = await searchAdministrative(options);
        if(searched){
            let areaGeoJson:AreaGeoJson;
            if(result.length){
                areaGeoJson = Object.assign(result[0],searched,{update_time:time});
                //更新
                await updateAreaGeoJson(areaGeoJson);
            }else{
                areaGeoJson = <AreaGeoJson>{area_id,create_time:time,update_time:time,...searched};
                //插入
                await insertAreaGeoJson(areaGeoJson);
            }
            geo_json_str = searched.geo_json;
        }
    }

    try {
        return JSON.parse(geo_json_str)
    }catch (e){
        //
        console.warn(e,geo_json_str)
    }
}

export async function updateBySearch(area_id:string,options){
    const time = new Date().getTime();
    const result = await appDataBase.select<AreaGeoJson[]>(SQL_FIND,[area_id]);

}


/**
 * 从天地图同步获取行政区划边界
 * @param options
 */
export async function searchAdministrative(options) {
    const url = `http://api.tianditu.gov.cn/v2/administrative?keyword=${options.district}&childLevel=2&extensions=true&tk=${options.tdtKey}`
    const response = await fetch(url);
    const json = await response.json();

    if(json.status === 200 && json.data.district){
        // 异常处理
        const geo_json = await to_geojson(json.data.district.boundary);
        const center = JSON.stringify(json.data.district.center);

        //一键同步下级
        if(json.data.district.children){
            syncSubAreas(json.data.district.children,options).then(r=>{
                console.log('同步下级行政区划边界',r);
            });
        }
        return {
            geo_json,
            center
        }
    }
}

/**
 *
 * @param districts
 * @param options
 */
async function syncSubAreas(districts,options){
   return await Promise.all(districts.map(sub=>findGeoJson(sub.gb.substring(3)+'000000',{...options,district:sub.gb})))
}

/**
 * 插入行政区划geojson
 * @param AreaGeoJson
 */
export async function insertAreaGeoJson(areaGeoJson:AreaGeoJson):Promise<number|string>{
    // eslint-disable-next-line @typescript-eslint/ban-ts-comment
    // @ts-ignore
    const bindValues= PROPERTIES.map(prop=>areaGeoJson[prop]||'');
    //
    const result = await appDataBase.execute(SQL_INSERT,bindValues);
    return result.lastInsertId;
}

/**
 * 更新插入行政区划geojson
 * @param AreaGeoJson
 */
export async function updateAreaGeoJson(areaGeoJson:AreaGeoJson):Promise<number>{
    // eslint-disable-next-line @typescript-eslint/ban-ts-comment
    // @ts-ignore
    const bindValues= PROPERTIES.map(prop=>areaGeoJson[prop]||'');
    //
    const result = await appDataBase.execute(SQL_UPDATE,bindValues);
    return result.rowsAffected;
}

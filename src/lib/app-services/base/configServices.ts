import type {Config} from "../../app-entity/base/config";
import {BaseDirectory, exists, readTextFile, writeTextFile} from "@tauri-apps/plugin-fs";
import {appConfigDir} from "@tauri-apps/api/path";

const DEFAULT_CONFIG:Config = {
    dataDir:'',
    areaId:'',
    ownerAreaId:'',
};

/**
 * 获取系统配置信息
 */
export async function findConfig():Promise<Config>{
    const configExists = await exists("app.config",{baseDir:BaseDirectory.AppData});

    const configDir = await appConfigDir();

    if(!configExists){
        await writeTextFile("app.config",JSON.stringify(DEFAULT_CONFIG),{baseDir:BaseDirectory.AppData});
    }else{
        const result = await readTextFile("app.config",{baseDir:BaseDirectory.AppData});
        try {
            return {...JSON.parse(result),dbConnect:`sqlite://${configDir}/app.db`};
        }catch (e) {
            console.log(e)
        }
    }
    return {...DEFAULT_CONFIG,dbConnect:`${configDir}/app.db`};
}

/**
 *
 * @param config
 */
export async function saveConfig(config:Config):Promise<void>{
    await writeTextFile("app.config",JSON.stringify(config),{baseDir:BaseDirectory.AppData});
}
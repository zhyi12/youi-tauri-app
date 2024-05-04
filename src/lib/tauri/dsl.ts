import { invoke } from "@tauri-apps/api/core";

/**
 *
 * @param script
 */
export async function execute(script: string,params:[]): Promise<string> {
    return await invoke<string>('plugin:dsl|execute', { script ,params:params||[]}).then((result) => {
        try{
            return JSON.parse(result);
        }catch (e) {
            console.warn(e);
            return [];
        }
    });
}
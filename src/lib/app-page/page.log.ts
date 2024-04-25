import { info, attachConsole } from '@tauri-apps/plugin-log';

let detach = undefined;

export const APP_LOG = {

    info:async (message)=>{
        if(!detach){
            detach = await attachConsole();
        }
        await info(message);
    }
}
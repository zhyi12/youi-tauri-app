import { message,confirm } from '@tauri-apps/plugin-dialog';

export const APP_MESSAGE = {

    /**
     *
     */
    info:async (title,content)=>{
        await message(content, { title, type: 'info' });
    },

    confirm:async (message,title?:string):Promise<boolean>=>{
        return await confirm(message,{king:'info',title:title||'确认'})
    }
}
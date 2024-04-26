import { message,confirm } from '@tauri-apps/plugin-dialog';

export const APP_MESSAGE = {

    /**
     *
     */
    info:async (title,content)=>{
        await message(content, { title, type: 'info' });
    },

    confirm:async (message):Promise<boolean>=>{
        return await confirm(message)
    }
}
import { message } from '@tauri-apps/plugin-dialog';

export const APP_MESSAGE = {

    /**
     *
     */
    info:async (title,content)=>{
        await message(content, { title, type: 'info' });
    }

}
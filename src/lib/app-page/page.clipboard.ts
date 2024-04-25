// @ts-ignore
import { writeText, readText,readImage } from '@tauri-apps/plugin-clipboard-manager';
export const APP_CLIPBOARD = {

    /**
     *
     * @param text
     */
    writeText:async (text:string)=>{
        await writeText(text);
    },

    // writeImage:async ()=>{
    //
    // }
    /**
     *
     */
    readText:async ()=>{
        return await readText();
    },

    /**
     * 读取图片
     */
    readImage:async (callback: (arg0: string | ArrayBuffer | null | undefined) => void)=>{
        const clipboardImage = await readImage();
        clipboardImage.rgba().then((b: BlobPart)=>{
            const blob = new Blob([b], { type: 'image' });
            const reader = new FileReader();
            reader.onload = (e)=>{
                callback(e.target?.result)
            }
            reader.readAsDataURL(blob);
        });
    }
}
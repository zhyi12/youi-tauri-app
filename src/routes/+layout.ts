import type { LayoutLoad } from './$types';
import {findAppMenus} from "../lib/app-services/base/menuServices";
import {parseActiveModule} from "../lib/app-page/page.menu";
import {findConfig} from "../lib/app-services/base/configServices";

// since there's no dynamic data here, we can prerender
// it so that it gets served as a static asset in production
export const prerender = true
export const ssr = false

let cachedAppConfig = undefined;

export const load: LayoutLoad = async ({url,params}) => {
    const menus = await findAppMenus();
    // 当前路径所属的模块
    const activeModule = parseActiveModule(url,params);
    // 配置信息
    if(!cachedAppConfig){
        const appConfig = await findConfig();
        cachedAppConfig = appConfig;
    }
    return {
        pathname:url.pathname,
        appConfig:{...cachedAppConfig},
        menus,
        activeModule
    }
}

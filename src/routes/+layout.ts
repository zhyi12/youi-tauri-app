import type { LayoutLoad } from './$types';
import {findAppMenus} from "../lib/app-services/base/menuServices";
import {parseActiveModule} from "../lib/app-page/page.menu";

// since there's no dynamic data here, we can prerender
// it so that it gets served as a static asset in production
export const prerender = true
export const ssr = false

export const load: LayoutLoad = async ({url,params}) => {
    // 获取系统菜单
    const menus = await findAppMenus();
    // 当前路径所属的模块
    const activeModule = parseActiveModule(url,params);

    return {
        pathname:url.pathname,
        menus,
        activeModule
    }
}

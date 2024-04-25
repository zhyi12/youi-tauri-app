/**
 * 从访问路径解析当前模块信息
 * @param url
 * @param params
 */
import type {MenuInfo,MenuPath} from "../app-entity/base/menu";
import type {Page} from "@sveltejs/kit";

// eslint-disable-next-line @typescript-eslint/ban-ts-comment
// @ts-ignore
export const parseActiveModule = (url,params):string => {
    const moduleMatcher:RegExpMatchArray|null = (url.pathname+'/').match(/\/(\w+[-]?\w+)\//);
    let module = '';
    if(moduleMatcher){
        module = moduleMatcher[1];
    }
    return params.module||module;
}

/**
 * 从约定的页面数据中解析导航菜单
 */
export function parseMenuPaths(page):MenuPath[]{
    return findMenuPaths(page.url.pathname,page.data.menus);
}

/**
 *
 * @param pathname
 * @param menus
 */
export function findMenuPaths(pathname,menus):MenuPath[] {
    let paths:MenuPath[] = [];
    paths = processMenuPaths(pathname,menus,paths);
    return paths;
}

/**
 *
 * @param uri
 * @param menus
 * @param parentPaths
 */
function processMenuPaths(uri:string,menus:MenuInfo[],parentPaths:MenuPath[]):MenuPath[] {
    for(let i=0;i<menus.length;i++){
        const menu = menus[i];
        let paths:MenuPath[] = [];
        paths = paths.concat(parentPaths).concat([{text:menu.text,href:menu.href,icon:menu.icon||menu.name}]);
        if(menu.href == uri){
            return paths;
        }else if(menu.children && menu.children.length){
            //
            const subMenuPaths = processMenuPaths(uri,menu.children,paths);
            if(subMenuPaths.length){
                return subMenuPaths;
            }
        }
    }
    return [];
}
import type { LayoutLoad } from './$types';
import {findMenuPaths} from "$lib/app-page/page.menu";

export const load: LayoutLoad = async ({parent,params}) => {
    const {menus} = await parent();
    //修改为实际路径
    const baseUri = '/geo/area';
    const baseMenuPaths = findMenuPaths(baseUri,menus);

    let activeId = '';
    let expandedNodeIds:string[] = [];

    if(params.path){
        expandedNodeIds = params.path.split('/');
        activeId = expandedNodeIds.pop();
    }

    return {
        baseUri,
        baseMenuPaths,
        activeId,
        expandedNodeIds
    }
}
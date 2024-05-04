import type { LayoutLoad } from './$types';
import {findMenuPaths} from "$lib/app-page/page.menu";

export const prerender = false;

export const load: LayoutLoad = async ({parent,params}) => {
    const {menus } = await parent();
    //修改为实际路径
    const baseUri = '/terminal';
    const baseMenuPaths = findMenuPaths(baseUri,menus);

    let activeNodeId = '';
    let expandedNodeIds:string[] = [];

    if(params.path){
        expandedNodeIds = params.path.split('/');
        activeNodeId = expandedNodeIds.pop();
    }

    return {
        baseUri,
        baseMenuPaths,
        activeNodeId,
        expandedNodeIds
    }
}
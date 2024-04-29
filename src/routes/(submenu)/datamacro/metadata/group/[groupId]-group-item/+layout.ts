import type { LayoutLoad } from './$types';
import {findMenuPaths} from "$lib/app-page/page.menu";
import {findMacroGroup} from "$lib/app-services/datamacro/macroGroupServices";

// 缓存根据groupId获取的分组信息
const cached = {};

export const load: LayoutLoad = async ({parent,params}) => {
    const {menus} = await parent();
    //修改为实际路径
    const baseUri = '/datamacro/metadata/group';
    const baseMenuPaths = findMenuPaths(baseUri,menus);

    let activeNodeId = '';
    let expandedNodeIds:string[] = [];

    if(params.path){
        expandedNodeIds = params.path.split('/');
        activeNodeId = expandedNodeIds.pop();
    }
    let macroGroup;
    if(!cached[params.groupId]){
        macroGroup = await findMacroGroup(params.groupId);
        cached[params.groupId] = macroGroup;
    }else{
        macroGroup = cached[params.groupId];
    }

    if(macroGroup){
        baseMenuPaths.push({text:macroGroup.text});
    }

    return {
        baseUri:`${baseUri}/${params.groupId}-group-item`,
        baseMenuPaths,
        pathMenuPaths:baseMenuPaths,
        activeNodeId,
        expandedNodeIds
    }
}
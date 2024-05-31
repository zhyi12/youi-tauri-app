import type { LayoutLoad } from './$types';
import {findMenuPaths} from "$lib/app-page/page.menu";
import {findDashboard} from "$lib/app-services/dmp/dashboardServices";

export const load: LayoutLoad = async ({parent,params}) => {
    const {menus } = await parent();
    //修改为实际路径
    const dashboardUri = '/datanalysis/dashboard';
    const pathMenuPaths = findMenuPaths(dashboardUri,menus);

    const record = await findDashboard(params.id);

    if(record){
        pathMenuPaths.push({text:record.text});
    }

    return {
        baseUri:`${dashboardUri}-designer-${params.id}`,
        pathMenuPaths,
        record
    }
}
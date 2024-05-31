import type { PageLoad } from './$types';
import {findDashboardGroup} from "$lib/app-services/dmp/dashboardGroupServices";

export const load: PageLoad = async ({params,parent}) => {

    const {baseMenuPaths} = await parent();
    const paths = params.path.split('/');
    const id = paths.pop();

    const record = await findDashboardGroup(id);

    return {
        record,
        pathMenuPaths:[...baseMenuPaths,{text:record?.text}]
    }
}
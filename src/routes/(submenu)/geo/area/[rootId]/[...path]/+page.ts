import type { PageLoad } from './$types';
import {findArea} from "$lib/app-services/base/areaServices";

export const prerender = false

export const load: PageLoad = async ({params,parent}) => {

    const {baseMenuPaths} = await parent();
    const paths = params.path.split('/');
    const id = paths.pop();

    const record = await findArea(id);

    return {
        nodeId:id,
        district:`156${id.substring(0,6)}`,
        record,
        pathMenuPaths:[...baseMenuPaths,{text:record?.text}]
    }
}
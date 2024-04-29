import type { PageLoad } from './$types';
import {findMacroGroupItem} from "$lib/app-services/datamacro/macroGroupItemServices";

export const load: PageLoad = async ({params,parent}) => {

    const {baseMenuPaths} = await parent();
    const paths = params.path.split('/');
    const id = paths.pop();

    const record = await findMacroGroupItem(id);

    return {
        record,
        pathMenuPaths:[...baseMenuPaths,{text:record?.text}]
    }
}
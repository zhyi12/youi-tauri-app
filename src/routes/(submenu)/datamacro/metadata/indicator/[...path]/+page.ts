import type { PageLoad } from './$types';
import {findMacroIndicator} from "$lib/app-services/datamacro/macroIndicatorServices";

export const load: PageLoad = async ({params,parent}) => {

    const {baseMenuPaths} = await parent();
    const paths = params.path.split('/');
    const indicatorId = paths.pop();

    const indicator = await findMacroIndicator(indicatorId);

    return {
        record:indicator,
        pathMenuPaths:[...baseMenuPaths,{text:indicator?.text}]
    }
}
import type { PageLoad } from './$types';

export const load: PageLoad = async ({parent}) => {
    const {pathMenuPaths} = await parent();

    return {
        pathMenuPaths:[...pathMenuPaths,{text:'新增图标'}]
    }
}
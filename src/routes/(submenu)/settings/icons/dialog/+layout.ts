import type { LayoutLoad } from './$types';
import {findMenuPaths} from "$lib/app-page/page.menu";

export const load: LayoutLoad = async ({parent}) => {

    const {menus} = await parent();

    let pathMenuPaths = findMenuPaths('/settings/icons',menus);

    return {
        pathMenuPaths
    }
}

import type { LayoutLoad } from './$types';

export const load: LayoutLoad = async ({parent}) => {

    const {activeModule,menus} = await parent();

    let submenu = menus.find(menu=>menu.name === activeModule);

    return {
        submenu
    }
}


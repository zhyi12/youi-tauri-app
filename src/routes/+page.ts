import type { PageLoad } from './$types';
// import {findByPager} from "../lib/app-services/base/desktopItemServices";

export const load: PageLoad = async () => {

    // const pagers = await findByPager({pageIndex:1,pageSize:10});
    // console.log(pagers)

    return {
        pathMenuPaths:[{text:'工作台',icon:'home'}]
    }
}
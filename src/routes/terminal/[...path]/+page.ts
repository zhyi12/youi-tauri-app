import type { PageLoad } from './$types';
import {findDslScriptContent} from "$lib/app-services/base/dslScriptServices";
import {findConfig} from "../../../lib/app-services/base/configServices";

export const load: PageLoad = async ({params,parent}) => {

    const {baseMenuPaths} = await parent();
    const paths = params.path.split('/');
    const id = paths.pop();

    const record = await findDslScriptContent(id);

    const config = await findConfig();

    console.log(record)

    return {
        id,
        dataDir:config.dataDir,
        dslContent:record.content,
        pathMenuPaths:[...baseMenuPaths,{text:record?.text}]
    }
}
import type { PageLoad } from './$types';
import {findMetaNode} from "$lib/app-services/metadata/surveyPlanTreeServices";
import _ from 'lodash';
import {findMetaObject} from "$lib/app-services/metadata/surveyPlanTreeServices";

export const load: PageLoad = async ({params,parent}) => {

    const parentData = await parent();

    const {pathMenuPaths,activeId} = parentData;

    const node = await findMetaNode(activeId);

    const metaObject = await findMetaObject(node);

    return {
        pathMenuPaths:[...pathMenuPaths,'plan'!==node.object_name?{text:node.text,icon:`survey${_.upperFirst(node.object_name)}`}:{}],
        objectName:node.object_name,
        nodeId:node.id,
        metaObject
    }
}
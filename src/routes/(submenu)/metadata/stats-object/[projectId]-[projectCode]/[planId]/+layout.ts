import type { LayoutLoad } from './$types';
import {findSurveyPlan} from "$lib/app-services/metadata/surveyPlanServices";
import {findMenuPaths} from "$lib/app-page/page.menu";
import {findOrCreatePlanNode} from "$lib/app-services/metadata/surveyPlanTreeServices";

/**
 *
 * @param parent
 */
export const load: LayoutLoad = async ({params,parent}) => {
    const parentData = await parent();
    const plan = await findSurveyPlan(params.planId);
    const baseUri = `/metadata/stats-object/${params.projectId}-${params.projectCode}/${params.planId}`;
    let rootId = '';
    if(plan){
        const node = await findOrCreatePlanNode(plan.id,plan.text);
        rootId = node.id;
    }

    const pathMenuPaths = findMenuPaths('/metadata/stats',parentData.menus);

    let activeId = '';
    let expandedIds = [];

    if(params.path){
        expandedIds = params.path.split('/')
        activeId = expandedIds.pop();
    }

    //
    return {
        baseUri,
        pathMenuPaths:[...pathMenuPaths,{text:'制度设计',icon:'tree'},{text:plan.text,href:baseUri,icon:'surveyPlan'}],
        rootId,
        activeId,
        expandedIds
    }
}
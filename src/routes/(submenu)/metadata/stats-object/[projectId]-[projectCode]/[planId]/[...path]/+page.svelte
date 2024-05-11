<script>
    import {page} from "$app/stores";
    import {afterNavigate, beforeNavigate, goto} from "$app/navigation";
    import {Toolbar,Button,Icon,saveIcon} from "$lib/youi";
    import {isEqual} from "lodash";

    import {updateMetaObject} from "$lib/app-services/metadata/surveyPlanTreeServices";
    import {treeStore} from "$lib/app-stores/metadata/surveyPlanTreeStore";
    import SurveyPlanPage from "$lib/app-page/metadata/SurveyPlanPage.svelte";
    import SurveyTaskPage from "$lib/app-page/metadata/SurveyTaskPage.svelte";
    import SurveyTablePage from "$lib/app-page/metadata/SurveyTablePage.svelte";
    import {APP_MESSAGE} from "../../../../../../../lib/app-page/page.message";

    export let data;

    export let metaObject = undefined;

    let orgMetaObject = {};

    let isDirty = false;

    $:{
        isDirty = !isEqual(metaObject,orgMetaObject);
    }
    /**
     *
     */
    const doSave = async () => {
        // 更新元数据,同时更新树节点文本
        await updateMetaObject(data.nodeId,data.objectName,metaObject);
        if(orgMetaObject.text !== metaObject.text){
            //重命名，onlyShow设置为true，不重复更新数据库
            treeStore.rename(data.nodeId,metaObject.text,true);
        }
        orgMetaObject = {...metaObject};
    }

    beforeNavigate(({cancel,to})=>{
        if(isDirty){
            cancel();
            APP_MESSAGE.confirm('修改的数据未保存，确认离开?').then(passed=>{
                if(passed){
                    isDirty = false;
                    goto((to.url.href))
                }
            })
        }
    });

    afterNavigate(()=>{
        metaObject = {...data.metaObject};
        orgMetaObject = {...metaObject};
    })

</script>

<Toolbar>
    <Button title="保存" disabled={!isDirty} on:click={()=>doSave()}>
        <Icon data={saveIcon}/>
    </Button>
</Toolbar>
<div class="flex-1 p-1 space-y-2">
{#if metaObject}
    {#if 'plan' === data.objectName}
        <SurveyPlanPage bind:metaObject projectCode={$page.params.projectCode}/>
    {:else if 'task' === data.objectName}
        <SurveyTaskPage bind:metaObject/>
    {:else if 'table' === data.objectName}
        <SurveyTablePage bind:metaObject/>
    {/if}
{/if}
</div>
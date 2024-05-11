<script>

    import {surveyProjectStore} from "$lib/app-stores/metadata/surveyProjectStore";
    import {onMount} from "svelte";
    import {Tooltip,Button,Icon,plusIcon} from "$lib/youi";
    import {findLastNByProject} from "$lib/app-services/metadata/surveyPlanServices";
    import {configStore} from "$lib/app-stores/base/configStore";

    let projectLastPlans = new Map();

    $:if($configStore.dbConnect){
        findLastNByProject(3,$configStore.dbConnect).then(plans=>{
            const lastPlans = new Map();
            plans.reduce((a,plan)=>{
                if(!a.has(plan.project_id)){
                    a.set(plan.project_id,[]);
                }
                a.get(plan.project_id).push(plan);
                return a;
            },lastPlans);
            projectLastPlans = lastPlans;
        });
    }

    /**
     *
     */
    const createProject = () => {
        surveyProjectStore.addSurveyProject({
            text:'新项目'
        })
    }

    onMount(async ()=>{
        await surveyProjectStore.findByPager({pageSize:1000,pageIndex:1});
    })

</script>

<div class="grid p-4 grid-cols-6 gap-4">
    {#each $surveyProjectStore.records as record}
        <div class="card bg-base-100 shadow-xl">
            <figure>
                {record.text}
            </figure>
            <div class="card-body center">
                {#if projectLastPlans.has(record.id)}
                    {#each projectLastPlans.get(record.id) as plan}
                        <a href={`/metadata/stats-object/${record.id}-${record.code}/${plan.id}`}>{plan.text}</a>
                    {/each}
                {:else}
                    <Button class="btn-ghost btn-md" title="创建新制度">
                        <Icon data={plusIcon}/>新制度
                    </Button>
                {/if}
            </div>
        </div>
    {/each}
    <div class="card w-48 h-32 bg-base-100 shadow-xl hover:bg-blue-100 hover:cursor-pointer">
        <div class="card-body">
            <Tooltip title="创建新项目">
                <div class="text-center pt-4" on:click={()=>createProject()}>
                    <Icon scale="2" data={plusIcon}></Icon>
                </div>
            </Tooltip>
        </div>
    </div>
</div>
<script>

    import {onMount} from "svelte";
    import {goto} from "$app/navigation";

    import {treeStore} from "$lib/app-stores/metadata/surveyPlanTreeStore";
    import {Tree,TreeNodeEditor,renameIcon,removeIcon,findPathNodes} from "$lib/youi";
    import {createFolderIcon,surveyTaskIcon,surveyPlanIcon,surveyTableIcon} from "$lib/app-icons";
    import {APP_MESSAGE} from "$lib/app-page/page.message";

    export let data;

    const META_ICONS = {
        'meta_plan':surveyPlanIcon,
        'meta_task':surveyTaskIcon,
        'meta_table':surveyTableIcon,
    };

    const icons = (iconName) => {
        return META_ICONS[iconName];
    }

    const addSurveyTask = (id) => createMetaNode('task','新方法',id);

    const beforeInsertSurveyTask = (id) => insertBefore('task','新方法',id);

    const afterInsertSurveyTask = (id) => insertAfter('task','新方法',id);

    const addSurveyTable = (id) => createMetaNode('table','新调查表',id);

    const removeMetaNode = async (id) => {
        const passed = await APP_MESSAGE.confirm('确认删除?')
        if(passed){
            await treeStore.removeNode(id,(node,redirectPaths)=>{
                goto(`${data.baseUri}/${redirectPaths.join('/')}`);
            });
        }
    }

    /**
     * 创建新节点
     */
    const createMetaNode = async (object_name,text,pid) => {
        const newMetaNode = {text, object_name, pid};
        await treeStore.addChild(newMetaNode);
        if(!expandedIds.includes(pid)){
            expandedIds = [...expandedIds,pid];
        }
        // 定位到新节点
        redirectToNode(newMetaNode);
    };
    /**
     * 前插入节点
     */
    const insertBefore = async (object_name,text,refId) => {
        const newMetaNode = {text, object_name};
        await treeStore.insertBefore(refId,{object_name,text});
        // 定位到新节点
        redirectToNode(newMetaNode);
    };
    /**
     * 后插入节点
     */
    const insertAfter = async (object_name,text,refId) => {
        const newMetaNode = {text, object_name};
        await treeStore.insertAfter(refId,newMetaNode);
        // 定位到新节点
        redirectToNode(newMetaNode);
    };

    /**
     * 定向到节点
     */
    const redirectToNode = (metaNode) => {
        if(metaNode.id){
            let paths = [];
            if(metaNode.pid){
                paths = findPathNodes($treeStore.nodes,metaNode.pid).map(n=>n.id);
            }
            paths.push(metaNode.id);
            goto(`${data.baseUri}/${paths.join('/')}`);
        }
    }

    /**
     * 重命名
     * @param id
     * @param newName
     */
    const rename = (id,newName) => {
        treeStore.rename(id,newName).then(t=>t);
    }

    /**
     *
     * @param detail
     */
    const handle_node_select = ({detail}) => {
        const {id} = detail;
        const pathNodes = findPathNodes($treeStore.nodes,id);
        const path = pathNodes.map(p=>p.id).join('/');
        if(id){
            goto(`${data.baseUri}/${path}`);
        }
    }

    // 树节点操作按钮
    let buttons = [
        {name:'rename', text:'重命名', icon:renameIcon, action:rename},

        {name:'addSurveyTask',text:'新增调查方法',icon:surveyTaskIcon,groups:['plan'],action:addSurveyTask},
        {name:'addSurveyTable',text:'新增调查表',icon:surveyTableIcon,groups:['plan','task','folder'],action:addSurveyTable},

        {name:'beforeInsertSurveyTask',text:'上插入调查方法',icon:surveyTaskIcon,groups:['task'],action:beforeInsertSurveyTask},
        {name:'afterInsertSurveyTask',text:'下插入调查方法',icon:surveyTaskIcon,groups:['task'],action:afterInsertSurveyTask},

        {name:'addFolder',text:'新增文件夹',groups:['task','plan','folder'],icon:createFolderIcon},
        {name:'remove',text:'删除',icon:removeIcon,action:removeMetaNode},
    ];

    let expandedIds = [];

    onMount(()=>{
        if(data.rootId){
            treeStore.fetch({rootId:data.rootId})
            expandedIds = [...data.expandedIds];
        }
    });

</script>

<div class="flex flex-1">
    <Tree class="w-64 border-r" {icons}
          showSelect = {false}
          bind:expandedIds
          selectedIds = {[data.activeId]}
          on:select={handle_node_select}
          children={$treeStore.nodes}
          nodeRender={({id,text,group,hovered,selected})=>({component:TreeNodeEditor,props:{id,text,group,hovered,selected,buttons}})}/>
    <div class="flex-1 flex flex-col">
        <slot/>
    </div>
</div>

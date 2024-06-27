import {uuid} from "../../youi/util/uuid";

export const CONTEXT = 'DataQueryContext';

export const COLOR_JOIN_COLUMN_LEFT = 'rgb(240 253 244)';
export const COLOR_JOIN_COLUMN_RIGHT = 'rgb(254 252 232)';
export const COLOR_JOIN_COLUMN_ON = 'rgb(239 246 255)';

export const STEP_MENUS = [
    {name:'Select',text:'选字段'},
    {name:'Filter',text:'过滤'},
    {name:'Calculator',text:'计算列'},
    {name:'Sort',text:'排序'},
    {name:'Join',text:'左右连接'},
    {name:'Union',text:'上下连接'},
    {name:'Agg',text:'分组汇总'},
];

export const AGG_ITEMS = [
    {name:'sum',text:'求和'},
    {name:'count',text:'计数'},
    {name:'mean',text:'平均值'},
    {name:'median',text:'中位数'},
    {name:'max',text:'最大值'},
    {name:'min',text:'最小值'},
    {name:'first',text:'首个值'},
    {name:'std',text:'标准差'},
    {name:'variance',text:'方差'},
];

export const JOINS = [
    {name:'left',text:'左连接'},
    {name:'right',text:'右连接'},
    {name:'outer',text:'并集连接'},
    {name:'inner',text:'交集连接'},
]
/**
 *
 */
const COLUMNS_STEPS = ['Reader','Select'];

/**
 *
 */
const NO_COLUMNS_STEPS = ['Filter','Sort'];

/**
 *
 */
const MERGE_COLUMNS_STEPS = ['Calculator','Join'];

/**
 * 获取步骤输出列
 */
export const findStepOutputColumns = (steps,index) => {
    const step = steps[index];
    if(step){
        // 不影响输出的步骤，过滤、排序等
        if(NO_COLUMNS_STEPS.includes(step.name)){
            return findStepOutputColumns(steps,index-1);
        }else if(COLUMNS_STEPS.includes(step.name)){
            return step.columns.filter(c=>step.selectedColumnNames && step.selectedColumnNames.includes(c.name));
        }else if(MERGE_COLUMNS_STEPS.includes(step.name)){
            let outputColumns = findStepOutputColumns(steps,index-1);
            if('Calculator' === step.name){
                outputColumns = [...outputColumns,...step.calculators||[]]
            }else if('Join' === step.name){
                let leftColumns;
                let rightColumns;
                if(step.how === 'right'){
                    leftColumns = [...step.columns];
                    rightColumns = [...outputColumns];
                }else{
                    leftColumns = [...outputColumns];
                    rightColumns = [...step.columns];
                }
                const nameMap = leftColumns.reduce((m,v)=>m.set(v.name,v.name),new Map());
                const right_out_columns = rightColumns.map(c=>{
                    if(!nameMap.has(c.name)){
                        return {...c,fill:COLOR_JOIN_COLUMN_RIGHT};
                    }else if(step.how !== 'outer'){
                        const name = `${c.name}_right`;
                        return {
                            ...c,name,text:name,fill:COLOR_JOIN_COLUMN_RIGHT
                        }
                    }
                }).filter(c=>!!c);

                let joins = step.joinColumns
                    .map(c=>({
                        name:c.name,
                        text:c.name,
                        dataType:'text',
                        fill:COLOR_JOIN_COLUMN_ON
                    }));

                let join_names = step.joinColumns.map(c=>c.name);

                outputColumns = [...joins,...leftColumns
                    .filter(c=>!join_names.includes(c.name)).map(c=>({...c, fill: COLOR_JOIN_COLUMN_LEFT})),...right_out_columns];
            }
            return outputColumns;
        }else if('Union' === step.name){
            return (step.unionColumns||[]).map(uc=>({
                name:uc.name,
                text:uc.name,
                dataType:uc.dataType,
            }));
        }else if('Agg' === step.name){
            let prevColumns = findStepOutputColumns(steps,index-1);
            let textMap = prevColumns.reduce((m,v)=>m.set(v.name,v.text),new Map());
            return [...(step.groupNames||[]).map(n=>({name:n,text:textMap.get(n)||n,dataType:'text'})),...(step.measureItems||[]).map(m=>({
                name:`${m.name}_${m.aggregate}`,
                text:`${m.text||m.name}_${m.aggregate}`,
                dataType:'number',
                precision:m.precision||2
            }))];
        }
    }
    return [];
}

/**
 *
 * @param steps
 * @param text
 * @param count
 */
export const fixedStepText = (steps,text,count?) => {
    count = count||0;
    const exist = steps.find(step=>step.text === text);
    if(exist){
        return fixedStepText(steps,text+(count+1),count+1);
    }
    return text;
}

export const buildNewStep = (name,text) => {
    const step = {
        id:uuid(),
        name,
        text
    };

    if('Filter' === name){
        Object.assign(step,{items:[{id:uuid(),nodeType:'And'}]});
    }else if('Join' === name){
        Object.assign(step,{
            reader:'CsvReader',uri:'',how:'left',columns:[],selectedColumnNames:[],
        joinColumns:[]});
    }

    return step;
}
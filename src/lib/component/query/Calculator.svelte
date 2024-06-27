<script>
    import CircleAddButton from "./CircleAddButton.svelte";
    import {Toolbar, Button, Icon, saveIcon, List, Offcanvas, uuid} from "../../youi"
    import ExpressionEditor from "../expression/ExpressionEditor.svelte";
    import {createEventDispatcher} from "svelte";
    import {insertIntoExpression} from "../expression/helper";

    const dispatch = createEventDispatcher();

    export let expression = '';

    export let text = '';

    export let dataType = '';

    export let calculators = [];

    export let columns = [];

    export let funcItems = [];

    export let icons = _=>undefined;

    let view;

    let editorIsOpen = false;

    let editingIndex = -1;

    let editingName;

    let dialog;

    /**
     *
     */
    const addCalculator = () => {
        editorIsOpen = true;
        editingIndex = -1;
        editingName = uuid().replaceAll('-','');
        text = editingName;
        expression = '';
    }

    const editCalculator = (calculator,index)=>{
        editorIsOpen = true;
        editingIndex = index;
        text = calculator.text;
        expression = calculator.expression;
        dataType = calculator.dataType;
        editingName = undefined;
    }

    const doSave = () => {
        if(expression && text){
            if(editingName){
                calculators.push({
                    name:editingName,
                    text,
                    expression,
                });
                dispatch('change',{type:'addCalculator',text,expression});
            }else if(editingIndex>-1 && calculators[editingIndex]){
                Object.assign(calculators[editingIndex],{text,expression,dataType});
                dispatch('change',{type:'updateCalculator',text,expression});
            }
            calculators = calculators;
            editorIsOpen = false;
        }
    }

    const handle_column_dblclick = ({detail}) => {
        insertIntoExpression(view,`col(\"${detail.id}\")`)
    }

    $:items = columns.map((column,index)=>{
        const dataType = column.dataType||'text';
        return {
            id:column.name,
            text:column.text,
            group:dataType,
            datas:{num:index}
        }
    });

    $:expressionColumns = [...columns,calculators];

</script>

{#each calculators as calculator,index}
    <div data-tip={calculator.expression} on:click={()=>editCalculator(calculator,index)}
            class="tooltip flex float-left justify-center items-center min-w-16 rounded-md cursor-pointer bg-blue-50 mr-1 h-8">
        {calculator.text}
    </div>
{/each}

<CircleAddButton title="新计算列" on:click={()=>addCalculator()}/>

<Offcanvas bind:isOpen={editorIsOpen} bodyClass = {"p-0 w-10/12 flex border-t"} contentClass="flex flex-1">
    <div class="min-w-64 border-r relative  bg-gray-50">
        <List on:dblclick={handle_column_dblclick} droppable={false} multiple={false} {icons} {items}/>
    </div>

    <div class="flex flex-col flex-1">
        <Toolbar class="toolbar-xs">
            <Button title="保存计算列" on:click={()=>doSave()}>
                <Icon data={saveIcon}/>
            </Button>
        </Toolbar>
        <ExpressionEditor
                bind:view
                columns={expressionColumns}
                bind:expression bind:text bind:dataType
                {icons} on:load {funcItems}/>
    </div>
</Offcanvas>
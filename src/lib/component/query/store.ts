
import {writable} from "svelte/store";
import type {QueryModel, ColumnsStep,Filter,IQueryModelService} from "./DataQuery.d";
import {createCommand} from "../store/command.store";
import {uuid} from "../../youi/util/uuid";
import {buildNewStep, fixedStepText} from "./helper";

export function createQueryStore<T>(service:IQueryModelService<T>) {

    const {subscribe, update, set} = writable(<QueryModel>{});

    const command = createCommand<QueryModel>(set);

    let isSaving = false;

    subscribe((model:QueryModel)=>{
        if(isSaving){
            isSaving = false;
            command.setDirty(false);
            return;
        }
        command.changeState(model);
    });

    command.setInAction(false);

    return {
        subscribe,
        set,
        fetch:async (id)=>{
            const model = await service.fetch(id);
            command.reset();
            set(model);
        },

        /**
         * 保存
         */
        save:async(processor,after)=>{
            isSaving = true;
            return update(model=>{
                if(model.id){
                    if(processor){
                        processor(model);
                    }
                    const content = JSON.stringify(model);
                    service.save({id:model.id, content}).then(r=>{
                        if(after){
                            after();
                        }
                    });
                }
                return model;
            })
        },
        /**
         *
         */
        setModel:(model:QueryModel)=>{
            command.reset();
            set(model);
        },

        reset:()=>{
            return update(model=>{
                model.steps = [];
                return model;
            });
        },

        insertStep:(stepInfo,index)=>{
            const step = buildNewStep(stepInfo.name,stepInfo.text);

            return update(model=>{
                step.text = fixedStepText(model.steps,step.text)
                model.steps.splice(index+1,0,step);
                return model;
            })
        },

        removeStep:(index)=>{
            return update(model=>{
                command.setCommand(`删除步骤:${model.steps[index].text}`);
                model.steps.splice(index,1);
                return model;
            })
        },

        /**
         * 列信息变化
         * @param index
         * @param change
         */
        changeColumns:(index,changed)=>{
            return update(model=>{
                const step:ColumnsStep = model.steps[index];

                if(step){
                    let commandText = '';
                    if('select' === changed.type){
                        //列选择
                        const column = step.columns.find(c=>c.name === changed.id);
                        if(column){
                            commandText = `列选择：${column.text}-${changed.selectedIds.includes(changed.id)?'选择':'去掉选择'}`;
                            Object.assign(step,{
                                selectedColumnNames:changed.selectedIds
                            })
                        }
                    }else if('selectAll' === changed.type){
                        step.selectedColumnNames = changed.selectedIds;
                    }else if('cancelAll' === changed.type){
                        step.selectedColumnNames = [];
                    }else if('item' === changed.type){
                        if(changed.change.property === '_delete'){
                            const column = step.columns.find(c=>c.name === changed.change.id);
                            //删除列
                            commandText = `删除列:${column.text}`;
                            step.columns = step.columns.filter(c=>c.name !== changed.change.id);
                        }else{
                            //列属性变化
                            const column = step.columns.find(c=>c.name === changed.change.id);
                            if(column){
                                commandText = `列${'text'===changed.change.property?'名称':'类型'}变化：${changed.change.oldValue}=>${changed.change.value}`;
                                Object.assign(column,{
                                    ['group'===changed.change.property?'dataType':changed.change.property]:changed.change.value
                                })
                            }
                        }
                    }else if('move' === changed.type){
                        //列位置移动
                        const {position,from,to} = changed;

                        let fromIndex = step.columns.map((item,index)=>item.name === from?index:-1).reduce((a,v)=>Math.max(a,v),-1);
                        let toIndex = step.columns.map((item,index)=>item.name === to?index:-1).reduce((a,v)=>Math.max(a,v),-1);
                        let column = step.columns[fromIndex];

                        commandText = `列移动:[${column.text}]移动到[${step.columns[toIndex].text}]${'after' === position?'后':'前'}`;

                        step.columns.splice(fromIndex,1);
                        // 插入的位置
                        let insertIndex = toIndex;
                        if('after' === position && toIndex!==fromIndex-1){
                            if(toIndex<fromIndex){
                                insertIndex--;
                            }
                        }else if('before' === position && toIndex!==fromIndex+1){
                            if(toIndex>fromIndex){
                                insertIndex--;
                            }
                        }else{
                            return;
                        }
                        step.columns.splice(insertIndex,0,column);
                    }

                    command.setCommand(commandText);
                }

                return model;
            })
        },

        changeFilter:(index,changed)=>{
            return update(model=>{
                let commandText = '';
                const step:Filter = model.steps[index];

                if('remove' === changed.type){
                    // 删除
                    commandText = `删除条件:${changed.item.property} ${changed.item.operator} ${changed.item.value}`;
                }else if('addCondition' === changed.type){
                    commandText = '插入条件:';
                }else if('update' === changed.type){
                    commandText = `条件变化:${changed.oldValue} => ${changed.value}`;
                }else if('toggle' === changed.type){
                    commandText = `切换为${changed.value === 'And'?'且':'或'}`;
                }
                command.setCommand(commandText);
                return model;
            });
        },

        acceptCommand:({text})=>{
            command.setCommand(text||'');
            return update(model=>{
                return model;
            })
        },
        /**
         * 撤销
         */
        undo:()=>command.undo(),
        /**
         * 重做
         */
        redo:()=>command.redo(),
        /**
         * 是否可重做
         */
        canRedo : ()=>command.canRedo(),
        /**
         * 是否可撤销
         */
        canUndo : ()=>command.canUndo(),
        /**
         * 是否发生数据变化
         */
        isDirty:()=>command.isDirty(),

        getHistories:()=>command.getHistories()
    }

}
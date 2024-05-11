import {writable} from "svelte/store";
import type {ReportModel} from "./Report";
import {resizeColumn} from "./helper";

export function createReportStore() {

    const {subscribe, update, set} = writable(<ReportModel>{});

    let states:string[] = [];
    let index = 0;
    let inAction = true;
    let dirty = false;
    let isSaving = false;

    subscribe((model:ReportModel)=>{
        if(isSaving){
            isSaving = false;
            dirty = false;
            return;
        }
        if(index < 0)return;
        if(states.length>0){
            if (!inAction){
                if (states.length > index + 1) {
                    states = states.slice(0, index + 1)
                }
                states.push(JSON.stringify(model))
                index++;
                dirty = true;
            }
        }else{
            states.push(JSON.stringify(model));
        }
    });

    inAction = false;

    function processWidget(widgetId,processor) {
        return update(model=>{
            model.widgets = model.widgets.map(w=>{
                if(w.id === widgetId){
                    processor(w);
                }
                return w;
            });
            return model;
        });
    }

    return {
        subscribe,

        setModel:(model:ReportModel)=>{
            index = 0;
            dirty = false;
            states= [];
            set(model)
        },

        resizeColumn:(widgetId,columnIndex,value)=>{
            return processWidget(widgetId,w=>{
                resizeColumn(w,columnIndex,value);
            });
        },

        updateText:(widgetId,rowIndex,columnIndex,text)=>{
            return processWidget(widgetId,w=>{
                const index = rowIndex*w.colModels.length+columnIndex;
                w.data[index].text = text;
            });
        },
        /**
         * 撤销
         */
        undo:()=>{
            inAction = true
            if (index > 0) {
                index--
            }
            const obj:ReportModel = JSON.parse(states[index]);
            set(obj);
            inAction = false
        },
        /**
         * 重做
         */
        redo:()=>{
            inAction = true
            if (index < states.length - 1) {
                index++
            }
            const obj:ReportModel = JSON.parse(states[index]);
            set(obj);
            inAction = false
        },
        /**
         * 是否可重做
         */
        canRedo : () => {
            return index < states.length - 1
        },
        /**
         * 是否可撤销
         */
        canUndo : () => {
            return states.length>1 && index > 0
        },
        /**
         * 是否发生数据变化
         */
        isDirty:()=>{
            return index>0 && dirty;
        }
    }
}
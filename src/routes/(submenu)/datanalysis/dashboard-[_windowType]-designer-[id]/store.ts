import {writable} from "svelte/store";
import type {DashboardModel} from "./dashboardModel";

export const createModel = () => {

    const {subscribe, update, set} = writable(<DashboardModel>{
        page:{
            w:1920,
            h:1080
        }
    });

    return {
        subscribe,

        setModel:(model:DashboardModel)=>{
            return set(model)
        },

        addLine:(direction,value)=>{
            return update(model=>{
                model.page.lines = model.page.lines||[];
                model.page.lines.push({direction,value});
                return model
            })
        },

        updateLine:(index,value)=>{
            return update(model=>{
                if(model.page.lines && model.page.lines[index]){
                    model.page.lines[index].value = value;
                }
                return model
            })
        },

        removeLine:(index)=>{
            return update(model=>{
                model.page.lines = model.page.lines||[];
                model.page.lines.splice(index,1);
                return model
            })
        },
    }
}
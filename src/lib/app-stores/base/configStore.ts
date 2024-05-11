import {writable} from "svelte/store";
import type {Config} from "../../app-entity/base/config";
import {findConfig} from "../../app-services/base/configServices";

export const createStore = () => {

    const {subscribe, update, set} = writable<Config>({
        dataDir: "", areaId: "", ownerAreaId: ""
    });

    return {

        subscribe,

        fetch:async ()=>{
            const config = await findConfig();
            return set(config)
        },

        /**
         *
         * @param name
         * @param value
         */
        updateConfig:async (name,value)=>{
            return update(config=>{
                Object.assign(config,{[name]:value})
                return config;
            });
        }
    }
}

export const configStore = createStore();
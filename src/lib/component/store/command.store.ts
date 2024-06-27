
interface CommandState {
    state:string,
    text:string
}

export const createCommand = <T> (set)=>{

    let states:CommandState[] = [];

    let histories = [];

    let index = 0;
    let inAction = true;
    let dirty = false;

    let text = '';

    return {
        changeState:(model:T)=>{
            if (index < 0) {
                return;
            }

            if(states.length > 0 ){
                if (!inAction){
                    if (states.length > index + 1) {
                        states = states.slice(0, index + 1)
                    }
                    states.push({
                        state:JSON.stringify(model),
                        text
                    });
                    histories.push(text);
                    index++;
                    dirty = true;
                }
            }else{
                states.push({
                    state:JSON.stringify(model),
                    text
                });
                histories.push(text);
            }
        },

        reset:()=>{
            index = 0;
            dirty = false;
            states= [];
            histories = [];
        },

        /**
         * 撤销
         */
        undo:()=>{
            inAction = true
            if (index > 0) {
                histories.push('撤销-'+states[index].text);
                index--
            }
            const obj:T = JSON.parse(states[index].state);
            set(obj);
            inAction = false
        },
        /**
         * 重做
         */
        redo:()=>{
            inAction = true
            histories.push('重做-'+states[index+1].text);
            if (index < states.length - 1) {
                index++
            }
            const obj:T = JSON.parse(states[index].state);
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
        },

        setDirty:(theDirty)=>{
            dirty = theDirty;
        },

        setInAction:(theAction)=>{
            inAction = theAction;
        },

        setCommand:(theText)=>{
            text = theText;
        },

        getHistories:()=>{
            return histories;
        }
    }
}
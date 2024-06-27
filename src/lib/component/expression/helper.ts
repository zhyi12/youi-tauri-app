
export const OPS = [
    {name:'add',symbol:'+'},
    {name:'minus',symbol:'-'},
    {name:'multi',symbol:'*'},
    {name:'div',symbol:'/'},
    {name:'eq',symbol:'=='},
    {name:'lt',symbol:'<'},
    {name:'lte',symbol:'<='},
    {name:'gt',symbol:'>'},
    {name:'gte',symbol:'>='},
    {name:'not',symbol:'!='},
    {name:'quote',symbol:'"'},
];

export const insertIntoExpression = (view,text) => {
    if(view && text){
        view.dispatch(view.state.replaceSelection(text))
    }
}
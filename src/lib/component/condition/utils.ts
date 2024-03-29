
export const connItems = [{name:'and',text:'且'},{name:'or',text:'或'}];
export const operatorItems = [
    {name:'eq',text:'='},
    {name:'lt',text:'<'},
    {name:'lte',text:'<='},
    {name:'gt',text:'>'},
    {name:'gte',text:'>='},
    {name:'nq',text:'!='}
];

/**
 *
 * @param operatorName
 */
export const operatorConvert = (operatorName:string)=>
    operatorItems.filter(({name})=>name == operatorName)
        .map(({text})=>text).join('')

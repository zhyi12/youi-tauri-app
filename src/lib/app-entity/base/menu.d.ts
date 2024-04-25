export interface MenuInfo {
    id:string,
    name:string,
    text:string,
    route?:string,
    group?:string,
    module?:string,
    href?:string,
    icon?:string,
    scale?:number,
    children?:Array<MenuInfo>
}

export interface MenuPath{
    text:string,
    href?:string,
    icon?:string
}
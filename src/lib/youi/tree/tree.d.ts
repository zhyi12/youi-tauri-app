export interface TreeItem{
    id:string,
    text:string,
    html?:string,
    children?:TreeItem[],
    group?:string,
    icon?:string,
    href?:string,
    datas?:Record<string, unknown>,
    path?:string
}

// import { SvelteComponentTyped } from 'svelte';
export type TreeNodeProps =
    | string
    | number
    | {
    id?: string;
    text?: string;
};

export declare enum InsertPosition {
    before = 'before',
    child = 'child',
    after = 'after'
}
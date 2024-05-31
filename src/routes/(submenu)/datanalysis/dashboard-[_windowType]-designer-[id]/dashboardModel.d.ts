/**
 *
 */
export interface DashboardModel {
    page:Page
}

interface Page {
    w:number,
    h:number,
    lines?:Line[]
}

export declare enum LineDirection {
    /**
     * 水平线
     */
    H = 'h',
    /**
     * 垂直线
     */
    V = 'v'
}

export interface Line {

    id?:number,

    index?:number,

    value: number,
    // 方向
    direction: LineDirection
}

// export const REPORT_CONTAINER_CONTEXT = "ReportContainer";

import type {TableWidget} from "./Report";

export const DEFAULT_COL_WIDTH = 80;

/**
 *
 */
export function resizeColumn<T>(widget:TableWidget<T>,columnIndex,value){
    widget.colModels[columnIndex].width = value;
}
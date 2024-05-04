import type {CellPosition,CellData,Range,Area,Position} from "./DataGrid.d";
import Konva from 'konva';
import {isNull} from "../util/string.util";
import {drawCell} from "./cell";

/**
 *
 * @param contentSize 容器尺寸
 * @param scroll 滚动距离
 * @param count item数量
 * @param sizer item size函数
 */
export function calculateRange(contentSize:number,scroll:number,count:number,sizer:(index: number) => number):Range {
    //
    let start = -1,stop = -1;
    let sum = 0;
    const sizes = [];
    for(let i=0;i<count;i++){
        const size = sizer(i);
        const offset = sum;
        sum += size;
        if(start ==  -1 && sum>scroll){
            start = i;
        }

        if(start>-1 && stop === -1){
            sizes.push({size,offset,index:i});
        }

        if(stop === -1 && sum>scroll+contentSize){
            stop = i;
        }
    }

    if(stop === -1){
        stop = count-1;
    }

    return {start,stop,sizes,scroll}
}


/**
 *
 * @param rows
 * @param rowHeight
 */
export function calculateRowHeights(height:number,start:number,rows:number,rowHeight:(index: number) => number):number[]{
    const rowHeights:number[] = [];
    if(height>0){
        let sumHeight = 0;
        for(let i = start;i< rows;i++){
            const cellHeight = rowHeight(i);
            rowHeights.push(cellHeight);
            sumHeight += cellHeight;
            if(sumHeight>height){
                break;
            }
        }
    }
    return rowHeights;
}

/**
 *
 * @param width
 * @param start
 * @param columns
 * @param columnWidth
 */
export function calculateColumnWidths(width:number,start:number,columns:number,columnWidth:(index: number) => number):number[]{
    const columnWidths:number[] = [];
    if(width>0){
        let sumWidth = 0;
        for(let i = start;i< columns;i++){
            const cellWidth = columnWidth(i);
            columnWidths.push(cellWidth);
            sumWidth += cellWidth;
            if(sumWidth>width){
                break;
            }
        }
    }
    return columnWidths;
}

/**
 *
 * @param colWidths
 * @param rowHeights
 * @param startRow
 * @param startCol
 */
export function buildShowCells(
    frozenRows:number,frozenColumns:number,
    rowRange:Range,colRange:Range,
    mergedCellMap:Map<string,Area>,
    data:({rowIndex,columnIndex}:CellPosition)=>CellData):CellData[]{

    const cells:CellData[] = [];

    if(rowRange.sizes.length === 0 || colRange.sizes.length===0){
        return cells;
    }

    const {start:startRow,stop:stopRow,sizes:rowSizes,scroll:scrollTop} = rowRange;
    const {start:startColumn,stop:stopColumn,sizes:colSizes,scroll:scrollLeft} = colRange;

    for(let rowIndex=startRow;rowIndex<=stopRow;rowIndex++){
        if(rowIndex<frozenRows){
            continue;
        }
        const rowSize = rowSizes[rowIndex-startRow];
        if(!rowSize)break;
        for(let columnIndex=startColumn;columnIndex<=stopColumn;columnIndex++){

            if(columnIndex<frozenColumns){
                continue;
            }

            const colSize = colSizes[columnIndex-startColumn];
            const cellData = data({rowIndex,columnIndex})||{text:''};
            Object.assign(cellData,{rowIndex,columnIndex});
            let cell = undefined;
            if(isMergedCell(mergedCellMap,{rowIndex,columnIndex})){
                const merged = mergedCellMap.get(cellIdentifier(rowIndex,columnIndex));
                if(merged && merged.startRow == rowIndex && merged.startCol == columnIndex){
                    //
                    cell = {
                        ...cellData,
                        x:colSize.offset,
                        y:rowSize.offset,
                        width:calculateMergeSize(merged.startCol  - startColumn,merged.endCol - startColumn,colSizes),
                        height:calculateMergeSize(merged.startRow - startRow,merged.endRow - startRow,rowSizes),
                    };
                }
            }else{
                cell = {
                ...cellData,
                    x:colSize.offset,
                    y:rowSize.offset,
                    width:colSize.size,
                    height:rowSize.size,
                }
            }

            if(cell){
                cells.push(cell);
            }
        }
    }

    return cells;
}

/**
 *
 * @param frozenRows
 * @param tableRowRange
 * @param colRange
 * @param mergedCellMap
 * @param data
 */
export function buildFrozenRowCells(frozenRows:number,tableRowRange:Range,colRange:Range,mergedCellMap:Map<string,Area>,
                                    data:({rowIndex,columnIndex}:CellPosition)=>CellData):CellData[]{
    const cells:CellData[] = [];

    if(frozenRows === 0 ||!tableRowRange){
        return cells;
    }

    const rowSizes:Position[] = tableRowRange.sizes.filter(size=>size.index<=frozenRows);
    const {start:startColumn,stop:stopColumn,sizes:colSizes} = colRange;
    for(let rowIndex=0;rowIndex<frozenRows;rowIndex++){
        const rowSize = rowSizes[rowIndex];
        if(!rowSize)break;
        for(let columnIndex = startColumn;columnIndex<=stopColumn;columnIndex++){
            const colSize = colSizes[columnIndex-startColumn];
            if(!colSize)break;
            const cellData = data({rowIndex,columnIndex})||{text:''};
            Object.assign(cellData,{rowIndex,columnIndex});
            if(isMergedCell(mergedCellMap,{rowIndex,columnIndex})){
                const merged = mergedCellMap.get(cellIdentifier(rowIndex,columnIndex));
                if(merged && merged.startRow == rowIndex && merged.startCol == columnIndex){
                    //
                    cells.push({
                        ...cellData,
                        x:colSize.offset,
                        y:rowSize.offset,
                        width:calculateMergeSize(merged.startCol  - startColumn,merged.endCol - startColumn,colSizes),
                        height:calculateMergeSize(merged.startRow,merged.endRow,rowSizes),
                    });
                }
            }else if(rowSize && colSize){
                cells.push({
                    ...cellData,
                    x:colSize.offset,
                    y:rowSize.offset,
                    width:colSize.size,
                    height:rowSize.size,
                });
            }
        }
    }
    return cells;
}

/**
 *
 * @param frozenColumns
 * @param rowRange
 * @param tableColRange
 * @param mergedCellMap
 * @param data
 */
export function buildFrozenColumnCells(frozenColumns:number,rowRange:Range,tableColRange:Range,mergedCellMap:Map<string,Area>,
                                    data:({rowIndex,columnIndex}:CellPosition)=>CellData):CellData[] {
    const cells:CellData[] = [];

    if(frozenColumns === 0){
        return cells;
    }

    const {start:startRow,stop:stopRow,sizes:rowSizes} = rowRange;
    const colSizes:Position[] = tableColRange.sizes.filter(size=>size.index<=frozenColumns);
    for(let rowIndex=startRow;rowIndex<=stopRow;rowIndex++){
        const rowSize = rowSizes[rowIndex - startRow];
        if(!rowSize)break;
        for(let columnIndex = 0;columnIndex<frozenColumns;columnIndex++){
            const colSize = colSizes[columnIndex];
            const cellData = data({rowIndex,columnIndex})||{text:''};
            Object.assign(cellData,{rowIndex,columnIndex});
            if(isMergedCell(mergedCellMap,{rowIndex,columnIndex})){
                const merged = mergedCellMap.get(cellIdentifier(rowIndex,columnIndex));
                if(merged && merged.startRow == rowIndex && merged.startCol == columnIndex){
                    cells.push({
                        ...cellData,
                        x:colSize.offset,
                        y:rowSize.offset,
                        width:calculateMergeSize(merged.startCol, merged.endCol,colSizes),
                        height:calculateMergeSize(merged.startRow-startRow ,merged.endRow-startRow,rowSizes),
                    });
                }
            }else{
                cells.push({
                    ...cellData,
                    x:colSize.offset,
                    y:rowSize.offset,
                    width:colSize.size,
                    height:rowSize.size,
                });
            }

        }
    }

    return  cells;
}

/**
 *
 * @param frozenRows
 * @param frozenColumns
 * @param tableRowRange
 * @param tableColRange
 * @param mergedCellMap
 * @param data
 */
export function buildIntersectionCells(frozenRows:number,frozenColumns:number,tableRowRange:Range,tableColRange:Range,
                                       mergedCellMap:Map<string,Area>, data:({rowIndex,columnIndex}:CellPosition)=>CellData):CellData[] {
    const cells:CellData[] = [];
    const rowSizes:Position[] = tableRowRange.sizes.filter(size=>size.index<=frozenRows);
    const colSizes:Position[] = tableColRange.sizes.filter(size=>size.index<=frozenColumns);
    for(let rowIndex=0;rowIndex<frozenRows;rowIndex++){
        const rowSize = rowSizes[rowIndex];
        if(!rowSize)break;
        for(let columnIndex = 0;columnIndex<frozenColumns;columnIndex++){
            const colSize = colSizes[columnIndex];
            const cellData = data({rowIndex,columnIndex})||{text:''};
            Object.assign(cellData,{rowIndex,columnIndex});
            if(isMergedCell(mergedCellMap,{rowIndex,columnIndex})){
                const merged = mergedCellMap.get(cellIdentifier(rowIndex,columnIndex));
                if(merged && merged.startRow == rowIndex && merged.startCol == columnIndex){
                    cells.push({
                        ...cellData,
                        x:colSize.offset,
                        y:rowSize.offset,
                        width:calculateMergeSize(merged.startCol,merged.endCol,colSizes),
                        height:calculateMergeSize(merged.startRow ,merged.endRow,rowSizes),
                    });
                }
            }else{
                cells.push({
                    ...cellData,
                    x:colSize.offset,
                    y:rowSize.offset,
                    width:colSize.size,
                    height:rowSize.size,
                });
            }
        }
    }

    return cells;
}

/**
 *
 * @param start
 * @param end
 * @param sizes
 */
function calculateMergeSize(start:number,end:number,sizes:Position[]):number{
    let mergeSize = 0;
    for(let i=Math.max(0,start);i<=Math.min(end,sizes.length-1);i++){
        mergeSize+=sizes[i].size;
    }
    return mergeSize;
}
/**
 *
 * @param rowRange
 * @param colRange
 * @param offsetX
 * @param offsetY
 */
export function findCellPosition(rowRange:Range,colRange:Range,offsetX:number,offsetY:number):CellPosition|undefined {
    const rowSize = rowRange.sizes.filter(({size,offset})=>offsetY>offset-rowRange.scroll && offsetY<=offset+size-rowRange.scroll)[0];
    const columnSize = colRange.sizes.filter(({size,offset})=>offsetX>offset-colRange.scroll && offsetX<=offset+size-colRange.scroll)[0];
    if(!rowSize || !columnSize){
        return undefined;
    }
    return {rowIndex:rowSize.index,columnIndex:columnSize.index};
}

/**
 *
 * @param a
 * @param b
 */
export const isEqualCells = (
    a: CellPosition | null,
    b: CellPosition | null
) => {
    if (isNull(a) || isNull(b) || a === null || b === null) return false;
    return a.rowIndex === b.rowIndex && a.columnIndex === b.columnIndex;
};
/**
 *
 * @param a
 * @param b
 */
export const isEqualAreas = (
    a:Area,
    b:Area
)=>{
    return a.startRow === b.startRow && a.endRow === b.endRow
        && a.startCol === b.startCol && a.endCol === b.endCol
}
/**
 *
 * @param mergedCells
 */
export const toMergeMap = (mergedCells:Area[]) => {
    const mergedCellMap = new Map();
    if(mergedCells){
        for (let i = 0; i < mergedCells.length; i++) {
            const bounds = mergedCells[i];
            for (const cell of getBoundedCells(bounds)) {
                mergedCellMap.set(cell, bounds);
            }
        }
    }
    return mergedCellMap;
}

export const isMergedCell = (mergedCellMap:Map<string,Area>,{rowIndex,columnIndex}: CellPosition) => mergedCellMap.has(cellIdentifier(rowIndex, columnIndex));

/**
 *
 * @param area
 */
export const getBoundedCells = (area: Area | null | undefined) => {
    const cells = new Set();
    if (!area) return cells;
    const { startRow, endRow, startCol, endCol } = area;
    for (let i = startRow; i <= endRow; i++) {
        for (let j = startCol; j <= endCol; j++) {
            cells.add(cellIdentifier(i, j));
        }
    }
    return cells;
};

/* Create a string cell identifier */
export const cellIdentifier = (rowIndex: number, columnIndex: number): string =>
    `${rowIndex},${columnIndex}`;

/**
 * 根据合并框扩展选区
 */
export const expandedAreaByMerge = (area:Area,mergedCellMap:Map<string,Area>):Area|undefined => {
    if(area.startRow == area.endRow && area.startCol == area.endCol){
        //单个单元格
        const key = cellIdentifier(area.startRow,area.startCol);
        if(mergedCellMap.has(key)){
            return mergedCellMap.get(key);
        }
    }else{
        const inAreaMerges = new Map();
        //根据边线扩展区域
        //左右边
        for(let i=area.startRow;i<=area.endRow;i++){
            addMergeArea(mergedCellMap,cellIdentifier(i,area.startCol),inAreaMerges);
            addMergeArea(mergedCellMap,cellIdentifier(i,area.endCol),inAreaMerges);
        }
        //上下边
        for(let j=area.startCol;j<=area.endCol;j++){
            addMergeArea(mergedCellMap,cellIdentifier(area.startRow,j),inAreaMerges);
            addMergeArea(mergedCellMap,cellIdentifier(area.endRow,j),inAreaMerges);
        }
        //区域边线有合并单元格时，扩展选择区域
        if(inAreaMerges.size){
            let extendedArea = {...area};
            inAreaMerges.forEach((mergeArea,_)=>{
                extendedArea = Object.assign(extendedArea,{
                    startRow:Math.min(extendedArea.startRow,mergeArea.startRow),
                    endRow:Math.max(extendedArea.endRow,mergeArea.endRow),
                    startCol:Math.min(extendedArea.startCol,mergeArea.startCol),
                    endCol:Math.max(extendedArea.endCol,mergeArea.endCol)
                })
            });

            if(inAreaMerges.size>0){
                if(isEqualAreas(area,extendedArea)){
                    return area;
                }else{
                    return expandedAreaByMerge(extendedArea,mergedCellMap);
                }
            }
        }
    }

    return area;
}
/**
 *
 * @param area
 * @param tableColRange
 * @param tableRowRange
 * @param frozenRows
 * @param frozenColumns
 * @param scrollLeft
 * @param scrollTop
 */
export const toBox = (area:Area,tableColRange:Range,tableRowRange:Range,
                      frozenRows:number,frozenColumns:number,scrollLeft:number,scrollTop:number)=>{
    const {startRow,endRow,startCol,endCol} = area;
    if(!tableColRange.sizes[startCol] || !tableRowRange.sizes[startRow]
        || !tableColRange.sizes[endCol]){
        return {x:0,y:0,width:0,height:0};
    }

    let x = tableColRange.sizes[startCol].offset;
    let y = tableRowRange.sizes[startRow].offset;
    let width = tableColRange.sizes[endCol].offset - x + tableColRange.sizes[endCol].size;
    let height = tableRowRange.sizes[endRow].offset - y + tableRowRange.sizes[endRow].size;

    if(startRow<frozenRows){
        //行冻结区域
        height = tableRowRange.sizes[endRow].offset + tableRowRange.sizes[endRow].size - y;
        if(endRow>=frozenRows){
            height = height - scrollTop;
        }
    }else {
        y = y - scrollTop;
    }

    if(startCol<frozenColumns){
        //列冻结区域
        width = tableColRange.sizes[endCol].offset + tableColRange.sizes[endCol].size - x;
        if(endCol>=frozenColumns){
            width = width - scrollLeft;
        }
    }else{
        x = x - scrollLeft;
    }

    return {x,y,width,height};
}

/**
 *
 * @param mergedCellMap
 * @param key
 * @param inAreaMerges
 */
function addMergeArea(mergedCellMap:Map<string,Area>,key:string,inAreaMerges:Map<string,Area>) {
    if(mergedCellMap.has(key)){
        const mergedArea = mergedCellMap.get(key);
        if(mergedArea){
            const mergedKey = cellIdentifier(mergedArea.startRow,mergedArea.startCol);
            if(!inAreaMerges.has(mergedKey)){
                inAreaMerges.set(mergedKey,mergedArea);
            }
        }
    }
}
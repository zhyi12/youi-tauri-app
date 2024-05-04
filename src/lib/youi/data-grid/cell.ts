import Konva from 'konva';
import {isNull} from "../util/string.util";

/**
 *
 * @param layer
 * @param cells
 * @param frozenRowCells
 * @param frozenColumnCells
 * @param frozenIntersectionCells
 */
export const drawCells = (layer,cells,frozenRowCells,frozenColumnCells,frozenIntersectionCells,scrollTop,scrollLeft,showGrid) => {
    if(layer){
        layer.removeChildren();
        if(cells.length){
            drawCellsGroup(layer,cells,scrollLeft,scrollTop,showGrid);
            drawCellsGroup(layer,frozenRowCells,scrollLeft,0,showGrid);
            drawCellsGroup(layer,frozenColumnCells,0,scrollTop,showGrid);
            drawCellsGroup(layer,frozenIntersectionCells,0,0,showGrid);
        }
    }
}

/**
 *
 * @param layer
 * @param cells
 * @param offsetX
 * @param offsetY
 */
function drawCellsGroup(layer,cells,offsetX,offsetY,showGrid) {
    if(cells && cells.length){
        const group = new Konva.Group({
            offsetX,offsetY
        });
        cells.forEach(cell=>drawCell(group,cell,showGrid));
        layer.add(group);
    }
}

/**
 *
 * @param layer
 * @param cell
 */
export const drawCell = (parent,cell,showGrid) => {
    if(!parent)return;

    const group = new Konva.Group({
        x:cell.x, y:cell.y, width:cell.width, height:cell.height
    });

    const rect = {
        x: 0,
        y: 0,
        width: cell.width,
        height: cell.height,
        fill: cell.fill
    }

    if(showGrid){
        Object.assign(rect,{
            stroke: 'silver',
            strokeWidth: 1,
        })
    }

    group.add(new Konva.Rect(rect));
    // 文本
    drawText(group,cell);
    // 边框
    drawBorder(group,cell);
    parent.add(group);
}

/**
 *
 * @param group
 * @param cell
 */
function drawText(group:Konva.Group,cell) {
    if(!isNull(cell.text)){
        const fontSize = cell.fontSize||14;
        const offset = Math.floor((cell.height - fontSize)/2)-1;
        //计算位置
        group.add(new Konva.Text({
            padding:2,
            fontSize,
            fontStyle:cell.fontStyle,
            align:cell.align,
            letterSpacing:cell.letterSpacing,
            x:1,
            y:offset,
            text:cell.text,
            width:cell.width,
            height:cell.height,
            wrap:'none',
            ellipsis:true
        }))
    }
}
/**
 * 边框
 */
function drawBorder(group:Konva.Group,cell) {
    const strokeWidth:number = cell.borderWidth||1;
    if(cell.borderLeft){
        group.add(new Konva.Line({
            stroke:cell.borderLeft,
            strokeWidth,
            points:[0,0,0,cell.height+1]
        }));
    }

    if(cell.borderTop){
        group.add(new Konva.Line({
            stroke:cell.borderTop,
            strokeWidth:<number>cell.borderTopWidth||strokeWidth,
            points:[0,0,cell.width,0]
        }));
    }

    if(cell.borderRight){
        group.add(new Konva.Line({
            stroke:cell.borderRight,
            strokeWidth,
            points:[cell.width,0,cell.width,cell.height]
        }));
    }

    if(cell.borderBottom){
        group.add(new Konva.Line({
            stroke:cell.borderBottom,
            strokeWidth:<number>cell.borderBottomWidth||strokeWidth,
            points:[0,cell.height,cell.width,cell.height]
        }));
    }
}
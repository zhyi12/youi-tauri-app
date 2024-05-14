use std::cmp::max;
use geo::{BoundingRect, Centroid, Coord, GeodesicArea, Polygon, Rect};
use log::debug;
use crate::Hexagon;

///
/// 使用六边形网格切分矩形
///
pub struct RectHexGrid<'a>{
    ///
    /// 矩形
    ///
    rect:&'a Rect,

    ///
    /// 基础六边形
    ///
    hex:Hexagon,

    meter_size:f64,

    x_start:f64,

    y_start:f64,

    x_end:f64,

    y_end:f64,

}

impl<'a> RectHexGrid<'a> {

    pub fn new(rect:&'a Rect,meter_size:f64)->Self{

        let max_coord = rect.max();
        let min_coord = rect.min();

        let x_start = max_coord.x.min(min_coord.x);
        let y_start = max_coord.y.min(min_coord.y);
        let x_end = max_coord.x.max(min_coord.x);
        let y_end = max_coord.y.max(min_coord.y);

        // 矩形区域的第一个六边形
        let hex = Hexagon::new(x_start,y_start,meter_size);

        Self{
            rect,
            meter_size,
            hex,
            x_start,
            y_start,
            x_end,
            y_end
        }
    }
}

impl<'a> RectHexGrid<'a> {

    ///
    ///
    ///
    pub fn split_grid(&self)->Vec<Hexagon>{
        let mut cells = vec![];
        // 首六边形单元格
        cells.push(self.hex.clone());
        //经度移动
        let mut first_row_cells = vec![];
        let mut right_hex = self.hex.neighbor(120.);

        let mut index = 0;
        while right_hex.x < self.x_end{
            cells.push(right_hex.clone());
            first_row_cells.push(right_hex);
            right_hex = right_hex.neighbor(if index%2==0 {60.} else {120.});
            index = index+1;
        }
        //纬度移动
        let mut first_column_cells = vec![];
        let mut down_hex = self.hex.neighbor(0.);

        while down_hex.y < self.y_end{
            first_column_cells.push(1);
            cells.push(down_hex);
            down_hex = down_hex.neighbor(0.);
        }

        // 遍历经度纬度移动
        for x in 0..first_row_cells.len() {
            let mut next_hex = first_row_cells[x].clone();
            for _y in 0..first_column_cells.len(){
                cells.push(next_hex);
                next_hex = next_hex.neighbor(0.);
            }
        }
        cells
    }
}
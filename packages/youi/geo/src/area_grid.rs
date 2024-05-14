use geo::{BoundingRect, Contains, coord, GeodesicArea, Polygon, Rect};
use log::debug;
use polars_core::frame::row::Row;
use polars_core::prelude::{AnyValue, DataFrame, DataType, Field, Schema};
use polars_io::csv::QuoteStyle;
use polars_io::prelude::CsvWriter;
use polars_io::SerWriter;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use crate::{Area, Hexagon};
use crate::hex_grid::RectHexGrid;


///
///
///
pub fn to_grid(areas:&Vec<Area>,hex_square:f64,out_path:&str){

    // 六边形比宽(米)
    let metre_size = (hex_square*2./3./3_f64.sqrt()).sqrt();

    let area = &areas[0];

    // DataFrame
    let schema = Schema::from_iter(
        vec![
            Field::new("code",DataType::String),
            Field::new("x",DataType::Float64),
            Field::new("y",DataType::Float64)]);

    areas.into_par_iter().for_each(|area|{

        let cells = area.polygons.iter().map(|poly|{
            let covers = poly.geodesic_area_signed().abs()/1000./1000.;
            if covers>1.{
                let rect = poly.bounding_rect().unwrap();
                let grid_cells = RectHexGrid::new(&rect,metre_size).split_grid();
                debug!("{} {} 切分：{}",area.text,covers,grid_cells.len());
                grid_cells
                    .into_par_iter()
                    .filter(|cell|is_area_cell(poly,cell))
                    .map(|cell|Row::new(vec![AnyValue::String(area.code.as_str()), AnyValue::Float64(cell.x),AnyValue::Float64(cell.y)]))
                    .collect::<Vec<Row>>()
            }else{
                vec![]
            }
        }).flatten()
            .collect::<Vec<Row>>();
        //
        let mut df = DataFrame::from_rows_and_schema(&cells,&schema).unwrap();

        let out_file = std::fs::File::create(format!("{}_{out_path}",area.code)).unwrap();

        CsvWriter::new(out_file).with_quote_style(QuoteStyle::Never).finish(&mut df).unwrap();

    });
}

///
/// 完善包含判断
///
fn is_area_cell(poly: &Polygon, cell: &Hexagon) ->bool{
    let is_in = poly.contains(&coord! {x:cell.x,y:cell.y});
    is_in
}
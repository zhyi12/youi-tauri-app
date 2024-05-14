use geo::{BooleanOps, BoundingRect, Centroid, Contains, ConvexHull, coord, Coord, GeodesicArea, GeodesicDestination, GeodesicDistance, Intersects, LineString, Point, point, Polygon};
use geojson::{Feature,FeatureCollection, GeoJson, PolygonType, Value};
use log::debug;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use crate::cell::Cell;
use crate::hex_grid::RectHexGrid;
use crate::Hexagon;

pub struct Area{

    pub code:String,

    pub text:String,

    ///
    /// 多边形集合
    ///
    pub(crate) polygons:Vec<Polygon>
}

impl From<&Value> for Area{

    fn from(value: &Value) -> Self {
        let polygons = match value{
            Value::Polygon(x) => vec![to_polygon(x)],
            Value::MultiPolygon(x) => x.iter()
                .map(|p|to_polygon(p))
                .collect::<Vec<Polygon>>(),
            _=>vec![]
        };

        Area{
            code:"".to_string(),
            text:"".to_string(),
            polygons
        }
    }
}

impl Area {

    pub fn set_code(mut self,code:&str)->Self{
        self.code = code.to_string();
        self
    }

    pub fn set_text(mut self,text:&str)->Self{
        self.text = text.to_string();
        self
    }
}


impl Area {

    //生成网格
    pub fn to_grid(&self,hex_size:f64)->Vec<Hexagon>{
        (&self.polygons).iter()
            .map(|p|polygon_to_cells(&self,p,hex_size))
            .flatten()
            .collect::<Vec<Hexagon>>()
    }
}

///
///
///
fn to_polygon(geometry_polygon:&PolygonType)->Polygon{
    let flat = geometry_polygon.iter()
        .map(|ps|ps.iter()).flatten();
    let points:Vec<Coord<f64>> = flat.map(|f|{
        let x = f.get(0).unwrap().clone();
        let y  = f.get(1).unwrap().clone();
        coord!(x:x,y:y)
    }).collect();

    let line = LineString::new(points);

    Polygon::new(line,vec![])
}

///
/// 网格切分
///
fn polygon_to_cells(_area:&Area,poly:&Polygon,hex_size:f64)->Vec<Hexagon>{
    let covers = poly.geodesic_area_signed().abs()/1000./1000.;
    //大于1平方公里的区域
    if covers>1.{
        let rect = poly.bounding_rect().unwrap();
        let hex_grid = RectHexGrid::new(&rect,hex_size);
        hex_grid.split_grid()
    }else{
        vec![]
    }
}
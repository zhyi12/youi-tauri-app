use std::f64::consts::PI;
use geo::{EuclideanDistance, GeodesicBearing, GeodesicDistance, point, Point};
use serde::{Deserialize, Serialize};
///
/// 点数据
///
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PointData{
    x:f64,
    y:f64,
    values:Vec<f64>,
    translate_x:f64,
    translate_y:f64
}

impl PointData {

    pub fn new(x:f64,y:f64,values:Vec<f64>)->Self{
        Self{
            x,
            y,
            values,
            translate_x:0.,
            translate_y:0.
        }
    }
}

impl PointData {

    pub fn calculate_translate(&mut self,x:f64,y:f64){
        let p = point!{x:self.x,y:self.y};
        let (bearing,distance) = p.geodesic_bearing_distance(point!{x:x,y:y});
        self.translate_x = distance*(bearing*PI/180.).sin();
        self.translate_y = distance*(bearing*PI/180.).cos();
    }

}

///
/// [x,y,values]
///
pub fn process_chart_data(point_data:&mut Vec<PointData>){
    let start = &point_data[0];
    let x = start.x;
    let y = start.y;
    point_data.iter_mut().for_each(|p|{
        p.calculate_translate(x,y)
    });
}
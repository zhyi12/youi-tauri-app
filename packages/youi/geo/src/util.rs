use std::f64::consts::PI;
const R:f64 = 6371000.;
///
/// 两点距离（经纬度）
///
pub fn geo_earth_distance(lng1:f64,lat1:f64,lng2:f64,lat2:f64)->f64{
    let rad:f64 = PI/180.;

    let y1 = lat1 * rad;
    let y2 = lat2 * rad;

    let sin_d_lat = f64::sin((lat2 - lat1) * rad / 2.);
    let sin_d_lon = f64::sin((lng2 - lng1) * rad / 2.);

    let a = sin_d_lat * sin_d_lat + f64::cos(y1) * f64::cos(y2) * sin_d_lon * sin_d_lon;
    let c = 2. * f64::atan2(f64::sqrt(a), f64::sqrt(1. - a));

    R*c
}
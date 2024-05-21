use geo::{Geometry, GeometryCollection};
use geozero::{ToGeo, ToJson, ToMvt};
use geozero::geojson::GeoJsonString;
use geozero::wkt::Wkt;
use crate::error::GeoResult as Result;

///
/// geom格式转geo json
///
pub fn geom_to_json(geom:&str)->Result<String>{
    Ok(Wkt(geom).to_json()?)
}

///
///
///
pub fn json_to_geo(geo_json:&str) -> Result<Geometry> {
    Ok(GeoJsonString(geo_json.to_string()).to_geo()?)
}

///
///
///
pub fn merge_geometries(geometries:Vec<Geometry>) -> Result<String>{
    Ok(geo::geometry::Geometry::GeometryCollection(GeometryCollection(geometries)).to_json()?)
}
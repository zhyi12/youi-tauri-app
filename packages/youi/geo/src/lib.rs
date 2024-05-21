//!
//! 地理信息处理
//!
mod area;
mod cell;
mod util;
mod hexagon;
mod hex_grid;
mod area_grid;
mod mvt;
mod error;
mod geom;
mod chart;

pub use geo::Geometry;
pub use geojson::{FeatureCollection,Feature,JsonObject,Value,feature::Id};
pub use geo::algorithm::centroid::Centroid;
pub use geozero::geojson::GeoJsonString;
pub use area::Area;
pub use hexagon::Hexagon;
pub use area_grid::to_grid;
pub use mvt::{geom_to_mvt_data,json_to_mvt_data};
pub use geom::{geom_to_json,json_to_geo,merge_geometries};
pub use error::GeoError;
pub use chart::{process_chart_data,PointData};
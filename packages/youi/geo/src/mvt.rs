use geo::Geometry;
use geozero::mvt::{Tile, tile};
use geozero::geojson::GeoJson;
use geozero::{GeozeroGeometry, ToGeo, ToMvt};
use geozero::mvt::Message;
use geozero::mvt::tile::Feature;
use geozero::wkt::Wkt;
use crate::error::GeoResult as Result;

pub fn geom_to_mvt_data(geom:&str)->Result<Vec<u8>>{
    to_mvt_data(Wkt(geom).to_mvt_unscaled()?)
}

///
/// 输出mvt格式bytes
///
pub fn json_to_mvt_data(geo_json:&str)->Result<Vec<u8>>{
    to_mvt_data(GeoJson(geo_json).to_mvt_unscaled()?)

}

fn to_mvt_data(mvt_feature:Feature)->Result<Vec<u8>>{

    let mut mvt_tile = Tile::default();

    let mut mvt_layer = tile::Layer {
        version: 2,
        ..Default::default()
    };

    mvt_layer.name = String::from("mvtLayer");
    mvt_layer.extent = Some(4096);
    mvt_layer.features = vec![mvt_feature];

    mvt_tile.layers.push(mvt_layer);

    let mut buf = Vec::with_capacity(mvt_tile.encoded_len());

    mvt_tile.encode_raw(&mut buf);

    Ok(buf)
}

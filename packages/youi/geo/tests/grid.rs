
use geojson::{GeoJson};
use youi_geo::{Area,to_grid};
use youi_test::find_real_path;

#[test]
pub fn grid(){

    youi_test::enable_test_log();
    let file= youi_test::open_file(&youi_test::find_real_path("geo","china.geojson"));

    let geojson =  GeoJson::from_reader(file).unwrap();

    let areas = match geojson {
        GeoJson::FeatureCollection(x) => {
            x.features.iter().map(|f|{
                let gb = f.properties.as_ref().unwrap().get("gb").unwrap().to_string();

                let geometry = f.geometry.as_ref().unwrap();
                // 11 位编码
                if gb.len() == 11{
                    let name = f.properties.as_ref().unwrap().get("name").unwrap().to_string();
                    vec![Area::from(&geometry.value).set_code(&gb).set_text(&name)]
                }else{
                    vec![]
                }
            }).flatten().collect::<Vec<Area>>()
        },
        _=>vec![]
    };

    // 六边形面积
    let hex_square = 1000000.;
    to_grid(&areas,hex_square,&find_real_path("geo","hexagon"));

    // let cells = areas.into_par_iter()
    //     .map(|area|area.to_grid(metre_size))
    //     .flatten()
    //     .map(|hex|{
    //         Row::new(vec![AnyValue::Float64(hex.x),AnyValue::Float64(hex.y)])
    //     })
    //     .collect::<Vec<Row>>();
    //
    //
    // let schema = Schema::from_iter(vec![Field::new("x",DataType::Float64),Field::new("y",DataType::Float64)]);
    //
    // let mut df = DataFrame::from_rows_and_schema(&cells,&schema).unwrap();
    //
    // println!("{}",df.height());

}

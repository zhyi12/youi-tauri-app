use polars_io::SerReader;
use youi_chart::{Chart, ChartQuery};

#[test]
pub fn chart_line()->Result<(), Box<dyn std::error::Error>> {

    let mut chart = youi_test::read_from_json::<Chart>("chart","model/chart.json");

    let mut query = ChartQuery::from(&mut chart);

    let path = youi_test::find_real_path("chart","xy.csv");

    let mut df = polars_io::prelude::CsvReader::from_path(&path)?
        .infer_schema(None)
        .has_header(true)
        .finish()?;

    query.set_df(&mut df);
    query.replace_dataset();

    println!("{}",serde_json::to_string(&chart).unwrap());

    Ok(())
    // let report = chart.create_report_model();
    //
    // println!("{}",serde_json::to_string(&report).unwrap());
}
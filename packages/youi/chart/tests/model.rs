use polars_core::df;
use youi_chart::ChartOption;
use youi_test::read_from_json;

#[test]
pub fn line(){

    let mut option = read_from_json::<ChartOption>("chart","line.json");

    let mut df = df!("product" => &["Matcha Latte", "Milk Tea", "Cheese Cocoa","Walnut Brownie"],
             "2015" => &[43.3, 83.1, 86.4,72.4],
             "2016" => &[85.8, 73.4, 65.2,53.9],
             "2017" => &[93.7, 55.1, 82.5,39.1]
    ).unwrap();

    option.add_dataset(&mut df);

    println!("{}",serde_json::to_string(&option).unwrap());
}
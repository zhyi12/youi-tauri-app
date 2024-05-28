use polars_core::df;
use youi_chart::{ChartOption};
use youi_test::read_from_json;

#[test]
pub fn line(){

    let mut option = read_from_json::<ChartOption>("chart","line.json");

    let mut df = df!("product" => &["Matcha Latte", "Milk Tea", "Cheese Cocoa","Walnut Brownie"],
             "2015" => &[43.3, 83.1, 86.4,72.4],
             "2016" => &[85.8, 73.4, 65.2,53.9],
             "2017" => &[93.7, 55.1, 82.5,39.1]
    ).unwrap();

    option.add_df(&mut df);

    println!("{}",serde_json::to_string(&option).unwrap());
}

#[test]
pub fn radar(){
    read_option("radar.json");
}

#[test]
pub fn line_simple(){
    read_option("line-simple.json");
}

#[test]
pub fn line_smooth(){
    read_option("line-smooth.json");
}

#[test]
pub fn area_basic(){
    read_option("area-basic.json");
}

#[test]
pub fn line_stack(){
    read_option("line-stack.json");
}

#[test]
pub fn area_stack(){
    read_option("area-stack.json");
}

#[test]
pub fn area_stack_gradient(){
    read_option("area-stack-gradient.json");
}

#[test]
pub fn area_pieces(){
    read_option("area-pieces.json");
}

#[test]
pub fn bump_chart(){
    read_option("bump-chart.json");
}

#[test]
pub fn data_transform_filter(){
    read_option("data-transform-filter.json");
}

fn read_option(path:&str)->ChartOption{
    let option = read_from_json::<ChartOption>("chart",path);
    println!("{}",serde_json::to_string(&option).unwrap());
    option
}
use youi_table::replace_area_expression;

#[test]
pub fn transform(){

    let result = replace_area_expression("bs1","'B204'!C8");

    println!("{result}")
}
use itertools::Itertools;
use regex::{Regex};

///
/// 提取列名 ：中文、字符、数字、下划线的组合
///
pub fn parse_column_names(expression:&str)->Vec<String>{
    let regex = Regex::new("col\\(\"(?<name>[\\w_\\u4e00-\\u9fa5]+)\"\\)").unwrap();

    let names = regex.captures_iter(expression).map(|c|{
        (&c["name"]).to_string()
    })
        .sorted()
        .dedup()
        .collect::<Vec<String>>();
    names
}


#[test]
pub fn test_parse_column_names(){
    let names = parse_column_names("col(\"字段\").str_slice(0,1).in([\"A\",\"B\"]) && (col(\"industry_type\").str_slice(0,2).in([\"47\",\"48\",\"49\",\"50\"]) && col(\"report_type\").eq(\"C\")");

    println!("{:?}",names);
}




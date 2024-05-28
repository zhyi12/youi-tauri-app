use youi_chart::Filter;

#[test]
pub fn filter(){

    let j = r#"
        {
            "config":{
                "and":[
                     { "dimension": "Year", "gte": 1950 },
                     { "dimension": "Country", "eq": "Germany" }
                ]
            }
        }
    "#;

    let filter = serde_json::from_str::<Filter>(j);

    println!("{}",serde_json::to_string(&filter.unwrap()).unwrap());
}

#[test]
pub fn filter_one(){

    let j = r#"
        {
            "config":{ "dimension": "Year", "gte": 1950 }
        }
    "#;

    let filter = serde_json::from_str::<Filter>(j);

    println!("{}",serde_json::to_string(&filter.unwrap()).unwrap());
}


#[test]
pub fn filter_not(){

    let j = r#"
        {
            "config":{ "not":{"dimension": "Year", "gte": 1950} }
        }
    "#;

    let filter = serde_json::from_str::<Filter>(j);

    println!("{}",serde_json::to_string(&filter.unwrap()).unwrap());
}
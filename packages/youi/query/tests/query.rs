use youi_query::{QueryModel,DslNode};

#[test]
pub fn query(){
    let model = load_model("query.json");

    println!("{}",model.to_dsl(None).unwrap());
}

#[test]
pub fn query_reader(){
    let model = load_model("reader.json");

    println!("{}",model.to_dsl(None).unwrap());
}

#[test]
pub fn query_agg(){
    let model = load_model("agg.json");

    println!("{}",model.to_dsl(None).unwrap());
}

fn load_model(path:&str)->QueryModel{
    youi_test::read_from_json::<QueryModel>("query",path)
}
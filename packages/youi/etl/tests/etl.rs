use youi_etl::{EtlExecutor, to_dsl};
use youi_flow::Flow;
use youi_test::{enable_test_log,find_real_path, read_from_json};

#[test]
pub fn test_run_step(){

    enable_test_log();

    let mut flow = read_from_json::<Flow>("etl","flow.json");

    let dsl = EtlExecutor::from(&mut flow).run_step("node9");

    println!("dsl: {}",dsl.unwrap());

}

#[test]
pub fn test_run_flat_catalog_table_transform(){

    enable_test_log();

    let mut flow = read_from_json::<Flow>("etl","flat_catalog_table_etl.json");

    let dsl = EtlExecutor::from(&mut flow).run_step("node2");

    println!("dsl: {}",dsl.unwrap());

}
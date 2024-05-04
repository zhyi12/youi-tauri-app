
use youi_dsl::{df_engine, df_execute};
use youi_test::find_real_path;

#[test]
pub fn dsl_test(){

    let script = "\
                let df_t_jp_2018_601_record = read_csv(\"/Volumes/D/youi-app-data/我的数据/jp/000000_20230000_601.csv\")\
    .select([col(\"respondent_id\"),col(\"period_id\"),col(\"area_id\"),col(\"construction_qualifications\"),col(\"industry_type\"),col(\"reg_address_area\"),col(\"report_type\")]);\
            let df_dataset = df_t_jp_2018_601_record;\
            df_dataset\
                    .with_columns([\
                    col(\"reg_address_area\").cast_str().str_slice(0,2).alias(\"mg_15\"),\
                    when(col(\"construction_qualifications\").cast_str().str_slice(0,1).is_in([\"A\",\"B\"]) && col(\"industry_type\").cast_str().str_slice(0,2)==\"60\" && col(\"report_type\") == \"C\")\
                    .then(\"1\")\
                    .when(col(\"construction_qualifications\").cast_str().str_slice(0,1) == \"C\" && col(\"industry_type\").cast_str().str_slice(0,2)==\"60\" && col(\"report_type\")==\"C\")\
                    .then(\"2\")\
                    .otherwise(\"_999\").alias(\"mg_16\")])\
                    .agg(\"mg_15,mg_16\",[col(\"respondent_id\").count().alias(\"respondent_id_count\")])";

    test_dsl_execute(script);
}

///
///
///
#[test]
pub fn test_dsl_schema(){
    //respondent_id,period_id,area_id,employee
    let script = "read_csv(\"/Volumes/D/youi-app-data/我的数据/jp/000000_20230000_f603.csv\",\
                    [field(\"employee\",\"\"),field(\"respondent_id\",\"text\"),field(\"area_id\",\"text\")])\
                    .write_parquet(\"/Volumes/D/youi-app-data/我的数据/jp/000000_20230000_f603.parquet\")";
    test_dsl_execute(script);
}

#[test]
pub fn test_read_parquet(){
    //respondent_id,period_id,area_id,employee
    let script = "read_parquet(\"/Volumes/D/youi-app-data/我的数据/jp/000000_20230000_f603.parquet\")";
    test_dsl_execute(script);
}

#[test]
pub fn test_catalog_table_transform(){
    //respondent_id,period_id,area_id,employee
    let script = "let df_node1 = read_csv(\"/Volumes/D/youi-app-data/我的数据/605.csv\",[field(\"respondent_id\",\"text\"),field(\"area_id\",\"text\"),field(\"period_id\",\"text\"),field(\"C_1\",\"number\"),field(\"C_2\",\"number\"),field(\"C_18\",\"number\"),field(\"C_19\",\"number\"),field(\"C_20\",\"number\"),field(\"C_21\",\"number\"),field(\"C_22\",\"number\"),field(\"C_23\",\"number\"),field(\"C_24\",\"number\"),field(\"C_25\",\"number\"),field(\"C_26\",\"number\"),field(\"C_27\",\"number\"),field(\"C_28\",\"number\"),field(\"C_29\",\"number\")]);\
    let df_node2 = df_node1.stats_fct_transform(\"product_code\",18,\"C_\",[\"respondent_id\",\"period_id\",\"area_id\"],[\"01\",\"02\",\"03\",\"04\"],[\"indicator1\",\"indicator2\",\"indicator3\"]);\
    df_node2";
    test_dsl_execute(script);
}

#[test]
pub fn test_pivot(){
    test_dsl_file("dsl_pivot.txt")
}

fn test_dsl_file(path:&str){
    let path = find_real_path("dsl",path);
    let script = std::fs::read_to_string(path).unwrap();
    test_dsl_execute(&script);
}

fn test_dsl_execute(script:&str){
    println!("{script}");
    let engine = df_engine();

    let result = df_execute(&engine,script,&vec![]).unwrap();

    println!("execute dsl result: {}",result);
}
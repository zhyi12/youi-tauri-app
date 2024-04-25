use log::debug;
use rhai::Engine;
use youi_report::SqlTranslator;

pub fn sql_translate(column_expression:&str){
    youi_test::enable_test_log();
    let engine = Engine::new_raw();
    let result = SqlTranslator::from(&engine).sql_column_expression(column_expression).unwrap();
    debug!("\n    {} \n => {}",column_expression,result);
}

#[test]
pub fn sql_translate_bool() {
    let column_expression = "col(\"period_id\") == col(\"period_id2\")";
    sql_translate(column_expression);
}

///
/// col("colName").start_with("xxx").str_slice("xxx",1,2)
/// substr(start_with(colName,"xxx"),1,2)
///
#[test]
pub fn sql_translate_start_with() {
    let column_expression = "col(\"industry_type\").start_with(\"60\")";
    sql_translate(column_expression);
}

#[test]
pub fn sql_translate_and() {
    let column_expression = "col(\"period_id\") == 20230000 && col(\"area_id\") == \"420100000000\" ";
    sql_translate(column_expression);
}

#[test]
pub fn sql_translate_or() {
    let column_expression = "col(\"period_id\") == 20230000 || col(\"period_id\") == 20210000 ";
    sql_translate(column_expression);
}

#[test]
pub fn sql_translate_and_or() {
    let column_expression = "col(\"period_id\") == 20230000 && (col(\"area_id\") == 42 || col(\"area_id\") == 43)";
    sql_translate(column_expression);
}

#[test]
pub fn sql_translate_nest() {
    let column_expression = "col(\"period_id\") == 20230000 \
         && (col(\"area_id\") == 42 || (col(\"a\") == 1 && col(\"b\") == 2)) \
         && (col(\"industry_id\") == \"43\" || col(\"ct\") == \"2\" ) \
         && col(\"o\") == 9 ";
    sql_translate(column_expression);
}

#[test]
pub fn test_sql_translate1(){
    let column_expression = "(col(\"period_id\") == 20230000 || col(\"area_id\") == \"42\" || col(\"area_id\") == \"41\" || col(\"area_id\") == \"40\") \
         && col(\"name1\").is_in([\"1\",\"2\",\"3\"]) \
         && (col(\"respondent_id\")==1 || col(\"respondent_id\") == 2 || col(\"respondent_id\") == 3 || col(\"respondent_id\") == 4)";
    sql_translate(column_expression);
}
#[test]
pub fn test_sql_translate_simple(){
    sql_translate("col(\"period_id\") == 20230000 && col(\"area_id\") == \"42\"");
}


///
/// when then 生成的列
///
#[test]
pub fn test_sql_translate_case_when_mapping(){
    let column_expression = "when(col(\"report_type\") == 1).then(\"A\")\
                                    .when(col(\"report_type\") == 2).then(\"B\")\
                                    .otherwise(\"C\").alias(\"col_name\")";
    sql_translate(column_expression);
}
///
/// 包含when then的判断
///
#[test]
pub fn test_sql_translate_case_when_condition(){
    let column_expression = "when(col(\"report_type\") == 1).then(\"A\")\
                                    .when(col(\"report_type\") == 2).then(\"B\")\
                                    .otherwise(\"C\")";
    sql_translate(column_expression);
}

#[test]
pub fn test_sql_translate_concat(){
    sql_translate("concat_str([col(\"industry_type\"),\"00\"],\"\")");
}
#[test]
pub fn test_sql_translate_in(){
    sql_translate("col(\"leaf_union_col\").is_in([\"t_jp_2018_611_5_record\",\"t_jp_2018_611_6_record\"])");
}

#[test]
pub fn test_sql_translate_concat_bool(){
    sql_translate("concat_str([col(\"industry_type\"),\"00\"],\"\") == \"6100\"");
}

#[test]
pub fn test_sql_translate_str_slice(){
    sql_translate("col(\"industry_type\").str_slice(0,2)");
}

#[test]
pub fn test_sql_translate_str_trim_slice(){
    sql_translate("col(\"industry_type\").str_trim().str_slice(0,2)");
}

#[test]
pub fn test_sql_translate_str_chain(){
    sql_translate("col(\"industry_type\").str_trim().str_slice(0,2) == \"61\"");
}

#[test]
pub fn test_sql_translate_chain_cond(){
    sql_translate("col(\"industry_type\").str_trim().str_slice(0,2) == \"61\" && col(\"respondent_id\") == \"123456789\"");
}

#[test]
pub fn test_sql_translate_add(){
    sql_translate("(col(\"employee\")+1)/100");
}
use std::borrow::Cow;
use std::time::SystemTime;
use itertools::Itertools;
use polars_lazy::frame::LazyFrame;
use polars_lazy::dsl::{col, Expr, GetOutput};
use polars_core::prelude::{DataType, AnyValue, Series, IntoSeries, PolarsError, ChunkApply, StringChunked};
use youi_search::{FieldValue, Index, Value};


///
/// 用于替换特殊字符  #后续考虑可以作为参数由脚本传递
///
pub const REPLACE_ARRAY: [&str; 9] = [
    "[", "]", "(", ")", "（", "）", ":", "：", "-"
];

///
/// 用于分词的关键字 #后续考虑可以作为参数由脚本传递
///
pub const SEGMENTATION_ARRAY: [&str; 6] = [
    "市", "区", "县", "村", "街道", "路"
];

///
///  匹配入口函数
///
///
pub fn df_match_items(df:LazyFrame,index_df:&LazyFrame,text_fields:&Vec<String>,search_filed:String,out_prefix:String)->LazyFrame{
    let index = youi_search::create_ram_index(text_fields);

    // 索引及相关初始数据
    let mut writer = youi_search::create_writer(&index);
    let exprs:Vec<Expr> = text_fields.iter().map(|name|col(name)).collect();
    let index_df_clone = index_df.clone().select(exprs).collect().unwrap();
    let count = index_df_clone.height();

    // 处理获取需要建索引的df中的数据，进行前置数据类型转换
    // tip：现阶段将值全部转为字符串，后续可根据实际情况进行补充类型建索引
    // 一行一行添加索引
    for i in 0..count{
        let row_vec = index_df_clone.get(i).unwrap();
        let values:Vec<FieldValue> = row_vec.iter().map(|v|{
            match v {
                AnyValue::String(n)=>{
                    FieldValue::Text(n.to_string())
                }
                AnyValue::Int64(u)=>{
                    FieldValue::I64(*u)
                }
                AnyValue::Float32(u)=>{
                    FieldValue::F64(*u as f64)
                }
                AnyValue::Float64(u)=>{
                    FieldValue::F64(*u as f64)
                }
                _=>{
                    FieldValue::Text("".to_string())
                }
            }
        }).collect();
        //写入索引
        youi_search::write_row_data(&index,&mut writer,text_fields,&values);
    }
    //提交索引
    writer.commit().unwrap();

    println!("{}","索引构建完成.");

    let empty_values = text_fields.iter().map(|_|"".to_string()).join(",");
    //执行查询
    let func = move |s: Series| {
        let ca = s.str()?;
        println!("{:?}",ca);
        Ok(Some(match_item(ca,&index,&empty_values)?.into_series()))
    };

    let matched_name = format!("{}_matched",&out_prefix);

    //从df中取得要检索的字段的值 然后触发回调func(该函数实际进行检索),将检索出来的结果字段重命名别名追加到df中，返回数据到调用方
    df.with_columns([col(&format!("{}",search_filed))
         .map(func,GetOutput::from_type(DataType::String)).alias(&matched_name)])
}
///
/// 将检索字段列的所有值进行循环检索
///
pub fn match_item(s:&StringChunked, index:&Index,empty_values:&str) -> Result<StringChunked,PolarsError> {
    Ok(s.apply(|opt_value|{
        let start = SystemTime::now();
        // 处理特殊字符及分词
        let search_str = remake_string(opt_value.unwrap());
        // 将索引字段列表及检索字符串传入进行检索
        let fields:Vec<String> = index.schema().fields().map(|f|String::from(f.1.name())).collect();
        let matched_items = youi_search::search(index,search_str.as_str(),5,&fields);

        if !matched_items.is_empty(){
            //输出field_values
            let result = matched_items[0].doc.field_values().iter().map(|v|
                match &v.value {
                    Value::Str(x) => format!("{}",x),
                    Value::I64(x) => format!("{}",x),
                    Value::F64(x) => format!("{}",x),
                    _ => "".to_string()
                }
            ).join(",");
            println!("耗时：{}毫秒,{} matched {}",SystemTime::now().duration_since(start).unwrap().as_millis(),search_str,result);
            Some(Cow::Owned(result))
        }else{
            Some(Cow::Owned(empty_values.to_string()))
        }
    }))
}

///
/// 功能函数
/// 功能1：将搜索字符串中，不能参与检索的特殊字符进行过滤
/// 功能2：对所搜字符串进行分词
/// tip：特殊字符替换为空字符
///     分词使用空格隔开
///
pub fn remake_string(src:&str) -> String {
    let mut result_str = String::from(src);

    // 特殊字符过滤替换
    for item_rep in REPLACE_ARRAY.iter() {
        result_str = result_str.replace(item_rep, "");
    }

    // 将过滤后的字符串进行分词处理
    for item_seg in SEGMENTATION_ARRAY.iter() {
        result_str = result_str.replace(item_seg,&format!("{} ",item_seg) );
    }
    result_str
}

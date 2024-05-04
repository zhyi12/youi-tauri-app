
extern crate rhai;
extern crate polars_core;
extern crate polars_lazy;
extern crate polars_io;

use rhai::{Engine, EvalAltResult, exported_module};
use youi_dataframe::lazy::JsLazyFrame;
use youi_dataframe::df_to_json;
use itertools::Itertools;
use log::debug;
use polars_core::datatypes::AnyValue;
use serde::{Serialize,Deserialize};
pub use crate::error::DslError;
use crate::param::transform_param;
pub use crate::core::DslTransform;

mod dataframe;

mod param;
mod util;
mod error;
mod core;
mod stats;

///
/// 脚本参数
///
#[derive(Serialize,Deserialize,Debug)]
#[serde(rename_all = "camelCase")]
pub struct Param{
    name:String,
    value:String,
    data_type:String
}

///
/// 通过dsl调用服务
///
pub fn service_eval(engine:&Engine,script:&str)->Result<String, Box<EvalAltResult>>{
    let exec_script = DslTransform::from(script).transform().unwrap();
    engine.eval::<String>(&exec_script)
}
///
/// 脚本执行
///
pub fn dsl_eval(engine:&Engine,script:&str)->Result<JsLazyFrame, Box<EvalAltResult>>{
    let exec_script = DslTransform::from(script).transform().unwrap();
    debug!("exec_script: {}",exec_script);
    engine.eval::<JsLazyFrame>(&exec_script)
}

///
/// 领域脚本执行及返回
///
pub fn default_df_execute(script:&str,params:&Vec<Param>)->Result<String,DslError>{
    let engine = df_engine();
    df_execute(&engine,script,params)
}
///
///
///
pub fn df_execute(engine:&Engine,script:&str,params:&Vec<Param>)->Result<String,DslError>{
    let mut exec_script = script.to_string();
    //参数处理
    if !params.is_empty() {
        exec_script = transform_param(&exec_script,params);
        debug!("transform_param:{}",exec_script);
    }

    //转换为可执行脚本
    exec_script = DslTransform::from(exec_script.as_str()).transform().unwrap();

    debug!("exec_script:{}",exec_script);

    df_eval(&engine,&exec_script)
}
///
/// 立方体查询
///
pub fn df_cube_execute(engine:&Engine,script:&str,group_names:&Vec<String>)->Result<String, DslError>{

    //转换为可执行脚本
    let exec_script =  DslTransform::from(script).transform().unwrap();

    let result = engine.eval::<JsLazyFrame>(&exec_script).unwrap();

    let df = result.df.collect().unwrap();

    let json_str = df_to_json(df.clone());

    let mut series_json_list = Vec::with_capacity(group_names.len());
    for idx in 0..group_names.len(){
        // let name = &group_names[idx];
        let name = String::from(&group_names[idx]);
        let df_series = df.clone().column(&name).unwrap().unique().unwrap().rechunk();

        let s = df_series.iter().map(|v|match v {
            AnyValue::Boolean(x) => {format!("{}",x)}
            AnyValue::String(x) => {format!("\"{}\"",x)}
            AnyValue::UInt8(x) => {format!("{}",x)}
            AnyValue::UInt16(x) => {format!("{}",x)}
            AnyValue::UInt32(x) => {format!("{}",x)}
            AnyValue::UInt64(x) => {format!("{}",x)}
            AnyValue::Int8(x) => {format!("{}",x)}
            AnyValue::Int16(x) => {format!("{}",x)}
            AnyValue::Int32(x) => {format!("{}",x)}
            AnyValue::Int64(x) => {format!("{}",x)}
            AnyValue::Float32(x) => {format!("{}",x)}
            AnyValue::Float64(x) => {format!("{}",x)}
            _=>{String::new()}
        }).join(",");
        series_json_list.push(format!("{{\"name\":\"{}\",\"values\":[{}]}}",name,s));
    }

    Ok(format!("{{\"series\":[{}],\"rowDataList\":{}}}",series_json_list.join(","),json_str))
}

///
/// 分页查询
///
pub fn pager_execute(engine:&Engine,script:&str,page_index:usize,page_size:usize)->Result<String, Box<EvalAltResult>>{

    let exec_script = DslTransform::from(script).transform().unwrap();

    let result = engine.eval::<JsLazyFrame>(&exec_script)?;

    let offset = ((page_index-1).max(0) * page_size) as i64;
    let rdf = result.df.clone().slice(offset,page_size as u32).collect();

    match rdf {
        Ok(df) => {
            let total = result.df.collect().unwrap().height();
            let json_str = df_to_json(df);
            Ok(format!("{{\"total\":{},\"records\":{}}}",total,json_str))
        }
        Err(_) => {
            Ok(format!("{{\"message\":{{\"code\":999999}},\"total\":0,\"records\":[]}}"))
        }
    }
}

///
/// 脚本引擎
///
pub fn df_engine()->Engine{
    let mut engine = Engine::new();

    let module = exported_module!(dataframe::ds_module);
    let stats_module = exported_module!(stats::ds_module);

    engine.set_max_expr_depths(0,0);
    engine.set_max_string_size(0);
    engine.register_global_module(module.into());
    engine.register_global_module(stats_module.into());

    engine
}

///
/// 表格计算引擎
///
pub fn grid_calculate_engine()->Engine{
    let mut engine = Engine::new();
    engine.set_max_expr_depths(0,0);

    engine
}
///
/// 执行脚本
///
fn df_eval(engine:&Engine,exec_script:&str)->Result<String, DslError>{
    let result = engine.eval::<JsLazyFrame>(&exec_script);
    match result {
        Ok(jf)=>{
            let df = jf.df.collect().unwrap();
            let json_str = df_to_json(df);
            Ok(json_str)
        }
        Err(e)=>{
            //异常处理
            Err(error::to_dsl_error(&*e))
        }
    }
}
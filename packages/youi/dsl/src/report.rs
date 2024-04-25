//!
//!
//!

use std::collections::HashMap;
use itertools::Itertools;
use polars_core::prelude::{AnyValue, DataFrame};
use polars_lazy::prelude::{col, concat_list, concat_str, Expr, IntoLazy, lit};
use youi_report::{Cube, DataCube, Dimension, Report, constants::{ROW_GROUP_KEY,DIMENSION_MEASURE, ROW_VALUE_KEY, ROW_MEASURE_KEY, ITEM_KEY_SPLIT, ITEM_GROUP_SUM}, DataValue, DataGrid, Item};
use crate::{df_engine, dsl_eval};
use crate::condition::transform_dsl_condition;

///
/// 报表执行
///
pub fn report_execute(report:&mut Report)->DataGrid{

    //
    //
    let engine = df_engine();

    //println!("{:?}",report);
    //
    // 1、通过dsl方式执行报表数据查询
    // 2、根据立方体及固定的分组项展开表格
    //
    report.dsl_query(|dsl,cube|{
        //执行脚本，返回查询结果
        let result = dsl_eval(&engine,&dsl);
        //
        match result {
            Ok(j) => {
                let df = j.df.collect().unwrap();
                to_data_cube(&df,cube)
            }
            Err(e) => {
                //TODO 异常处理
                println!("error: {:?}",e);
                DataCube::empty()
            }
        }
    },|expr| transform_dsl_condition(expr))
}

///
/// 转带数据的立方体
///
fn to_data_cube(df:&DataFrame,cube:&Cube) -> DataCube{
    //填充分组维度
    let group_dimensions = to_cube_dimensions(df,cube);

    let measure_items = cube.get_measure_items();

    if measure_items.is_empty(){
        return DataCube::empty()
    }
    //排序的分组名集合
    let mut group_names = cube.get_groups().iter().map(|group|group.name.to_string())
        .collect::<Vec<String>>();
    group_names.sort();

    let mut data_map:HashMap<String,DataValue> = HashMap::new();
    //写入数据到data map
    add_cube_df_to_data_map(&df,&mut data_map,&group_names,&measure_items);
    //计算分组小计并写入data map
    add_group_sums_to_data_map(df,&mut data_map,&group_names,&measure_items);
    //输出数据立方体
    DataCube::new(group_dimensions,data_map)
}

///
///
///
fn add_group_sums_to_data_map(df:&DataFrame,data_map:&mut HashMap<String,DataValue>,group_names:&Vec<String>,measure_items:&Vec<Item>){
    //计算 groupSum
    let group_sums:Vec<Vec<&String>> = group_names.iter().powerset().filter(|s|s.len()>0 && s.len() < group_names.len()).collect();

    let measure_exprs = measure_items.iter().map(|item|col(&item.id).sum()).collect::<Vec<Expr>>();

    for idx in 0..group_sums.len(){
        let group_by_exprs = group_sums[idx].iter().map(|name|col(name)).collect::<Vec<Expr>>();
        let mut select_exprs =
            group_names.iter().filter(|name|!group_sums[idx].contains(name))
                .map(|name|lit(ITEM_GROUP_SUM).alias(name)).collect::<Vec<Expr>>();

        select_exprs.push(col("*"));

        let group_sum_df = df.clone().lazy()
            .group_by(&group_by_exprs)
            .agg(&measure_exprs)
            .select(&select_exprs).collect().unwrap();

        add_cube_df_to_data_map(&group_sum_df,data_map,group_names,measure_items);
    }

    //all group [groupSum,groupSum,...,groupSum]
    let mut sum_select_exprs = group_names.iter().map(|name|lit(ITEM_GROUP_SUM).alias(name)).collect::<Vec<Expr>>();
    sum_select_exprs.push(col("*"));
    //
    let sum_df = df.clone().lazy()
        .select(&measure_exprs)
        .sum().unwrap()
        .select(sum_select_exprs).collect().unwrap();

    add_cube_df_to_data_map(&sum_df,data_map,group_names,measure_items);
}

///
/// 立方体查询的dataframe结果存储到data map中
///
fn add_cube_df_to_data_map(df:&DataFrame,data_map:&mut HashMap<String,DataValue>,group_names:&Vec<String>,measure_items:&Vec<Item>){
    // 表达式：分组交叉
    let group_exprs = group_names.iter()
        .map(|name|vec![lit(name.to_string()),col(name)])
        .flatten().collect::<Vec<Expr>>();
    //表达式：计量名数组
    let measure_keys_exprs = measure_items.iter().map(|item|lit(item.id.to_string())).collect::<Vec<Expr>>();
    //表达式：计量值数组
    let measure_value_exprs = measure_items.iter().map(|item|col(&item.id)).collect::<Vec<Expr>>();
    // 输出三列（分组交叉key,计量名数组，计量值数组）
    let value_df = df.clone().lazy()
        .select(vec![concat_str(group_exprs,ITEM_KEY_SPLIT,true).alias(ROW_GROUP_KEY),
                     concat_list(measure_keys_exprs).unwrap().alias(ROW_MEASURE_KEY),
                     concat_list(measure_value_exprs).unwrap().alias(ROW_VALUE_KEY)])
        .explode(vec![col(ROW_MEASURE_KEY),col(ROW_VALUE_KEY)])
        .select(vec![concat_str(vec![col(ROW_GROUP_KEY),lit(DIMENSION_MEASURE),col(ROW_MEASURE_KEY)],ITEM_KEY_SPLIT,true).alias("row_key"),col(ROW_VALUE_KEY)])
        .collect().unwrap();

    let len = value_df.height();

    for idx in 0..len{
        let row = value_df.get_row(idx).unwrap();
        let row_key = &row.0[0];
        match row_key {
            AnyValue::String(u)=>{
                data_map.insert(u.to_string(),DataValue::new(&row.0[1].to_string()));
            }
            _=>{}
        }
    }
}

///
///
///
fn to_cube_dimensions(df:&DataFrame,cube:&Cube)->Vec<Dimension>{
    cube.get_groups().iter().map(|group|{
        let series = df.clone().column(&group.name).unwrap().unique().unwrap().rechunk();

        let ids = series.iter().map(|i|i.to_string()).collect::<Vec<String>>();

        let mut dimension = Dimension::new(&group.id,&group.name);
        //
        dimension.set_id_items(&ids);

        dimension
    }).collect::<Vec<Dimension>>()
}
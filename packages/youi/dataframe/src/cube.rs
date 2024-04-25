//!
//! 立方体计算
//! 1、汇总 groupby().agg()
//! 2、计算小计 groups.powerset()
//! 3、计算合计
//!
use itertools::Itertools;
use polars_core::prelude::{DataType, PolarsResult};
use polars_lazy::prelude::{col, concat, Expr, LazyFrame, lit, UnionArgs};

///
/// 交叉汇总
///
pub fn cube_exec(df:&LazyFrame,measures:&Vec<Expr>,groups:Vec<&str>,measure_columns:Vec<&str>)->PolarsResult<LazyFrame>{
    let group_by = groups.iter().map(|name|col(name).cast(DataType::String)).collect::<Vec<Expr>>();
    //
    let group_df = df.clone().group_by(group_by).agg(measures);

    let mut column_names = groups.clone();
    column_names.append(&mut measure_columns.clone());

    let column_exprs = column_names.iter().map(|name|col(name)).collect::<Vec<Expr>>();

    let measure_exprs = to_measure_exprs(&column_names,&groups);

    let group_sums = groups.iter()
        .powerset()
        .filter(|s|s.len()>0 && s.len() < groups.len()).collect::<Vec<Vec<&&str>>>();

    //分组小计
    let mut dfs = group_sums.iter()
        .map(|group_sum|to_group_sum_df(&group_df,&column_exprs,&measure_exprs,&groups,group_sum))
        .collect::<Vec<LazyFrame>>();
    //合计
    dfs.push(to_sum_df(&group_df,&column_exprs,&measure_exprs,&groups));
    dfs.push(group_df);
    //合并数据
    concat(dfs,UnionArgs::default())
}
///
///
///
fn to_measure_exprs(column_names:&Vec<&str>,groups:&Vec<&str>)->Vec<Expr>{
    column_names.iter()
        .filter(|name|!groups.contains(name))
        .map(|name|col(name).sum())
        .collect::<Vec<Expr>>()
}
///
/// 分组小计
///
fn to_group_sum_df(group_df:&LazyFrame, column_exprs: &Vec<Expr>, measure_exprs:&Vec<Expr>, groups:&Vec<&str>, group_sum:&Vec<&&str>) ->LazyFrame{

    let group_by_exprs = group_sum.iter().map(|name|col(name)).collect::<Vec<Expr>>();

    let mut select_exprs =
        groups.iter().filter(|name|!group_sum.contains(name))
            .map(|name|lit("groupSum").alias(name)).collect::<Vec<Expr>>();

    select_exprs.push(col("*"));
    //
    group_df.clone().group_by(&group_by_exprs)
        .agg(measure_exprs)
        .select(&select_exprs)
        .select(column_exprs)
}
///
/// 合计
///
fn to_sum_df(group_df:&LazyFrame,column_exprs: &Vec<Expr>, measure_exprs:&Vec<Expr>, groups:&Vec<&str>)->LazyFrame{
    let mut sum_select_exprs = groups.iter().map(|name|lit("groupSum").alias(name)).collect::<Vec<Expr>>();
    sum_select_exprs.push(col("*"));
    //
    group_df.clone()
        .select(&measure_exprs)
        .sum().unwrap()
        .select(sum_select_exprs)
        .select(column_exprs)
}
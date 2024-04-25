//!
//! 统计用相关模块
//!
//!
use polars_core::prelude::MeltArgs;
use polars_lazy::dsl::{col, Expr};
use polars_lazy::frame::LazyFrame;
use rhai::plugin::*;

#[export_module]
pub mod ds_module {
    use polars_lazy::prelude::JoinType;
    use polars_lazy::dsl::{col, Expr};
    use rhai::Dynamic;
    use youi_dataframe::lazy::{JsLazyFrame};
    use crate::stats::df_melt_catalog_indicator;

    pub type DF = JsLazyFrame;

    ///
    /// 定长目录表列转行
    /// @param catalog_name 目录列名
    /// @param column_start_num 列起始序号
    /// @param column_name_prefix 列名前缀
    /// @param keys 一维数据列集合 （melt id_vars）
    /// @param items 目录项集合 (melt value_vars)
    /// @param indicators 指标输出列集合
    ///
    /// 产品名称、年初产成品库存量,生产量,销售量
    ///
    ///
    pub fn stats_fct_transform(df:DF,catalog_name:String,column_start_num:i64,column_name_prefix:String,
                               keys:Vec<Dynamic>,items:Vec<Dynamic>,indicators:Vec<Dynamic>
    )->DF{

        let indicator_codes:Vec<String> = indicators.iter().map(|d|d.clone_cast()).collect();
        let item_codes:Vec<String> = items.iter().map(|d|d.clone_cast()).collect();
        let key_columns:Vec<String> = keys.iter().map(|d|d.clone_cast()).collect();

        let indicator_count = indicator_codes.len();

        let key_cols = key_columns.iter().map(|key|col(key)).collect::<Vec<Expr>>();

        let mut join_on = key_cols.clone();
        join_on.extend(vec![col(&catalog_name)]);

        //指标+[目录项]的列转行
        let mut transform_df = df_melt_catalog_indicator(
            &df.df, &key_cols, 0,&catalog_name,&indicator_codes[0],
            &key_columns,&item_codes,column_start_num,&column_name_prefix);

        if indicator_count>1{
            //合并多指标的列转行
            for i in 1..indicator_count{
                let melt_df = df_melt_catalog_indicator(
                    &df.df, &key_cols,i,&catalog_name,&indicator_codes[i],
                    &key_columns,&item_codes,column_start_num,&column_name_prefix);
                transform_df = transform_df.join_builder()
                    .with(melt_df)
                    .left_on(join_on.clone())
                    .right_on(join_on.clone())
                    .how(JoinType::Left)
                    .finish();
            }
        }

        //过滤全部指标都为空的数据

        DF{
            df:transform_df
        }
    }
}

///
/// 指标+[目录项]的列转行
///
fn df_melt_catalog_indicator(df:&LazyFrame,
                             key_cols:&Vec<Expr>,
                             i:usize,
                             catalog_name:&str,
                             indicator_name:&str,
                             key_columns:&Vec<String>,
                             item_codes:&Vec<String>,column_start_num:i64,column_name_prefix:&str)->LazyFrame{
    let item_count = item_codes.len();
    let mut cols = key_cols.clone();
    cols.extend((0..item_count).map(|idx|{
        let column_index = item_count*i+idx+(column_start_num as usize);
        let column_name = format!("{}{}",column_name_prefix,column_index);
        col(&column_name).alias(&item_codes[idx])
    }));
    //选择列并把当前指标和目录项的多列转为行
    df.clone().select(cols)
        .melt(MeltArgs {
            id_vars: key_columns.iter().map(|key|key.into()).collect(),
            value_vars: item_codes.iter().map(|code|code.into()).collect(),
            variable_name: Some(catalog_name.into()),
            value_name: Some(indicator_name.into()),
            streamable: false
        })
}


use itertools::Itertools;
use log::debug;
use crate::item::{Item};
use crate::query::column::ColumnSelect;
use crate::query::cube::CubeExecutor;
use crate::query::dsl::constants::DF_CUBE;

///
///
///
impl<'a> CubeExecutor<'a> {
    ///
    /// 立方体查询
    ///
    pub fn dsl(&self)->String{
        //查询语句
        let mut script = self.query_dataset.dsl(&self.cube_table_names,&self.cube_out_columns,&self.global_filter_script);
        //汇总语句
        script.push_str(&self.dsl_cube());
        //
        script
    }

    ///
    /// [.with_columns([])]  处理列计算（按需生成）
    /// .agg("",[])          汇总
    /// [.with_columns([])]  处理计量计算（按需生成）
    ///
    pub fn dsl_cube(&self)->String{
        //分组列名
        let group_names= self.group_select
            .iter()
            .map(|s|s.select_name())
            .collect::<Vec<String>>();

        //条件分组
        let with_columns = self.group_select
            .iter()
            .filter(|s|s.has_value_mapping()||s.has_column_expression())
            .map(|s|s.dsl_column_mapping()).join(",");

        let with_expr = if with_columns.is_empty() {
            "".to_string()
        }else{
            format!("\n    .with_columns([{}])",with_columns)
        };

        debug!("self.agg_measure_items {:?}",self.agg_measure_items);
        //汇总列
        let agg_expr = self.agg_measure_items.iter()
            .map(|m| match &m.column_expression {
                None=>format!("col(\"{}\").{}().alias(\"{}\")", m.column_name(), m.aggregate(),m.measure_name()),
                Some(column_expression)=>format!("({}).{}().alias(\"{}\")",column_expression,m.aggregate(),m.measure_name()),
            })
            .filter(|name|!name.is_empty())
            .join(",");

        // 汇总后计算
        let cal_expr = self.dsl_measure_calculator();

        format!("{}{}\n    .agg(\"{}\",[{}]){}",DF_CUBE,with_expr,group_names.iter().join(","),agg_expr,cal_expr)
    }
    ///
    /// 汇总后计算表达式
    ///
    fn dsl_measure_calculator(&self)->String{
        let expr_measure_calculator = self.measure_items
            .iter()
            .map(|item|
                match item {
                    Item::Measure(m)=>match &m.expression {
                        None=>"".to_string(),
                        Some(expression)=>format!("({}).alias(\"{}\")",expression,m.measure_name())
                    }
                    _=>"".to_string()
                }
            )
            .filter(|s|!s.is_empty())
            .join(",");

        debug!("expr_measure_calculator {expr_measure_calculator}");
        //汇总后的with_columns
        if expr_measure_calculator.is_empty(){
            "".to_string()
        }else{
            format!("\n    .with_columns([{}])",expr_measure_calculator)
        }
    }
}

///
///
///
impl ColumnSelect {

    ///
    /// 生成列映射dsl脚本
    ///
    pub fn dsl_column_mapping(&self)->String{
        if self.has_column_expression() {
            //列表达式
            format!("{}.alias(\"{}\")",self.column_expression().unwrap(),self.select_name())
        }else{
            //条件分组项
            let mut script =  self.value_mapping()
                .iter()
                .map(|(id,expression)|{
                    format!("when({}).then(\"{}\")",expression,id)
                }).join(".");

            script.push_str(&format!(".otherwise(\"_999\").alias(\"{}\")",self.id()));
            script
        }
    }
}
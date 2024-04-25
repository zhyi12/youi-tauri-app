use itertools::Itertools;
use crate::{Dimension, DimensionType, Item};
use crate::constants::ITEM_GROUP_SUM;

///
///
///
pub fn build_dimension_column_expression<F>(dimension: &Dimension,expression_processor:&mut F) -> String
    where F:FnMut(&str)->String{
    match dimension.dimension_type {
        DimensionType::Condition|DimensionType::Tree|DimensionType::Segment =>
            format!("{}.alias(\"{}\")",build_column_expression_by_items(dimension,expression_processor),dimension.name),
        _=>format!("col(\"{}\")",dimension.name)
    }
}

///
/// 生成列取值表达式
///
fn build_column_expression_by_items<F>(dimension: &Dimension,expression_processor:&mut F)->String
    where F:FnMut(&str)->String{
    let when_then_exprs = dimension.items.as_ref().unwrap()
        .iter()
        .filter(|item|&item.id != ITEM_GROUP_SUM)
        .map(|item|build_when_then_expression(item,&dimension.name,expression_processor)).join(".");
    format!("{}.otherwise(lit(\"__oth\"))",when_then_exprs)
}

///
///
///
fn build_when_then_expression<F>(item:&Item,dimension_name:&str,expression_processor:&mut F)->String
    where F:FnMut(&str)->String{
    let expr = match &item.expression {
        None => format!("col(\"{}\").eq(lit({}))",dimension_name,item.id),
        //条件表达式处理
        Some(expression) => expression_processor(expression)
    };
    format!("when({}).then(lit(\"{}\"))",expr,item.id)
}
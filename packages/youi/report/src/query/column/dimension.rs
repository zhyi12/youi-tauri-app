use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use crate::ColumnSelect;
use crate::dimension::{Dimension, IGroup};
use crate::item::Item;

///
/// 分组维度
///
impl From<&Dimension> for ColumnSelect{

    ///
    ///
    ///
    fn from(group_dimension: &Dimension) -> Self {
        let value_mapping = match group_dimension {
            Dimension::Group(g) => parse_value_mapping(g.column_name(),g.items()),
            Dimension::Period(g) => parse_value_mapping(g.column_name(),g.items()),
            Dimension::Area(g) => parse_value_mapping(g.column_name(),g.items()),
            _=>vec![],
        };

        Self{
            id: format!("mg_{}",group_dimension.find_id()),
            name: group_dimension.find_group_column_name(),
            column_expression: group_dimension.find_column_expression(),
            measure_expression:None,
            value_mapping
        }
    }
}

///
/// <值、表达式>
///
fn parse_value_mapping(column_name:&str,items:&Vec<Item>)->Vec<(String,String)>{
    //是否包含列计算表达式
    let has_column_expression = items.iter().find(|item|match item {
        Item::Group(group_item) => match group_item.column_expression() {
            None => false,
            Some(expression) => !expression.is_empty()
        }
        _ => false,
    }).is_some();

    if has_column_expression{
        //TODO 树结构的条件分组项处理
        items.into_par_iter()
            .filter(|item|match item {
                Item::Group(_) => true,
                _=>false
            })
            .map(|item|{
                match item {
                    Item::Group(group_item) => {
                        let id = group_item.id_clone();
                        let expression = match group_item.column_expression() {
                            None => format!("col(\"{}\").eq(\"{}\")",column_name,&id),
                            Some(expression) => expression.to_string()
                        };
                        vec![(id,expression)]
                    },
                    _=>vec![]
                }
            }).flatten()
            .collect::<Vec<(String,String)>>()
    }else{
        vec![]
    }
}
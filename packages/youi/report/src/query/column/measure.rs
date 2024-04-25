use crate::item::MeasureItem;
use crate::query::column::ColumnSelect;

///
/// 指标项生成列选择
///
impl From<&MeasureItem> for ColumnSelect{
    ///
    ///
    ///
    fn from(measure_item: &MeasureItem) -> Self {
        Self{
            id:format!("{}_{}", measure_item.column.column_name, measure_item.aggregate),
            name: measure_item.column.column_name.to_string(),
            column_expression: measure_item.column_expression.clone(),
            measure_expression:measure_item.expression.clone(),
            value_mapping:vec![]
        }
    }

}
use serde::{Serialize, Deserialize};
use std::fmt::Write;
use itertools::Itertools;
use crate::dsl::DslNode;
use crate::sort::Order;
use crate::step::StepInfo;

///
/// 排序
///
#[derive(Serialize, Deserialize,Debug)]
pub struct Sort{
    ///
    ///
    ///
    #[serde(flatten)]
    pub(crate) info:StepInfo,
    ///
    /// 计算列集合
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    orders:Option<Vec<Order>>

}

///
///
///
impl DslNode for Sort {

    ///
    ///
    ///
    fn to_dsl(&self,input_step_id:Option<&str>) -> crate::error::QueryResult<String> {
        // 检查输入
        self.check_input(input_step_id,self.info.text.as_ref().unwrap_or(&"Sort".to_string()))?;
        let input_id = input_step_id.unwrap();

        Ok(match &self.orders {
            None => "".to_string(),
            Some(orders) => {
                let mut orders_expr = orders.iter().fold(String::new(), |mut output, b| {
                    let _ = write!(output, "col(\"{}\"),",b.property);
                    output
                });
                orders_expr.pop();

                format!("let {} = {}.sort_by_exprs([{}],\"{}\")",self.build_df_id(&self.info.id),self.build_df_id(input_id),
                        orders_expr,
                        orders.iter().map(|o|match o.descending {
                            None=>false,
                            Some(descending)=>descending
                        }).join(",")
                )
            }
        })
    }
}
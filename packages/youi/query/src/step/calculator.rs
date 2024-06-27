use serde::{Serialize, Deserialize};

use crate::column::Column;
use crate::dsl::DslNode;
use crate::step::StepInfo;

///
/// 列计算
///
#[derive(Serialize, Deserialize,Debug)]
pub struct Calculator{
    ///
    ///
    ///
    #[serde(flatten)]
    pub(crate) info:StepInfo,
    ///
    /// 计算列集合
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    calculators:Option<Vec<Column>>

}

///
///
///
impl DslNode for Calculator {

    ///
    ///
    ///
    fn to_dsl(&self,input_step_id:Option<&str>) -> crate::error::QueryResult<String> {
        Ok(match &self.calculators {
            None => "".to_string(),
            Some(calculators) => {
                if calculators.is_empty(){
                    "".to_string()
                }else{
                    // 检查输入
                    self.check_input(input_step_id,self.info.text.as_ref().unwrap_or(&"Calculator".to_string()))?;
                    let input_id = input_step_id.unwrap();

                    let selected_columns = self.calculators.as_ref()
                        .unwrap()
                        .iter()
                        .collect::<Vec<&Column>>();

                    let columns_expr = self.to_columns_dsl(selected_columns)?;

                    format!("let {} = {}.with_columns([{}])",self.build_df_id(&self.info.id),self.build_df_id(input_id),columns_expr)
                }
            }
        })
    }
}
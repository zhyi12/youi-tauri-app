use itertools::Itertools;
use serde::{Serialize, Deserialize};
use std::fmt::Write;
use crate::column::{Columns};
use crate::dsl::DslNode;
use crate::error::QueryError;
use crate::step::StepInfo;

#[derive(Serialize, Deserialize,Debug)]
#[serde(rename_all = "camelCase")]
pub struct Select{
    ///
    ///
    ///
    #[serde(flatten)]
    pub(crate) info:StepInfo,

    #[serde(flatten)]
    columns_info:Columns
}

///
///
///
impl DslNode for Select {

    ///
    ///
    ///
    fn to_dsl(&self,input_step_id:Option<&str>) -> crate::error::QueryResult<String> {
        // 检查输入
        self.check_input(input_step_id,self.info.text.as_ref().unwrap_or(&"Select".to_string()))?;
        let input_id = input_step_id.unwrap();

        let columns_expr = self.to_columns_dsl(self.columns_info.find_selected_columns())?;

        Ok(format!("let {} = {}.with_columns([{}])",self.build_df_id(&self.info.id),self.build_df_id(input_id),columns_expr))
    }
}
use serde::{Serialize, Deserialize};
use crate::dsl::DslNode;
use crate::error::QueryError;
use crate::filter::{DslFilterTreeVisitor, Filter as FilterInfo};
use crate::step::{IStep, StepInfo};

///
///
///
#[derive(Serialize, Deserialize,Debug)]
pub struct Filter{
    ///
    ///
    ///
    #[serde(flatten)]
    pub(crate) info:StepInfo,

    ///
    ///
    ///
    #[serde(flatten)]
    filter:FilterInfo,

}

///
///
///
impl DslNode for Filter {

    ///
    ///
    ///
    fn to_dsl(&self,input_step_id:Option<&str>) -> crate::error::QueryResult<String> {
        if self.filter.items.len()>0{
            // 检查输入
            self.check_input(input_step_id,self.info.text.as_ref().unwrap_or(&"Filter".to_string()))?;

            let input_df = input_step_id.unwrap();
            let tree = self.filter.to_tree();

            let mut visitor = DslFilterTreeVisitor::new();

            tree.visit(&mut visitor);
            // 数据过滤
            Ok(format!("let {} = {}.filter({})",self.build_df_id(&self.info.id),self.build_df_id(input_df),visitor.dsl))
        }else{
            Ok("".to_string())
        }
    }
}
use crate::error::ReportResult as Result;
use rhai::Engine;
use crate::query::sql::translate::{column::SqlExpressionBuilder,tree::CondTreeBuilder};

pub const COND :&str = "cond";
pub const COND_OR: &str = "or";
pub const COND_AND: &str = "and";

///
///
///
pub struct SqlTranslator<'a>{

    engine:&'a Engine,

}

impl<'a> From<&'a Engine> for SqlTranslator<'a>{

    fn from(engine: &'a Engine) -> Self {
        Self{
            engine
        }
    }
}

impl<'a> SqlTranslator<'a> {
    ///
    /// 表达式转换为sql
    ///
    pub fn sql_column_expression(&self,column_expression:&str)->Result<String>{
        //中间表达式
        let conn_tree_expression = SqlExpressionBuilder::from(self.engine)
            .expression(column_expression)
            .build()?;

        if conn_tree_expression.is_empty(){
            Ok("".to_string())
        }else{
            //转sql
            CondTreeBuilder::from(self.engine)
                .cond_expression(&conn_tree_expression)
                .build()
        }

    }

}


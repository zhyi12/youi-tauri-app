

use rhai::{FnCallExpr};
use crate::error::ReportResult as Result;
use crate::query::sql::translate::column::SqlExpressionBuilder;

///
/// == != <> > >= < <=
///
pub struct CompileBuilder<'a,'b>{
    builder:&'a mut SqlExpressionBuilder<'b>,
}

impl<'a,'b> From<&'a mut SqlExpressionBuilder<'b>> for CompileBuilder<'a,'b>{

    fn from(builder: &'a mut SqlExpressionBuilder<'b>) -> Self {
        Self{
            builder
        }
    }
}

impl<'a,'b> CompileBuilder<'a,'b>{

    ///
    ///
    ///
    pub fn build(&mut self,fn_expr:&FnCallExpr,prev_conn:&str)->Result<()>{
        let is_start = self.builder.writer.len() == 0;
        let mut conn = prev_conn;

        let name = &(fn_expr.name.to_string());

        //连接符号
        let symbol = if name == "==" { "=" } else { name };

        if is_start{
            self.builder.write_str("\"")?;
            //只有一个等式的时候，需要包装为字符串，同时设置conn为start
            conn = "start";
        }
        self.builder.writer_left_right(&fn_expr.args[0],&fn_expr.args[1],symbol,conn)?;
        if is_start{
            self.builder.write_str("\"")?;
        }
        Ok(())
    }
}
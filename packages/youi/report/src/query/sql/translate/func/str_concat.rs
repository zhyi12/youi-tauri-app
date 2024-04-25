use log::debug;
use rhai::{Expr, FnCallExpr};
use crate::error::ReportResult as Result;
use crate::query::sql::translate::column::SqlExpressionBuilder;

///
/// 字符连接函数
/// concat([col(\"name\"),"00"],",")
/// => concat(",",name,"00")
///
pub struct StrConcatBuilder<'a,'b>{
    builder:&'a mut SqlExpressionBuilder<'b>,
}

impl<'a,'b> From<&'a mut SqlExpressionBuilder<'b>> for StrConcatBuilder<'a,'b>{

    fn from(builder: &'a mut SqlExpressionBuilder<'b>) -> Self {
        Self{
            builder
        }
    }
}

impl<'a,'b>  StrConcatBuilder<'a,'b>{

    ///
    ///
    ///
    pub fn build(&mut self,fn_expr:&FnCallExpr,_rhs:Option<&Expr>,_prev_conn:&str)->Result<()>{
        self.builder.write_str("concat(")?;

        let arg_values = &fn_expr.args[0];
        let arg_sep =  &fn_expr.args[1];

        self.builder.write_expr(arg_sep,"")?;
        self.builder.write_str(",")?;

        //
        match arg_values{
            Expr::DynamicConstant(x,..)=>{
                debug!("arg_values {:?}",x);
            }
            Expr::Array(x,..)=>{
                let count = x.len();
                for idx in 0..count{
                    if idx>0{
                        self.builder.write_str(",")?;
                    }
                    self.builder.write_expr(&x[idx],"")?;
                }
            }
            _=>{
                debug!("other args {:?}",arg_values);
            }
        }
        self.builder.write_str(")")?;

        Ok(())
    }
}



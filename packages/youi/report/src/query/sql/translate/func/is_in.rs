use log::debug;
use rhai::{Dynamic, Expr, FnCallExpr};
use crate::error::ReportResult as Result;
use crate::query::sql::translate::column::SqlExpressionBuilder;

pub struct IsInBuilder<'a,'b>{
    builder:&'a mut SqlExpressionBuilder<'b>,
}

impl<'a,'b> From<&'a mut SqlExpressionBuilder<'b>> for IsInBuilder<'a,'b>{

    fn from(builder: &'a mut SqlExpressionBuilder<'b>) -> Self {
        Self{
            builder
        }
    }
}

impl<'a,'b> IsInBuilder<'a,'b> {

    pub fn build(&mut self,fn_expr:&FnCallExpr,_rhs:Option<&Expr>,prev_conn:&str)->Result<()>{
        //删除.
        debug!("in {:?}",fn_expr);
        self.builder.write_str(" in(")?;
        //数组
        match &fn_expr.args[0] {
            Expr::Array(x,_)=>{
                for arr in x.as_ref(){
                    self.builder.write_expr(arr,prev_conn)?;
                }
            },
            Expr::DynamicConstant(x,_)=>{
                let dynamic:&Dynamic = x.as_ref();
                if dynamic.is_array(){
                    let mut arr_str = dynamic.to_string().replace("\"","'");
                    // 删除中括号
                    arr_str.pop();
                    if arr_str.len()>0{
                        arr_str.remove(0);
                    }
                    self.builder.write_str(&arr_str)?;
                }
            },
            _=>{
                debug!("other array");
            }
        }
        self.builder.write_str(")")?;

        Ok(())
    }
}
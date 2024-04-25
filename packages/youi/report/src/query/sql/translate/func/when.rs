use log::debug;
use rhai::{Expr, FnCallExpr};
use crate::error::ReportResult as Result;
use crate::query::sql::translate::column::SqlExpressionBuilder;

pub struct WhenBuilder<'a,'b>{
    builder:&'a mut SqlExpressionBuilder<'b>,
}

impl<'a,'b> From<&'a mut SqlExpressionBuilder<'b>> for WhenBuilder<'a,'b>{

    fn from(builder: &'a mut SqlExpressionBuilder<'b>) -> Self {
        Self{
            builder
        }
    }
}

impl<'a,'b> WhenBuilder<'a,'b> {

    ///
    /// when().then().when().then().otherwise()
    ///
    pub fn build(&mut self,expr:&Expr,rhs:Option<&Expr>,prev_conn:&str)->Result<()>{
        match rhs {
            None => {}
            Some(e) => {
                if prev_conn.is_empty(){
                    self.builder.write_str("\"")?;
                }
                self.write_when_expr(expr,e)?;
                if prev_conn.is_empty(){
                    self.builder.write_str("\"")?;
                }
            }
        }
        Ok(())
    }

    ///
    /// when then otherwise 处理
    ///
    fn write_when_expr(&mut self,fn_expr:&Expr,rhs:&Expr) -> Result<()>{
        // case when name = 'aaa' and addr ='aaa_addr' then 1 else 0 end
        //when args 处理
        match fn_expr{
            Expr::FnCall(x,..)=>{
                self.builder.write_str("case when ")?;
                self.builder.write_expr(&x.args[0],"")?;
                //后接then处理
                self.write_chain_then(rhs)?;
            },
            _=>{}
        }
        Ok(())
    }

    ///
    ///
    ///
    fn write_then_next_expr(&mut self,expr:&Expr)->Result<()>{
        match expr {
            Expr::Dot(x,..)=>{
                let _is_end = match &x.lhs {
                    Expr::MethodCall(mc,..)|Expr::FnCall(mc,..)=>{
                        let func_name = &mc.name.to_string();
                        if "when" == func_name {
                            self.write_chain_when(&mc,&x.rhs)?;
                        }else if "otherwise" == func_name{
                            self.write_otherwise(&mc)?;
                            //alias
                            match &x.rhs {
                                Expr::MethodCall(alias_func_expr,..)|Expr::FnCall(alias_func_expr,..)=>{
                                    let alias_name = &alias_func_expr.name.to_string();
                                    if alias_name == "alias"{
                                        self.builder.write_str(" as ")?;
                                        match  &alias_func_expr.args[0]{
                                            Expr::StringConstant(s,..)=>self.builder.write_str(&s.to_string())?,
                                            _=>{}
                                        }
                                    }
                                },
                                _=>{}
                            };
                        }
                    },
                    _=>{}
                };
            },
            Expr::MethodCall(mc,..)=>{
                let func_name = &mc.name.to_string();
                if "otherwise" == func_name{
                    self.write_otherwise(&mc)?;
                }
            }
            _=>{
                debug!("other then next {:?}",expr);
            }
        }
        Ok(())
    }

    ///
    /// 输出otherwise
    ///
    fn write_otherwise(&mut self,otherwise_expr:&FnCallExpr)->Result<()>{
        self.builder.write_str(" else ")?;
        self.builder.write_expr(&otherwise_expr.args[0],"")?;
        //结束case when
        self.builder.write_str(" end")?;
        Ok(())
    }

    ///
    /// chain when
    ///
    fn write_chain_when(&mut self,then_expr:&FnCallExpr,rhs:&Expr)->Result<()>{
        self.builder.write_str(" when ")?;
        self.builder.write_expr(&then_expr.args[0],"")?;
        self.write_chain_then(rhs)?;
        Ok(())
    }

    ///
    /// when 后的 then
    ///
    fn write_chain_then(&mut self,expr:&Expr)->Result<()>{
        match expr{
            Expr::Dot(dot,..)=>{
                match &dot.lhs {
                    Expr::MethodCall(mc,..)|Expr::FnCall(mc,..)=>{
                        let func_name = &mc.name.to_string();
                        if "then" == func_name{
                            self.builder.write_str(" then ")?;
                            self.builder.write_expr(&mc.args[0],"")?;
                            //then后面可以是chain when 或者otherwise
                            self.write_then_next_expr(&dot.rhs)?;
                        }
                    }
                    _=>{
                        debug!("chain then other:{:?}",&dot.rhs);
                    }
                }
            },
            _=>{}
        }
        Ok(())
    }

}
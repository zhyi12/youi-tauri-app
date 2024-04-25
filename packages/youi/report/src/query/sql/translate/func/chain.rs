use log::debug;
use rhai::Expr;
use crate::error::ReportResult as Result;
use crate::query::sql::translate::column::SqlExpressionBuilder;

#[derive(Debug)]
pub enum ChainFunc{
    ///
    /// trim
    ///
    StrTrim,
    ///
    /// slice
    ///
    StrSlice,
    ///
    ///
    ///
    StrPad,
    ///
    ///
    ///
    StrLength,
    ///
    /// starts_with
    ///
    StartWith,

    None
}

impl From<&str> for ChainFunc {
    fn from(value: &str) -> Self {
        match value {
            "str_slice"=>ChainFunc::StrSlice,
            "str_pad"=>ChainFunc::StrPad,
            "str_trim"=>ChainFunc::StrTrim,
            "str_length"=>ChainFunc::StrLength,
            "start_with"=>ChainFunc::StartWith,
            _=>ChainFunc::None
        }
    }
}

impl ChainFunc {
    pub fn name(&self)->&str{
        match self {
            ChainFunc::StrSlice=>"substr",
            ChainFunc::StrPad=>"pad",
            ChainFunc::StrTrim=>"trim",
            ChainFunc::StrLength=>"length",
            ChainFunc::StartWith=>"starts_with",
            _ => ""
        }
    }
}

///
///
/// col(\"industry_type\").str_trim().str_slice(0,2)
/// => substring(trim(industry_type),0,2)
///
/// col(\"industry_type\").str_pad(4,"0").str_slice(0,2)
/// => str_slice(pad(industry_type,4,"0"),0,2)
///
pub struct ChainBuilder<'a,'b>{
    builder:&'a mut SqlExpressionBuilder<'b>,
}

impl<'a,'b> From<&'a mut SqlExpressionBuilder<'b>> for ChainBuilder<'a,'b>{

    fn from(builder: &'a mut SqlExpressionBuilder<'b>) -> Self {
        Self{
            builder
        }
    }
}

impl<'a,'b> ChainBuilder<'a,'b> {

    ///
    ///
    ///
    pub fn build(&mut self,expr:&Expr,rhs:Option<&Expr>,prev_conn:&str)->Result<()>{
        match rhs {
            None => {}
            Some(e) => {
                let chain_func_list = parse_chain_func_list(e);
                debug!("chain_func_list {:?}",chain_func_list);
                let count = chain_func_list.len();

                for idx in 0..count{
                    let (chain_func,chain) = &chain_func_list[count-idx-1];
                    match chain_func {
                        Expr::MethodCall(..)=>{
                            self.builder.write_str(chain.name())?;
                            self.builder.write_str("(")?;
                        }
                        _=>{}
                    }
                }
                self.builder.write_expr(expr,"")?;

                for idx in 0..count {
                    let (chain_func,_) = &chain_func_list[idx];
                    match chain_func {
                        Expr::MethodCall(mc,..)|Expr::FnCall(mc,..)=>{
                            //参数
                            for arg_idx in 0..mc.args.len(){
                                self.builder.write_str(",")?;
                                self.builder.write_expr(&mc.args[arg_idx],"")?;
                            }
                            self.builder.write_str(")")?;
                        }
                        _=>{
                            debug!("other chain");
                        }
                    }
                }

                debug!("prev_conn {prev_conn},{}",self.builder.writer)
            }
        }
        Ok(())
    }

}

///
///
///
fn parse_chain_func_list(expr: &Expr) ->Vec<(&Expr,ChainFunc)> {
    let mut func_list:Vec<(&Expr,ChainFunc)> = vec![];
    match expr {
        Expr::Dot(x,..)=>{
            match &x.lhs {
                Expr::MethodCall(mc,..)=>{
                    let name:&str = &(mc.name.to_string());
                    let chain = ChainFunc::from(name);
                    match chain{
                        ChainFunc::None => {},
                        _=>{
                            func_list.push((&x.lhs,chain));
                            func_list.extend(parse_chain_func_list(&x.rhs));
                        }
                    }
                }
                _=>{
                    debug!("other {:?}",&x.lhs);
                }
            }
        },
        Expr::MethodCall(mc,..)=>{
            let name:&str = &(mc.name.to_string());
            let chain = ChainFunc::from(name);
            match chain {
                ChainFunc::None => {},
                _=>func_list.push((expr,chain))
            }
        }
        _=>{
            debug!("other {:?}",expr);
        }
    };

    func_list
}



use log::debug;
use rhai::{Expr, FnCallExpr};
use crate::error::ReportResult as Result;
use crate::query::sql::translate::column::SqlExpressionBuilder;

pub enum Operators{
    Add,
    Sub,
    Mul,
    Div,
    None
}

impl From<&str> for Operators {
    fn from(value: &str) -> Self {
        match value {
            "+"=>Self::Add,
            "-"=>Self::Sub,
            "*"=>Self::Mul,
            "/"=>Self::Div,
            _=>Self::None
        }
    }
}

impl Operators {
    ///
    /// 是否乘除法
    ///
    pub fn is_md(&self)->bool{
        match self {
            Self::Mul| Self::Div =>true,
            _=>false
        }
    }
}
///
/// + - * /
///
pub struct OperatorsBuilder<'a,'b>{
    builder:&'a mut SqlExpressionBuilder<'b>,
}

impl<'a,'b> From<&'a mut SqlExpressionBuilder<'b>> for OperatorsBuilder<'a,'b>{

    fn from(builder: &'a mut SqlExpressionBuilder<'b>) -> Self {
        Self{
            builder
        }
    }
}

impl<'a,'b> OperatorsBuilder<'a,'b> {
    ///
    ///
    ///
    pub fn build(&mut self, fn_expr: &FnCallExpr, _prev_conn: &str) -> Result<()> {
        let name = fn_expr.name.to_string();

        let operator = Operators::from(name.as_str());
        // 乘除法非简单部分需要添加括号
        let is_md = operator.is_md();

        self.write_part(&fn_expr.args[0],is_md)?;
        self.builder.write_str(" ")?;
        self.builder.write_str(&name)?;
        self.builder.write_str(" ")?;
        self.write_part(&fn_expr.args[1],is_md)?;

        Ok(())
    }

    ///
    ///
    ///
    fn write_part(&mut self,part:&Expr,is_md:bool)->Result<()>{

        let need_bracket =  is_md && match part {
            Expr::Dot(_,..)=>true,
            Expr::FnCall(_,..)=>true,
            _=>{
                debug!("part is {:?}",part);
                false
            }
        };

        if need_bracket{
            self.builder.write_str("(")?;
        }

        self.builder.write_expr(part,"")?;

        if need_bracket{
            self.builder.write_str(")")?;
        }

        Ok(())
    }
}
//!
//!
//!

use log::debug;
use regex::Regex;
use crate::error::ReportResult as Result;
use rhai::{AST, Engine, Expr, FnCallExpr, Stmt};
use crate::query::sql::translate::func::{ChainBuilder, ChainFunc, IsInBuilder, StrConcatBuilder, WhenBuilder,CompileBuilder,OperatorsBuilder};
use crate::query::sql::translate::translator::{COND_AND,COND_OR};


///
/// sql条件语句
///
pub struct SqlExpressionBuilder<'a>{
    pub(crate) writer:String,
    expression:String,
    engine: &'a Engine,
}

impl<'a> From<&'a Engine> for SqlExpressionBuilder<'a> {
    fn from(engine: &'a Engine) -> Self {
        Self{
            engine,
            expression:"".to_string(),
            writer:String::new()
        }
    }
}

impl<'a> SqlExpressionBuilder<'a> {
    ///
    ///
    ///
    pub fn expression(mut self,expression:&str)->Self{
        let reg = Regex::new("\\(\\s+").unwrap();
        self.expression = reg.replace_all(expression,"(").to_string();
        self
    }
    ///
    ///
    ///
    pub fn build(&mut self)->Result<String>{
        self.writer = String::new();
        let ast:AST = self.engine.compile_expression(&self.expression)?;

        debug!("ast {:?}",ast);

        let statements = ast.statements();
        for stmt in statements {
            self.write_stmt(stmt)?;
        }

        //单个表达式转字符串 ，非 (" 和 " 开始的为单个表达式
        if !self.writer.starts_with("\"") && !self.writer.starts_with("(\""){
            self.writer.insert_str(0,"\"");
            self.writer.push_str("\"");
        } else if self.writer.starts_with("("){
            //使用cond包装第一个(号内的表达式组
            self.writer.insert_str(0,"cond");
        }

        Ok(self.writer.to_string())
    }
    ///
    ///
    ///
    fn write_stmt(&mut self, stmt: &Stmt) ->Result<()>{
        match stmt {
            Stmt::Expr(x) => {
                self.write_expr(x,"")?;
            }
            Stmt::FnCall(x,..)=>{
                self.write_func(&x,"")?;
            }
            _=>{
                debug!("other stmt {:?}",stmt);
            }
        }
        Ok(())
    }

    ///
    /// 写字符串
    ///
    pub(crate) fn write_str(&mut self,value:&str)->Result<()>{
        self.writer.push_str(value);
        Ok(())
    }

    ///
    /// 函数表达式判断
    ///
    fn is_func(expr:&Expr,name:&str)->bool{
        match expr {
            Expr::FnCall(x,..)=>{
                &x.name.to_string() == name
            },
            _=>false
        }
    }

    ///
    /// 链式函数判断
    ///
    fn is_chain_func(lhs:&Expr, rhs:&Expr) ->bool{
        if Self::is_func(lhs,"col"){
            match rhs {
                Expr::MethodCall(mc,..)|Expr::FnCall(mc,..)=>{
                    let name:&str = &(mc.name.to_string());
                    match ChainFunc::from(name){
                        ChainFunc::None=>false,
                        _=>true
                    }
                },
                Expr::Dot(dot,..)=>Self::is_chain_func(lhs,&dot.rhs),
                _=>false
            }
        }else {
            false
        }
    }
    ///
    ///
    ///
    pub(crate) fn write_expr(&mut self, expr: &Expr,prev_conn:&str) ->Result<()>{
        match expr {
            Expr::StringConstant(x,_)=>self.write_str(&format!("'{}'",x))?,
            Expr::IntegerConstant(x,_)=>self.write_str(&x.to_string())?,
            Expr::FloatConstant(x,_)=>self.write_str(&x.to_string())?,
            Expr::MethodCall(x, _) | Expr::FnCall(x, _)=> {
                self.write_func(x,prev_conn)?;
            }
            Expr::Dot(x, _, _) => {
                if Self::is_func(&x.lhs,"when"){
                    //when处理
                    WhenBuilder::from(self).build(&x.lhs,Some(&x.rhs),prev_conn)?;
                }else if Self::is_chain_func(&x.lhs, &x.rhs){
                    //链式函数处理
                    ChainBuilder::from(self).build(&x.lhs,Some(&x.rhs),prev_conn)?;
                }else{
                    self.write_expr(&x.lhs,prev_conn)?;
                    self.writer.push_str(".");
                    self.write_expr(&x.rhs,prev_conn)?;
                }
            }
            Expr::And(x, _) => {
                self.writer_connect(&x.lhs,&x.rhs,COND_AND)?
            },
            Expr::Or(x, _) => {
                self.writer_connect(&x.lhs,&x.rhs,COND_OR)?;
            },
            _=>{
                debug!("other {:?}",expr);
            }
        }
        Ok(())
    }

    ///
    ///
    ///
    pub fn writer_left_right(&mut self, lhs: &Expr,rhs:&Expr,symbol:&str,prev_conn:&str) ->Result<()>{
        self.write_expr(lhs,prev_conn)?;
        self.writer.push_str(" ");
        self.writer.push_str(symbol);
        self.writer.push_str(" ");
        self.write_expr(rhs,prev_conn)?;
        Ok(())
    }

    ///
    /// and or 连接
    ///
    fn writer_connect(&mut self, lhs: &Expr,rhs:&Expr,conn:&str) ->Result<()>{
        //左边条件为and 或者 or时
        let mut wrap = false;
        if conn == COND_AND {
            let position = match lhs {
                Expr::Or(x,..) => parse_lhs_position(&x.lhs),
                _=>0
            };
            if position>1{
                let start = &self.expression[position-2..position-1];
                wrap = start == "(";
            }
        }

        if wrap{
            self.write_str("(")?;
        }

        self.write_sql_condition(lhs,conn)?;

        if wrap{
            self.write_str(")")?;
        }

        self.write_str(&format!(".{}(",conn))?;
        self.write_sql_condition(rhs,conn)?;
        self.write_str(")")?;

        Ok(())
    }

    ///
    ///
    ///
    fn write_sql_condition(&mut self,cond_expr: &Expr,prev_conn:&str)->Result<()>{
        let quote = match cond_expr {
            Expr::And(_, _) | Expr::Or(_, _)=> "",
            _=>"\""
        };
        self.write_str(quote)?;
        self.write_expr(cond_expr,prev_conn)?;
        self.write_str(quote)?;
        Ok(())
    }

    ///
    /// 函数
    ///
    fn write_func(&mut self,fn_expr:&FnCallExpr,prev_conn:&str) ->Result<()>{
        let func = ColumnFunc::from(fn_expr.name.to_string().as_str());
        match func {
            ColumnFunc::Col => {
                let column_name = match &fn_expr.args[0] {
                    Expr::StringConstant(s, _) => s.to_string(),
                    _=>"".to_string()
                };
                self.writer.push_str(&column_name);
            }
            ColumnFunc::Eq | ColumnFunc::Neq | ColumnFunc::Lt | ColumnFunc::Lte | ColumnFunc::Gt | ColumnFunc::Gte=> {
                // 比较函数
                CompileBuilder::from(self).build(fn_expr,prev_conn)?;
            }
            ColumnFunc::Add|ColumnFunc::Sub|ColumnFunc::Mul|ColumnFunc::Div => {
                // 四则运算
                OperatorsBuilder::from(self).build(fn_expr,prev_conn)?;
            }
            ColumnFunc::In=>{
                //删除Dot生成的.
                self.writer.pop();
                IsInBuilder::from(self)
                    .build(&fn_expr,None,"")?;
            }
            ColumnFunc::Concat=>{
                //字符串拼接
                StrConcatBuilder::from(self).build(fn_expr,None,prev_conn)?;
            }
            _ => {
                //
                debug!("other func {}",fn_expr.name);
            }
        }
        Ok(())
    }
}

///
/// 获取左边表达式字符位置
///
fn parse_lhs_position(lhs: &Expr)->usize{
    match lhs {
        Expr::FnCall(_,position)=>position.position().unwrap(),
        Expr::And(x,..)=>parse_lhs_position(&x.lhs),
        Expr::Or(x,..)=>parse_lhs_position(&x.lhs),
        _=>0
    }
}

///
///
///
pub enum ColumnFunc{
    Col,
    Eq,
    Neq,
    Lt,
    Lte,
    Gt,
    Gte,
    Add,
    Sub,
    Mul,
    Div,
    In,
    None,
    When,
    Then,
    Otherwise,
    StrSlice,
    Concat
}

impl From<&str> for ColumnFunc {

    fn from(name: &str) -> Self {
        match name {
            "col"=>ColumnFunc::Col,
            "=="=>ColumnFunc::Eq,
            "!="|"<>"=>ColumnFunc::Neq,
            "<"=>ColumnFunc::Lt,
            "<="=>ColumnFunc::Lte,
            ">"=>ColumnFunc::Gt,
            ">="=>ColumnFunc::Gte,
            "+"=>ColumnFunc::Add,
            "-"=>ColumnFunc::Sub,
            "*"=>ColumnFunc::Mul,
            "/"=>ColumnFunc::Div,
            "is_in"=>ColumnFunc::In,
            "when"=>ColumnFunc::When,
            "then"=>ColumnFunc::Then,
            "otherwise"=>ColumnFunc::Otherwise,
            "str_slice"=>ColumnFunc::StrSlice,
            "concat_str"=>ColumnFunc::Concat,
            _=>ColumnFunc::None
        }
    }
}
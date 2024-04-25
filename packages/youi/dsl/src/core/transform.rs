use log::debug;
use rhai::{BinaryExpr, Dynamic, Engine, Expr, FnCallExpr, Stmt};
use crate::core::func::{ComparisonFunc, DfColExprFunc, LitWrapFunc, OperationFunc,ColSymbolFunc};
use crate::DslError;
use crate::error::DslResult as Result;

///
/// 脚本转换
///
pub struct DslTransform<'a>{
    script:&'a str,
    writer:String,
    engine:Engine,
}

///
///
///
impl<'a> From<&'a str> for DslTransform<'a>{
    fn from(script: &'a str) -> Self {
        let mut engine = Engine::new();
        engine.set_max_expr_depths(0,0);

        Self{
            script,
            engine,
            writer:"".to_string(),
        }
    }
}

impl<'a> DslTransform<'a>  {

    ///
    /// 脚本转换
    ///
    pub fn transform(&mut self)->Result<String>{
        let comp_ast = self.engine.compile(self.script);
        self.writer = "".to_string();
        match comp_ast {
            Ok(ast) => {
                for stmt in ast.statements() {
                    self.write_stmt(stmt)?;
                }
                Ok(self.writer.to_string())
            }
            Err(e) => Err(DslError::DslParseError(e.to_string()))
        }
    }
    ///
    ///
    ///
    fn write_stmt(&mut self,stmt:&Stmt)->Result<()>{
        match stmt {
            Stmt::Var(x, ..) => self.write_var(&x.0.name,&x.1)?,
            Stmt::Assignment(x) => {
                let (_, BinaryExpr { lhs, rhs }) = &**x;
                self.write_assignment(lhs,rhs)?;
            },
            Stmt::FnCall(x, _) => self.write_fn_expr(x)?,
            Stmt::Expr(x) => {
                self.write_expr(&x)?;
                self.writer.push_str(";");
            }
            _=>{
                debug!("other {:?}",stmt);
            }
        };
        Ok(())
    }
    ///
    /// 赋值
    ///
    fn write_assignment(&mut self,lhs:&Expr,rhs:&Expr)->Result<()>{
        self.write_expr(lhs)?;
        self.writer.push_str("=");
        self.write_expr(rhs)?;
        self.writer.push_str(";");
        Ok(())
    }
    ///
    /// 变量定义
    ///
    fn write_var(&mut self,name:&str,expr:&Expr)->Result<()>{
        self.writer.push_str("let ");
        self.writer.push_str(name);
        self.writer.push_str(" = ");
        self.write_expr(expr)?;
        self.writer.push_str(";");
        Ok(())
    }
    ///
    /// 表达式
    ///
    fn write_expr(&mut self,expr:&Expr)->Result<()>{

        match expr {
            Expr::DynamicConstant(x, _) => {
                let dynamic:&Dynamic = x.as_ref();
                self.write_dynamic_constant(dynamic)?;
            }
            Expr::Variable(x,..)=>self.writer.push_str(&x.1),
            Expr::BoolConstant(x, _) => self.writer.push_str(x.to_string().as_str()),
            Expr::IntegerConstant(x, _) => self.writer.push_str(x.to_string().as_str()),
            Expr::FloatConstant(x, _) => self.writer.push_str(x.to_string().as_str()),
            Expr::CharConstant(x, _) => self.writer.push_str(x.to_string().as_str()),
            Expr::StringConstant(x, _) => {
                self.writer.push_str("\"");
                self.writer.push_str(x);
                self.writer.push_str("\"");
            },
            Expr::Array(x, _) => {
                let arr = x.as_ref();
                let is_col_expr_arr = (0..arr.len()).find(|idx|is_col_func_expr(&arr[*idx])).is_some();
                if is_col_expr_arr{
                    self.writer.push_str("exprs(");
                }
                self.writer.push_str("[");
                for idx in 0..arr.len(){
                    if idx>0{
                        self.writer.push_str(",");
                    }
                    self.write_expr(&arr[idx])?;
                }
                self.writer.push_str("]");
                if is_col_expr_arr{
                    self.writer.push_str(")");
                }
            },
            Expr::FnCall(x,_)|Expr::MethodCall(x,_)=>self.write_fn_expr(x)?,
            Expr::Dot(x, _, _) => {
                self.write_expr(&x.lhs)?;
                self.writer.push_str(".");
                self.write_expr(&x.rhs)?;
            }
            Expr::And(x,..)=>{
                self.write_left_right_func("and","&&",&x.lhs,&x.rhs)?;
            },
            Expr::Or(x,..)=>{
                let BinaryExpr { lhs, rhs } = &**x;
                self.write_left_right_func("or","||",lhs,rhs)?;
            }
            _=>{
                debug!("other {:?}",expr);
            }
        }
        Ok(())
    }

    ///
    ///
    ///
    fn write_left_right_func(&mut self,name:&str,symbol:&str,lhs:&Expr,rhs:&Expr)->Result<()>{
        if is_col_func_expr(lhs) ||  is_col_func_expr(rhs){
            //列表达式函数
            self.write_left_right_expr(lhs,true)?;
            self.writer.push_str(".");
            self.writer.push_str(name);
            self.writer.push_str("(");
            self.write_left_right_expr(rhs,true)?;
            self.writer.push_str(")");
        }else{
            //常规值
            self.write_expr(lhs)?;
            self.writer.push_str(" ");
            self.writer.push_str(symbol);
            self.writer.push_str(" ");
            self.write_expr(rhs)?;
        }
        Ok(())
    }
    ///
    ///
    ///
    fn write_left_right_expr(&mut self,expr:&Expr,warp_lit:bool)->Result<()>{
        if warp_lit{
            match expr {
                Expr::BoolConstant(_, _) | Expr::IntegerConstant(_, _) | Expr::FloatConstant(_, _) | Expr::StringConstant(_, _) => {
                    self.writer.push_str("lit(");
                    self.write_expr(expr)?;
                    self.writer.push_str(")");
                },
                _=>self.write_expr(expr)?
            }
        }else{
            self.write_expr(expr)?
        }
        Ok(())
    }

    ///
    ///
    ///
    fn write_dynamic_constant(&mut self,dynamic:&Dynamic)->Result<()>{
        match dynamic.type_name() {
            "array" =>{
                self.writer.push_str(&dynamic.to_string());
            }
            _ => {}
        }
        Ok(())
    }

    ///
    /// 函数
    ///
    fn write_fn_expr(&mut self,fn_expr:&FnCallExpr)->Result<()>{
        let name = fn_expr.name.as_str();
        let opf = OperationFunc::from(name);
        let cmp = ComparisonFunc::from(name);

        match (&opf,&cmp) {
            // 一般函数
            (OperationFunc::Empty,ComparisonFunc::Empty)=>self.write_func(fn_expr)?,
            // 比较运算
            (OperationFunc::Empty,_)=>self.write_left_right_func(cmp.name(),cmp.symbol(),&fn_expr.args[0],&fn_expr.args[1])?,
            // 四则运算
            (_,ComparisonFunc::Empty)=>self.write_left_right_func(opf.name(),opf.symbol(),&fn_expr.args[0],&fn_expr.args[1])?,
            _=>{}
        };
        Ok(())
    }

    ///
    /// 写函数
    ///
    fn write_func(&mut self,fn_expr:&FnCallExpr)->Result<()>{
        let ns = &fn_expr.namespace;
        if !ns.is_empty(){
            self.writer.push_str(&format!("{}::",ns.root()));
        }

        self.writer.push_str(&fn_expr.name);

        self.writer.push_str("(");

        for idx in 0..fn_expr.args.len(){
            if idx>0 {
                self.writer.push_str(",");
            }
            //then/otherwise等函数中 增加lit函数
            self.write_func_arg_expr(&fn_expr.name,&fn_expr.args[idx])?;
        }

        self.writer.push_str(")");
        Ok(())
    }

    fn write_func_arg_expr(&mut self,func_name:&str,expr:&Expr)->Result<()>{

        let lwf = LitWrapFunc::from(func_name);

        match lwf {
            LitWrapFunc::Empty => self.write_expr(expr)?,
            _=>self.write_left_right_expr(expr,true)?,
        };

        Ok(())
    }
}

///
/// 是否包含列函数
///
fn is_col_func_expr(expr:&Expr)->bool{
    match expr {
        Expr::FnCall(x,..)|Expr::MethodCall(x,..)=>{
            //展开比较函数和运算函数
            let cmp = ColSymbolFunc::from(x.name.as_str());
            match cmp {
                ColSymbolFunc::Empty => match DfColExprFunc::from(x.name.as_str()){
                    DfColExprFunc::Empty=>false,
                    _=>true
                },
                _=>is_col_func_expr(&x.args[0]) || is_col_func_expr(&x.args[1])
            }
        }
        Expr::Dot(x,..)=>{
            is_col_func_expr(&x.lhs) || is_col_func_expr(&x.rhs)
        },
        _=> {
            false
        },
    }
}
use itertools::Itertools;
use rhai::{Engine, Stmt, Expr, FnCallExpr, Dynamic, BinaryExpr};
use crate::condition::transform_dsl_condition;
use crate::{SpecialFunc, util::{is_col_fun,op_to_symbol}};

///
/// 转换为可执行的脚本
///
pub fn transform(script:&str) ->String {
    let mut engine = Engine::new();
    engine.set_max_expr_depths(0,0);

    let opt_ast = engine.compile(script);

    match opt_ast {
        Ok(ast)=>{
            let mut scripts = String::new();
            for stmt in ast.statements() {
                let res = transform_stmt(stmt);
                scripts.push_str(&res);
            }
            scripts
        },
        Err(e)=>{
            println!("{:?}",e);
            script.to_string()
        }
    }
}

///
///
///
fn transform_stmt(stmt:&Stmt)->String{
    let mut scripts:String = String::new();

    match stmt {
        Stmt::Noop(_) => {}
        Stmt::If(_, _) => {}
        Stmt::Switch(_, _) => {}
        Stmt::While(_, _) => {}
        Stmt::Do(_, _, _) => {}
        Stmt::For(_, _) => {}
        Stmt::Var(x, ..) => {
            scripts.push_str("let ");
            scripts.push_str(&x.0.name);
            scripts.push_str(" = ");
            scripts.push_str(&transform_expr(&x.1));
            scripts.push_str(";");
        }
        Stmt::Assignment(x) => {
            let (_, BinaryExpr { lhs, rhs }) = &**x;
            scripts.push_str(&transform_expr(lhs));
            scripts.push_str("=");
            scripts.push_str(&transform_expr(rhs));
            scripts.push_str(";");
        }
        Stmt::FnCall(x, _) => {
            scripts.push_str(&transform_fn(&x));
        }
        Stmt::Block(_) => {
            println!("Block");
        }
        Stmt::TryCatch(_, _) => {}
        Stmt::Expr(x) => {
            scripts.push_str(" ");
            scripts.push_str(&transform_expr(&x));
            scripts.push_str(";");
        }
        Stmt::BreakLoop(..) => {}
        Stmt::Return(x, _, _) => {
            println!("return {:?}",x);
        }
        _=> {
            println!("ELSE");
        }
    }

    scripts
}

///
///
///
fn transform_expr(expr:&Expr) -> String{
    
    let mut scripts:String = String::new();
    match expr {
        Expr::DynamicConstant(x, _) => {
            let dynamic:&Dynamic = x.as_ref();

            match dynamic.type_name() {
                "array" =>{
                    //let array = dynamic.clone().into_array().unwrap();
                    //let script = array.iter().map(|v|format!("\"{}\"",v)).join(",");
                    scripts.push_str(&dynamic.to_string());
                }
                _ => {

                }
            }
        }
        Expr::BoolConstant(x, _) =>{
            scripts.push_str(x.to_string().as_str());
        }
        Expr::IntegerConstant(x, _)=>{
            scripts.push_str(x.to_string().as_str());
        }
        Expr::FloatConstant (x, _)=>{
            scripts.push_str(x.to_string().as_str());
        }
        Expr::CharConstant(x, _)=> {
            scripts.push_str(x.to_string().as_str());
        }
        Expr::StringConstant(x, _) => {
            scripts.push_str("\"");
            scripts.push_str(x);
            scripts.push_str("\"");
        }
        Expr::InterpolatedString(_, _) => {
            println!("InterpolatedString");
        }
        Expr::Array(x, _) => {
            //自动处理col表达式数组
            let is_exprs = x.len()>0 && is_col_fun(&x.as_ref()[0]);
            if is_exprs{
                scripts.push_str("exprs(");
            }
            scripts.push_str("[");
            if x.len()>0 {

                for e in x.as_ref() {
                    scripts.push_str(&transform_expr(&e));
                    scripts.push_str(",");
                }
                scripts.remove(scripts.len() - 1);
            }
            scripts.push_str("]");

            if is_exprs{
                scripts.push_str(")");
            }
        }
        Expr::Map(_, _) => {println!("Map ");}
        Expr::Unit(_) => {println!("Unit ");}
        Expr::Variable(x, _, _) => {
            scripts.push_str(&x.1);
        }
        Expr::Property(_, _) => {
            println!("Property ");
        }
        Expr::Stmt(_) => {
            println!("Stmt {:?}","");
        }
        Expr::FnCall(x, _) | Expr::MethodCall(x, _) => {
            scripts.push_str(&transform_fn(&x));
            //println!("fn call {:?}",x);
        }
        Expr::Dot(x, _, _) => {
            scripts.push_str(&transform_expr(&x.lhs));
            scripts.push_str(".");
            scripts.push_str(&transform_expr(&x.rhs));
        }
        Expr::Index(_, _, _) => {
            println!("Index ");
        }
        Expr::And(x, _) => {
            //表达式
            if is_col_fun(&x.lhs) || is_col_fun(&x.rhs){
                scripts.push_str(&transform_expr(&x.lhs));
                scripts.push_str(".and(");
                scripts.push_str(&transform_expr(&x.rhs));
                scripts.push_str(")");
            }
        }
        Expr::Or(x, _) => {
            if is_col_fun(&x.lhs) || is_col_fun(&x.rhs){
                scripts.push_str(&transform_expr(&x.lhs));
                scripts.push_str(".or(");
                scripts.push_str(&transform_expr(&x.rhs));
                scripts.push_str(")");
            }
        }
        Expr::Custom(_, _) => {
            println!("Custom ");
        }
        _ => {
            println!("else ");
        }
    }

    scripts
}

///
///处理四则运行及特定的函数
///
fn transform_fn(fn_expr: &FnCallExpr) -> String {
    let mut scripts: String = String::new();

    let op = SpecialFunc::from(&fn_expr.name);

    match op {
        SpecialFunc::Add(x) | SpecialFunc::Sub(x) | SpecialFunc::Div(x) | SpecialFunc::Mul(x) =>
            scripts.push_str(&transform_addition_fn(fn_expr, x)),
        SpecialFunc::FilterByExpression => scripts.push_str(&transform_func_one_expression("filter",fn_expr.args.first())),
        SpecialFunc::ColExpression=>scripts.push_str(&transform_col_expression(fn_expr.args.first())),
        SpecialFunc::ExecCheckFormula=>{
            let formula_exprs = fn_expr.args.iter().map(|arg|match arg {
                Expr::StringConstant(x, _) => transform_dsl_condition(x),
                _ => String::from("")
            }).filter(|s|s.as_str() != "").collect::<Vec<String>>();
            scripts.push_str(&transform_exec_check_formula(&formula_exprs));
        }
        _ => scripts.push_str(&transform_normal_func(fn_expr))
    }
    scripts
}

pub fn transform_func_one_expression (func_name:&str,expr_arg:Option<&Expr>)->String{
    let mut scripts:String = String::new();
    match expr_arg {
        None => {}
        Some(expression) => {
            match expression {
                Expr::StringConstant(s, _) => {
                    scripts.push_str(func_name);
                    scripts.push_str("(");
                    scripts.push_str(&transform_dsl_condition(&s));
                    scripts.push_str(")");
                }
                _ => {}
            }
        }
    };
    scripts
}
///
/// col_expression("col('列1').fill_null(lit(''))") => col("列1").fill_null(lit(""))
///
pub fn transform_col_expression(expr_arg:Option<&Expr>)->String{
    let mut scripts:String = String::new();
    match expr_arg {
        None => {},
        Some(expression) => {
            match expression {
                Expr::StringConstant(s, _) => {
                    let dsl = transform_dsl_condition(&s);
                    scripts.push_str(&dsl);
                }
                _ => {}
            }
        }
    };
    scripts
}
///
/// 执行审核公式
///
pub fn transform_exec_check_formula(formula_exprs:&Vec<String>)->String{
    //
    let mut scripts:String = String::new();

    scripts.push_str("select([col(\"*\"),");

    //使用when then otherwise 输出表达式执行结果，成功返回1，失败返回0
    let select_formulas = (0..formula_exprs.len())
        .map(|idx|format!("when({}).then(lit(1)).otherwise(lit(0)).alias(\"CF_{}\")",&formula_exprs[idx],idx))
        .join(",");

    scripts.push_str(&select_formulas);
    scripts.push_str("])");

    scripts
}
///
/// 一般函数
///
pub fn transform_normal_func(fn_expr: &FnCallExpr) ->String{
    let mut scripts:String = String::new();
    let ns = &fn_expr.namespace;
    if !ns.is_empty(){
        scripts.push_str(&format!("{}::",ns.root()));
    }
    scripts.push_str(&fn_expr.name);
    scripts.push_str("(");
    if fn_expr.args.len() > 0 {
        for e in &fn_expr.args {
            scripts.push_str(&transform_expr(&e));
            scripts.push_str(",");
        }
        scripts.remove(scripts.len() - 1);
    }
    scripts.push_str(")");
    scripts
}

///
/// 四则运算函数处理
///
pub fn transform_addition_fn(fn_expr:&FnCallExpr,op:&str)->String{
    let mut scripts:String = String::new();

    //判断两个参数是否有返回值为JE的函数
    let is_col_addition = is_col_fun(&fn_expr.args[0]) || is_col_fun(&fn_expr.args[1]);

    if is_col_addition{
        scripts.push_str(&transform_addition(&fn_expr.args[0],is_col_addition));
        scripts.push_str(".");
        scripts.push_str(op);
        scripts.push_str("(");
        scripts.push_str(&transform_addition(&fn_expr.args[1],is_col_addition));
        scripts.push_str(")");
    }else{
        let symbol = op_to_symbol(op);
        scripts.push_str(&transform_addition(&fn_expr.args[0],is_col_addition));
        scripts.push_str(symbol);
        scripts.push_str(&transform_addition(&fn_expr.args[1],is_col_addition));
    }
    scripts
}

///
/// 四则运算参数处理
///
fn transform_addition(expr: &Expr,is_col_addition:bool) -> String {
    let mut scripts: String = String::new();

    match expr {
        Expr::BoolConstant(_, _) | Expr::IntegerConstant(_, _) | Expr::FloatConstant(_, _) | Expr::StringConstant(_, _) => {
            if is_col_addition{
                scripts.push_str("lit(");
            }
            scripts.push_str(&transform_expr(expr));

            if is_col_addition{
                scripts.push_str(")");
            }
        }
        Expr::FnCall(_, _) | Expr::MethodCall(_, _) => {
            scripts.push_str(&transform_expr(expr));
        }
        Expr::Variable(x,..)=>{
            scripts.push_str(&x.1);
        }
        _ => {}
    }
    scripts
}

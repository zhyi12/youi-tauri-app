use rhai::{AST, Engine, Expr, FnCallExpr, Stmt};
use crate::SpecialFunc;
use crate::transform::{transform_addition_fn, transform_normal_func};

///
/// dsl表达式转换
///
pub fn transform_dsl_condition(script:&str) ->String {
    let mut engine = Engine::new();
    //设置支持无限表达式层级
    engine.set_max_expr_depths(0,0);
    //将表达式中的单引号转双引号并编译为AST
    let ast:AST = engine.compile(&script.replace("'", "\"")).unwrap();

    let mut scripts = String::new();
    //dsl转换为rhai引擎可执行的脚本
    for stmt in ast.statements() {
        let res = transform_stmt(stmt);
        scripts.push_str(&res);
    }
    scripts
}
///
///
///
fn transform_stmt(stmt:&Stmt)->String{
    let mut scripts:String = String::new();
    match stmt {
        Stmt::If(_, _) => {
            println!("If")
        }
        Stmt::FnCall(x, _) => {
            scripts.push_str(&transform_fn(&x));
        }
        Stmt::Expr(x) => {
            scripts.push_str(&transform_expr(&x));
        }
        _ => {
            println!("---other");
        }
    };

    scripts
}
///
///
///
fn transform_expr(expr:&Expr) -> String{
    let mut scripts:String = String::new();

    match expr {
        Expr::BoolConstant(x, _) =>scripts.push_str(x.to_string().as_str()),
        Expr::IntegerConstant(x, _)=>scripts.push_str(x.to_string().as_str()),
        Expr::FloatConstant (x, _)=>scripts.push_str(x.to_string().as_str()),
        Expr::CharConstant(x, _)=> scripts.push_str(x.to_string().as_str()),
        Expr::StringConstant(x, _) => {
            scripts.push_str("\"");
            scripts.push_str(x);
            scripts.push_str("\"");
        }
        Expr::FnCall(x, _)|Expr::MethodCall(x, _) => scripts.push_str(&transform_fn(&x)),
        Expr::And(x, _)=> {
            scripts.push_str(&transform_expr(&x.lhs));
            scripts.push_str(".and(");
            scripts.push_str(&transform_expr(&x.rhs));
            scripts.push_str(")");
        }
        Expr::Or(x, _)=> {
            scripts.push_str(&transform_expr(&x.lhs));
            scripts.push_str(".or(");
            scripts.push_str(&transform_expr(&x.rhs));
            scripts.push_str(")");
        }
        // Expr::Array(_, _) => {}
        // Expr::Map(_, _) => {}
        // Expr::Unit(_) => {}
        // Expr::Property(_, _) => {}
        // Expr::Stmt(_) => {
        //     println!("expr Stmt")
        // }
        Expr::Dot(x, _, _) => {
            scripts.push_str(&transform_expr(&x.lhs));
            scripts.push_str(".");
            scripts.push_str(&transform_expr(&x.rhs));
        }
        // Expr::Index(_, _, _) => {}
        // Expr::Coalesce(_, _) => {}
        _ => {
            println!("other expr");
            println!("{:?}",expr);
        }
    }

    scripts
}

///
///处理四则运行，比较符等
///
fn transform_fn(fn_expr:&FnCallExpr) -> String{
    let mut scripts:String = String::new();

    let op = SpecialFunc::from(&fn_expr.name);

    match op {
        SpecialFunc::Add(x)|SpecialFunc::Sub(x)|SpecialFunc::Div(x)|SpecialFunc::Mul(x)
            |SpecialFunc::Eq(x)|SpecialFunc::Gt(x)|SpecialFunc::Gte(x)
            |SpecialFunc::Lt(x)|SpecialFunc::Lte(x)=>
            scripts.push_str(&transform_addition_fn(fn_expr,x)),
        _=>scripts.push_str(&transform_normal_func(fn_expr))
    }
    scripts
}
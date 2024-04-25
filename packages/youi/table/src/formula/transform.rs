use itertools::Itertools;
use rhai::{Engine, Expr, FnCallExpr, Stmt};

use hashbrown::HashMap;
use lazy_static::lazy_static;
use log::{debug, error};
use crate::formula::position::{area_regex, AreaExpressionSwapper, cell_regex, CellPositionSwapper, short_cell_regex};

lazy_static! {
    static ref FORMULA_F64_FUNC_LIST: HashMap<u32, &'static str> = {
        let mut m = HashMap::new();
        m.insert(0, "VLOOKUP");
        m.insert(1, "GET");
        m.insert(2, "ROUND");
        m
    };

    static ref FORMULA_DY_FUNC_LIST: HashMap<u32, &'static str> = {
        let mut m = HashMap::new();
        m.insert(0, "AVERAGE");
        m.insert(1, "CONCAT");
        m
    };
}
///
/// 表达式转换
///
pub fn transform_expression(main_grid_name:&str,expression:&str,engine:&Engine,table_names:&HashMap<String,String>)->String{
    //表达式转换 A2:E3
    let exec_expression = replace_area_expression(main_grid_name,expression,table_names);

    match engine.compile(&exec_expression){
        Ok(ast)=>{
            let mut scripts = String::new();
            //转换为rhai引擎可执行的脚本
            for stmt in ast.statements() {
                let res = transform_stmt(table_names,main_grid_name,stmt);
                scripts.push_str(&res);
            }
            debug!("公式转换:{expression} => {exec_expression} => {scripts}");
            scripts
        }
        Err(e)=>{
            error!("{expression} => {exec_expression} parse error {:?}",e);
            "".to_string()
        }
    }

}
///
/// 替换区域表达式，防止由 : 导致的表达式不可解析
///
pub fn replace_area_expression(main_grid_name:&str,expression:&str,table_names:&HashMap<String,String>)->String{
    //区域替换
    let regex_area = area_regex();
    //单元格替换-只转换跨表的单元格公式
    let regex_cell = cell_regex();

    let result = regex_area.replace_all(expression,
                                        AreaExpressionSwapper{main_grid_name:main_grid_name.to_string(),table_names})
        .to_string();

    regex_cell.replace_all(&result,CellPositionSwapper{main_grid_name:main_grid_name.to_string(),table_names}).to_string()
}

///
///
///
fn transform_stmt(table_names:&HashMap<String,String>,main_grid_name:&str,stmt:&Stmt)->String{
    let mut scripts:String = String::new();
    match stmt {
        Stmt::If(_, _) => {
            println!("If")
        }
        Stmt::FnCall(x, _) => {
            scripts.push_str(&transform_fn(table_names,main_grid_name,&x));
        }
        Stmt::Expr(x) => {
            scripts.push_str(&transform_expr(table_names,main_grid_name,&x));
        }
        Stmt::Var(x,..)=>{
            println!("var {:?}",&x.0.name);
        }
        _ => {
            println!("---other {:?}",stmt);
        }
    };

    scripts
}

fn transform_expr(table_names:&HashMap<String,String>,main_grid_name:&str,expr:&Expr) ->String{
    _transform_expr(table_names,main_grid_name,expr,None,false)
}

fn transform_func_arg_expr(table_names:&HashMap<String,String>,main_grid_name:&str,expr:&Expr,fn_expr: &FnCallExpr) -> String{
    _transform_expr(table_names,main_grid_name,expr,Some(fn_expr),false)
}

///
/// . 符号前的表达式处理
///
fn transform_func_dot_left(table_names:&HashMap<String,String>,main_grid_name:&str,expr:&Expr)->String{
    _transform_expr(table_names,main_grid_name,expr,None,true)
}
///
///
///
fn _transform_expr(table_names:&HashMap<String,String>,main_grid_name:&str,expr:&Expr,_opt_fn_expr:Option<&FnCallExpr>,dot_left:bool) -> String{
    let mut scripts:String = String::new();

    match expr {
        Expr::Variable(x,..)=>{
            if !dot_left{
                let cell_expr = short_cell_regex()
                    .replace(&x.1,CellPositionSwapper{main_grid_name:main_grid_name.to_string(),table_names})
                    .to_string();
                // debug!("Variable {main_grid_name}.{} => {cell_expr}",&x.3);
                scripts.push_str(&cell_expr);
            }else{
                scripts.push_str(&x.1);
            }
        }
        Expr::BoolConstant(x, _) =>scripts.push_str(x.to_string().as_str()),
        Expr::IntegerConstant(x, _)=>scripts.push_str(x.to_string().as_str()),
        Expr::FloatConstant (x, _)=>scripts.push_str(x.to_string().as_str()),
        Expr::CharConstant(x, _)=> scripts.push_str(x.to_string().as_str()),
        Expr::StringConstant(x, _) => {
            scripts.push_str("\"");
            scripts.push_str(x);
            scripts.push_str("\"");
        }
        Expr::FnCall(x, _) => {
            scripts.push_str(&transform_fn(table_names,main_grid_name,&x))
        },
        Expr::MethodCall(x, _) => scripts.push_str(&transform_fn(table_names,main_grid_name,&x)),
        Expr::Dot(x, _, _) => {
            scripts.push_str(&transform_func_dot_left(table_names,main_grid_name,&x.lhs));
            scripts.push_str(".");
            scripts.push_str(&transform_expr(table_names,main_grid_name,&x.rhs));
        }
        Expr::Array(x,_)=>{
            let count = x.len();
            scripts.push_str("[");
            scripts.push_str(&(0..count).map(|_idx|transform_expr(table_names,main_grid_name,&x[0])).join(","));
            scripts.push_str("]");
        }
        _ => {
            println!("other expr {:?}",expr);
        }
    }

    scripts
}

///
///处理四则运行，比较符等
///
fn transform_fn(table_names:&HashMap<String,String>,main_grid_name:&str,fn_expr:&FnCallExpr) -> String{
    let mut scripts:String = String::new();

    //四则运算
    //比较
    //动态参数函数
    //一般函数

    match fn_expr.name.as_str() {
        "+"|"-"|"*"|"/"=>{
            scripts.push_str(&transform_addition_fn(table_names,main_grid_name,fn_expr,&fn_expr.name));
        },
        "=="|">"|"<"|">="|"<=" => {
            scripts.push_str(&transform_compare_func(table_names,main_grid_name,fn_expr,&fn_expr.name));
        },
        _=>{
            scripts.push_str(&transform_normal_func(table_names,main_grid_name,fn_expr));
        }
    };
    scripts
}

///
/// 比较函数 ==,>,<,>=,<=
///
fn transform_compare_func(table_names:&HashMap<String,String>,main_grid_name:&str,fn_expr: &FnCallExpr,op:&str) ->String{
    let mut scripts:String = String::new();

    let casts = match (&fn_expr.args[0],&fn_expr.args[1]){
        (Expr::FloatConstant(..),Expr::FloatConstant(..)) | (Expr::StringConstant(..),Expr::StringConstant(..))=>vec!["",""],
        (Expr::FloatConstant(..),_) | (Expr::IntegerConstant(..),_)=>vec!["",".f64()"],
        (_,Expr::FloatConstant(..)) | (_,Expr::IntegerConstant(..))=>vec![".f64()",""],
        (Expr::StringConstant(..),_)=>vec!["",".str()"],
        (_,Expr::StringConstant(..))=>vec![".str()",""],
        _=>vec!["",""]
    };

    scripts.push_str(&transform_expr(table_names,main_grid_name,&fn_expr.args[0]));
    scripts.push_str(casts[0]);
    scripts.push_str(op);
    scripts.push_str(&transform_expr(table_names,main_grid_name,&fn_expr.args[1]));
    scripts.push_str(casts[1]);
    scripts
}

///
/// 一般函数
///
fn transform_normal_func(table_names:&HashMap<String,String>,main_grid_name:&str,fn_expr: &FnCallExpr) ->String{
    let mut scripts:String = String::new();
    let ns = &fn_expr.namespace;
    if !ns.is_empty(){
        scripts.push_str(&format!("{}::",ns.root()));
    }
    scripts.push_str(&fn_expr.name);
    scripts.push_str("(");
    //dyna
    if fn_expr.args.len() > 0 {
        if is_dynamic_fn(&fn_expr.name){
            scripts.push_str("[");
        }

        for e in &fn_expr.args {
            scripts.push_str(&transform_func_arg_expr(table_names,main_grid_name,&e,fn_expr));
            scripts.push_str(",");
        }
        scripts.remove(scripts.len() - 1);

        if is_dynamic_fn(&fn_expr.name){
            scripts.push_str("]");
        }
    }
    scripts.push_str(")");

    scripts
}
///
///四则运算
///
fn transform_addition_fn(table_names:&HashMap<String,String>,main_grid_name:&str,fn_expr:&FnCallExpr,op:&str)->String{
    let mut scripts:String = String::new();

    scripts.push_str(&transform_expr(table_names,main_grid_name,&fn_expr.args[0]));
    //如果是函数，加上f64转换
    scripts.push_str(&append_f64(&fn_expr.args[0]));
    scripts.push_str(op);
    scripts.push_str(&transform_expr(table_names,main_grid_name,&fn_expr.args[1]));
    scripts.push_str(&append_f64(&fn_expr.args[1]));
    scripts
}

///
///
///
fn is_f64_fn_call(expr:&Expr)->bool{
    let opt_name:Option<&str> = match expr {
        Expr::MethodCall(x, _) => {
            Some(&x.name)
        }
        _ => None
    };
    match opt_name {
        Some(name)=>{
            let opt_func = FORMULA_F64_FUNC_LIST.values().find(|v|**v == name);
            match opt_func {
                None=>false,
                Some(_)=>true
            }
        }
        None=>false,
    }
}

///
///
///
fn is_dynamic_fn(name:&str)->bool{
    let opt_func = FORMULA_DY_FUNC_LIST.values().find(|v|**v == name);
    match opt_func {
        None=>false,
        Some(_)=>true
    }
}


fn append_f64(expr:&Expr)->String{
    match expr {
        Expr::Dot(x,..) => {
            if is_f64_fn_call(&x.rhs){
                ".f64()".to_string()
            }else{
                "".to_string()
            }
        }
        Expr::FnCall(x, _) => {
            let opt_func = FORMULA_F64_FUNC_LIST.values().find(|v|**v == &x.name);
            match opt_func {
                None=>"".to_string(),
                Some(_)=>".f64()".to_string()
            }
        }
        Expr::Variable(_x,..)=>{
            ".f64()".to_string()
        }
        _ => {
            println!("other append_f64 {:?}",expr);
            "".to_string()
        }
    }
}
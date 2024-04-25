use rhai::Expr;

///
/// 列表达式判断
///
pub fn is_col_fun(expr:&Expr)->bool{

    let col_func_names = vec!["col","col_expression","when","lit","concat_str","concat_list"];

    match expr {
        Expr::Dot(x,_,_)=>{
            is_col_fun(&x.lhs)
        },
        Expr::FnCall(x,_)=>{
            let name:&str = &x.name;
            col_func_names.contains(&name)
        }
        _=>{
            false
        }
    }
}

pub fn op_to_symbol(op: &str) -> &str {
    match op {
        "add"=>"+",
        "sub"=>"-",
        "mul"=>"*",
        "div"=>"/",
        _=>""
    }
}

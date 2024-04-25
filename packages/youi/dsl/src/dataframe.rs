use rhai::plugin::*;

///
/// 注册polars数据计算函数
///
#[export_module]
pub mod ds_module {
    use polars_core::prelude::{DataType, Field, NamedFrom, SortOptions};
    use polars_core::series::Series;
    use polars_lazy::dsl::{When,Then, ChainedThen, ChainedWhen};
    use rhai::{Dynamic, plugin::EvalAltResult};
    use youi_dataframe::lazy::{JsLazyFrame, JsExpr};

    pub type DF = JsLazyFrame;

    pub type JE = JsExpr;

    pub type W = When;

    pub type T = Then;

    pub type CT = ChainedThen;

    pub type CW = ChainedWhen;

    pub type S = Series;

    pub type F = Field;

    pub fn field(name:String,data_type:String)->F{
        Field::new(&name,match data_type.as_str() {
            "int"=>DataType::Int64,
            "float"=>DataType::Float64,
            "number"=>DataType::Float64,
            _=>DataType::String
        })
    }

    pub fn series_str(name:String,data:Vec<Dynamic>)->S{
        S::new(&name,data.iter().map(|i|i.clone_cast::<String>()).collect::<Vec<String>>())
    }

    pub fn series_float(name:String,data:Vec<Dynamic>)->S{
        S::new(&name,data.iter().map(|i|i.clone_cast::<f64>()).collect::<Vec<f64>>())
    }

    pub fn series_int(name:String,data:Vec<Dynamic>)->S{
        S::new(&name,data.iter().map(|i|i.clone_cast::<i64>()).collect::<Vec<i64>>())
    }

    pub fn read_series(data:Vec<Dynamic>)->DF{
        let series_list = data.iter().map(|i|i.clone_cast::<S>()).collect::<Vec<S>>();
        DF::read_series(series_list)
    }

    ///
    /// 读取csv文件
    ///
    #[rhai_fn(return_raw,name = "read_csv")]
    pub fn read_csv(path:String)-> Result<DF, Box<EvalAltResult>>{
        match DF::read_csv(&path,vec![]) {
            Ok(df)=>Ok(df),
            Err(_)=>{
                Err(format!("文件{}读取错误.",path).into())
            }
        }
    }

    #[rhai_fn(return_raw,name = "read_csv")]
    pub fn read_csv_with_schema(path:String,data:Vec<Dynamic>)-> Result<DF, Box<EvalAltResult>>{
        let fields:Vec<F> = data.iter().map(|i|i.clone_cast::<F>()).collect::<Vec<F>>();
        match DF::read_csv(&path,fields) {
            Ok(df)=>Ok(df),
            Err(_)=>{
                Err(format!("文件{}读取错误.",path).into())
            }
        }
    }
    ///
    /// 读取 sqlite 数据库数据
    ///
    #[rhai_fn(name = "read_sql")]
    pub fn read_sql(connect:String,sql:String)->DF{
        DF::read_sql(connect,sql,"binary")
    }

    ///
    /// 自定义协议
    ///
    #[rhai_fn(name = "read_sql")]
    pub fn read_sql_with_protocol(connect:String,sql:String,protocol:String)->DF{
        DF::read_sql(connect,sql,&protocol)
    }

    ///
    /// 输出csv文件
    ///
    pub fn write_csv(df:DF,path:String)->DF{
        DF::write_csv(df,path)
    }

    ///
    /// 写入sqlite数据库
    ///
    pub fn write_sql(df:DF,connect:String,table_name:String,
                     input_col_names:Vec<Dynamic>,output_col_names:Vec<Dynamic>)->DF{
        let input_col_names:Vec<String> = input_col_names.iter().map(|item|item.clone_cast::<String>()).collect();
        let output_col_names:Vec<String> = output_col_names.iter().map(|item|item.clone_cast::<String>()).collect();
        DF::write_sql(df,connect,input_col_names,table_name,output_col_names)
    }

    ///
    /// 读取parquet文件
    ///
    pub fn read_parquet(path:String)->DF{
        DF::read_parquet(path)
    }
    ///
    /// 写入parquet文件
    ///
    pub fn write_parquet(df:DF,path:String)->DF{
        df.write_parquet(path)
    }

    ///
    /// 输出json
    ///
    pub fn to_json(df:DF)->String{
        df.to_json()
    }

    ///
    /// 读取csv 列表头
    ///
    pub fn read_csv_header(path:String)->DF{
        DF::read_csv_header(path)
    }
    ///
    /// 分页读取csv数据
    ///
    pub fn pager_read_csv(path:String,page_index:i64,page_size:i64)->DF{
        DF::pager_read_csv(path,page_index,page_size)
    }
    ///
    /// 选择
    ///
    pub fn select(df:DF,js_exprs:Vec<JsExpr>)->DF{
        df.select(js_exprs)
    }
    ///
    /// 汇总
    ///
    pub fn agg(df:DF,by:String,js_exprs:Vec<JsExpr>)->DF{
        df.agg(by,js_exprs)
    }

    ///
    ///
    ///
    pub fn cube(df:DF,by:String,js_exprs:Vec<JsExpr>,measures:String)->DF{
        df.cube(by,js_exprs,measures)
    }

    ///
    /// 透视表
    ///
    pub fn pivot(df:DF,values:Vec<Dynamic>,indexes:Vec<Dynamic>,columns:Vec<Dynamic>)->DF{
        let values:Vec<String> = values.iter().map(|d|d.clone_cast()).collect();
        let indexes:Vec<String> = indexes.iter().map(|d|d.clone_cast()).collect();
        let columns:Vec<String> = columns.iter().map(|d|d.clone_cast()).collect();
        df.pivot(&values,&indexes,&columns).unwrap()
    }
    ///
    /// 排序
    ///
    pub fn sort_by_exprs(df:DF,js_exprs:Vec<JsExpr>,reverses:String)->DF{
        df.sort_by_exprs(js_exprs,reverses)
    }

    ///
    /// 过滤
    ///
    pub fn filter(df:DF,expr:JsExpr)->DF{
        df.filter(expr)
    }

    ///
    /// 排序
    ///
    #[rhai_fn(name = "sort")]
    pub fn sort(df:DF,name:String,descending:bool)->DF{
        df.sort(name,descending)
    }
    ///
    /// 返回指定行数数据
    ///
    pub fn limit(df:DF,n:i64)->DF{
        df.limit(n)
    }

    ///
    /// 连接数据集
    ///
    pub fn join(df:DF,other:JsLazyFrame,how:&str,left_on:String,right_on:String)->DF{
        df.join(other,how,left_on,right_on)
    }

    pub fn unique(df:DF,col_name:String)->DF{
        let col_names = col_name.split(",").map(|s|s.to_string()).collect::<Vec<String>>();
        df.unique(col_names)
    }

    ///
    /// 左连接
    ///
    pub fn left_join(df:DF,other:JsLazyFrame,left_on:String,right_on:String)->DF{
        df.left_join(other,left_on,right_on)
    }
    ///
    /// 上下合并
    ///
    pub fn union(df:DF,other:JsLazyFrame)->DF{
        df.union(other)
    }

    ///
    /// 多数据集合并
    ///
    pub fn concat(dfs:Vec<Dynamic>)->DF{
        let dfs:Vec<JsLazyFrame> = dfs.iter().map(|d|d.clone_cast()).collect();
        DF::concat(dfs)
    }

    ///
    /// 集合类型的列转行
    ///
    pub fn explode(df:DF,js_exprs:Vec<JsExpr>)->DF{
        df.explode(js_exprs)
    }

    ///
    ///
    ///
    pub fn with_columns(df:DF,js_exprs:Vec<JsExpr>)->DF{
        df.with_columns(js_exprs)
    }

    ///
    /// 索引匹配
    ///
    pub fn match_items(df:DF,df_idx:DF,fields:String,search_filed:String,out_prefix:String)->DF{
        let text_fields = fields.split(",").map(|s|String::from(s)).collect();
        DF{
            df:youi_dataframe::column_match::df_match_items(df.df,&df_idx.df,&text_fields,search_filed,out_prefix)
        }
    }

    ///
    ///
    ///
    pub fn exprs(exprs: &mut Vec<Dynamic>)->Vec<JsExpr>{
        exprs.iter().map(|elem|{
            let js_expr:JsExpr = elem.clone_cast();
            js_expr
        }).collect()
    }

    ///
    ///
    ///
    pub fn col(name:String)->JE{
        JE::col(name)
    }

    pub fn cols(names:Vec<String>)->JE{
        JE::cols(names)
    }

    pub fn first(expr:JE)->JE{
        expr.first()
    }

    pub fn last(expr:JE)->JE{
        expr.last()
    }

    pub fn count(expr:JE)->JE{
        expr.count()
    }

    pub fn sum(expr:JE)->JE{
        expr.sum()
    }

    pub fn min(expr:JE)->JE{
        expr.min()
    }

    pub fn max(expr:JE)->JE{
        expr.max()
    }

    pub fn list(expr:JE)->JE{
        expr.list()
    }


    ///
    /// 平均数
    ///
    pub fn mean(expr:JE)->JE{
        expr.mean()
    }

    ///
    /// 中位数
    ///
    pub fn median(expr:JE)->JE{
        expr.median()
    }

    ///
    /// marks that we want to compute something within a group, but doesn't modify the original size of the DataFrame
    /// df.select(
    ///     [
    ///         "Type 1",
    ///         "Type 2",
    ///         pl.col("Attack").mean().over("Type 1").alias("avg_attack_by_type"),
    ///         pl.col("Defense").mean().over(["Type 1", "Type 2"]).alias("avg_defense_by_type_combination"),
    ///         pl.col("Attack").mean().alias("avg_attack"),
    ///     ]
    /// )
    ///
    pub fn over(expr:JE,js_exprs:Vec<JsExpr>)->JE{
        expr.over(js_exprs)
    }

    pub fn alias(expr:JE,alias_name:String)->JE{
        expr.alias(alias_name)
    }

    pub fn is_null(expr:JE)->JE{
        expr.is_null()
    }

    ///
    ///
    ///
    pub fn fill_null(expr:JE,other:JE)->JE{
        expr.fill_null(other)
    }

    pub fn is_not_null(expr:JE)->JE{
        expr.is_not_null()
    }

    ///
    /// 等于
    ///
    pub fn eq(expr:JE,other:JsExpr)->JE{
        expr.eq(other)
    }

    ///
    /// 不等于
    ///
    pub fn neq(expr:JE,other:JsExpr)->JE{
        expr.neq(other)
    }

    pub fn gt(expr:JE,other:JsExpr)->JE{
        expr.gt(other)
    }

    pub fn gte(expr:JE,other:JsExpr)->JE{
        expr.gte(other)
    }

    pub fn lt(expr:JE,other:JsExpr)->JE{
        expr.lt(other)
    }

    pub fn lte(expr:JE,other:JsExpr)->JE{
        expr.lte(other)
    }

    pub fn or(expr:JE,other:JsExpr)->JE{
        expr.or(other)
    }

    pub fn and(expr:JE,other:JsExpr)->JE{
        expr.and(other)
    }

    #[rhai_fn(name = "is_in")]
    pub fn is_in(expr:JE,values:Vec<Dynamic>)->JE{
        let values = values.iter().map(|i|i.clone_cast::<String>()).collect::<Vec<String>>();
        expr.is_in(values)
    }

    #[rhai_fn(name = "lit")]
    pub fn value_expr(value:String)->JE{
        JE::value_expr(value)
    }

    #[rhai_fn(name = "lit")]
    pub fn value_expr_i64(value:i64)->JE{
        JE::value_expr_i64(value)
    }

    #[rhai_fn(name = "lit")]
    pub fn value_expr_bool(value:bool)->JE{
        JE::value_expr_bool(value)
    }

    #[rhai_fn(name = "lit")]
    pub fn value_expr_f64(value:f64)->JE{
        JE::value_expr_f64(value)
    }

    ///
    /// 字符连接
    ///
    pub fn concat_str(js_exprs:Vec<JsExpr>, sep: &str)-> JE{
        JE::concat_str(js_exprs,sep)
    }

    pub fn exclude(expr:JE,col_name:String)->JE{
        let col_names = col_name.split(",").map(|s|s.to_string()).collect::<Vec<String>>();
        expr.exclude(col_names)
    }
    ///
    /// to list
    ///
    pub fn concat_list(js_exprs:Vec<JsExpr>)->JE{
        JE::concat_list(js_exprs)
    }

    #[rhai_fn(name = "sort")]
    pub fn sort_expr(expr:JE,descending:bool)->JE{
        JE{
            expr:expr.expr.sort(SortOptions::default().with_order_descending(descending))
        }
    }
    ///
    ///
    ///
    pub fn str_slice(expr:JE,start: i64,len:i64)->JE{
        expr.str_slice(start,len)
    }
    ///
    ///
    ///
    pub fn str_length(expr:JE) -> JE{
        expr.str_length()
    }

    ///
    ///
    ///
    pub fn str_split(expr:JE,by:String) -> JE{
        expr.str_split(&by)
    }
    ///
    ///
    ///
    pub fn str_replace(expr:JE,pat: String, val: String) -> JE{
        expr.str_replace(pat,val)
    }
    ///
    ///
    ///
    pub fn str_replace_all(expr:JE,pat: String, val: String) -> JE{
        expr.str_replace_all(pat,val)
    }

    ///
    ///
    ///
    pub fn str_contains(expr:JE,pat: String)->JE{
        expr.str_contains(pat)
    }

    ///
    /// 以...开始字符匹配
    ///
    pub fn start_with(expr:JE,pat: String)->JE{
        expr.str_contains(format!("^{}",pat))
    }
    ///
    ///
    ///
    pub fn cast_str(expr:JE)->JE{
        expr.cast_str()
    }

    pub fn cast_i16(expr:JE)->JE{
        expr.cast_i16()
    }
    pub fn cast_i32(expr:JE)->JE{
        expr.cast_i32()
    }

    pub fn cast_i64(expr:JE)->JE{
        expr.cast_i64()
    }

    pub fn cast_f32(expr:JE)->JE{
        expr.cast_f32()
    }

    pub fn cast_f64(expr:JE)->JE{
        expr.cast_f64()
    }

    #[rhai_fn(name = "add")]
    pub fn add(expr:JE,js_expr:JsExpr)->JE{
        expr.add(js_expr)
    }

    pub fn sub(expr:JE,js_expr:JsExpr)->JE{
        expr.sub(js_expr)
    }

    pub fn mul(expr:JE,js_expr:JsExpr)->JE{
        expr.mul(js_expr)
    }

    pub fn div(expr:JE,js_expr:JsExpr)->JE{
        expr.div(js_expr)
    }

    #[rhai_fn(name = "when")]
    pub fn when(js_expr:JsExpr)->W{
       JE::when(js_expr)
    }

    #[rhai_fn(name = "when")]
    pub fn when_chain(t:T,js_expr:JsExpr)->CW{
        t.when(js_expr.expr)
    }
    #[rhai_fn(name = "then")]
    pub fn then(w:W,js_expr:JsExpr)->T{
        JE::then(w,js_expr)
    }

    #[rhai_fn(name = "then")]
    pub fn then_chain(cw:CW,js_expr:JsExpr)->CT{
        cw.then(js_expr.expr)
    }

    #[rhai_fn(name = "otherwise")]
    pub fn otherwise(t:T,js_expr:JsExpr)->JE{
        JE{expr:t.otherwise(js_expr.expr)}
    }

    #[rhai_fn(name = "otherwise")]
    pub fn otherwise_wtt(ct:CT,js_expr:JsExpr)->JE{
        JE{expr:ct.otherwise(js_expr.expr)}
    }

    ///
    ///
    ///
    pub fn flatten(expr:JE)->JE{
        expr.flatten()
    }


    ///
    /// 特殊函数
    ///
    pub fn filter_by_expression(df:DF,_expression:String)->DF{
        df
    }

    ///
    /// 列计算
    ///
    pub fn calculator(df:DF,_expression:String)->DF{
        df
    }

    pub fn exec_check_formula(df:DF,_expressions:Vec<String>)->DF{
        df
    }
    ///
    ///
    ///
    pub fn col_expression(expr:JsExpr)->JE{
        expr
    }

}
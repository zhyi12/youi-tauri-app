use std::collections::HashMap;
use futures::executor::{block_on};
use polars_core::frame::DataFrame;
use polars_core::prelude::{PolarsResult as Result, AnyValue};
use sqlx::{QueryBuilder, SqlitePool, Sqlite};

///
/// 写入dataframe到数据库
///
pub(crate) fn write(pool:&SqlitePool,init_sql:&str,df:&DataFrame,col_names:&Vec<String>,chunk_size:usize) -> Result<()>{
    let total_count = df.height();
    let page_count = (total_count as f32 /chunk_size as f32).ceil() as i32;

    let names = df.get_column_names();

    let fut_values = async {
        for idx in 0..page_count{
            let start = idx as i32 * chunk_size as i32;
            let df = df.slice(start as i64, chunk_size);
            println!("{} {} {} {}",page_count,idx,start,chunk_size);

            if df.height() == 0 {
                break;
            }
            //
            let mut builder:QueryBuilder<Sqlite>= chunk_query_build(&df,&names,col_names,init_sql);
            let query = builder.build();
            //执行数据库写入
            query.execute(pool).await.unwrap();
        }
    };

    block_on(fut_values);

    Ok(())
}

///
/// 行数据转HashMap
///
fn to_map_data<'a>(names:&'a Vec<&'a str>,values:&'a Vec<AnyValue<'a>>) -> HashMap<String,&'a AnyValue<'a>>{
    let count = names.len();
    let mut data_map = HashMap::with_capacity(count);

    for idx in 0..count{
        data_map.insert(String::from(&names[idx].to_string()),&values[idx]);
    }

    data_map
}

///
/// 构建sqlx batch query builder - 批量插入数据sql语句
///
fn chunk_query_build<'a>(df:&'a DataFrame,names:&'a Vec<&'a str>,col_names:&'a Vec<String>,init_sql:&'a str)->QueryBuilder<'a, Sqlite>{
    let mut builder:QueryBuilder<Sqlite>= QueryBuilder::new(init_sql);
    builder.push_values(0..df.height(),|mut sp,idx|{
        let row_values = df.get(idx).unwrap();
        let data_map = to_map_data(names,&row_values);

        for j in 0..col_names.len(){
            let col_name = &col_names[j];

            if !data_map.contains_key(col_name){
                sp.push_bind("");
            }else{
                let value = data_map.get(col_name).unwrap();
                match value {
                    AnyValue::String(v) => {
                        sp.push_bind(String::from(*v));
                    }
                    AnyValue::Int64(v)=>{
                        sp.push_bind(v.to_owned());
                    }
                    AnyValue::Int32(v)=>{
                        sp.push_bind(v.to_owned());
                    }
                    AnyValue::Float64(v)=>{
                        sp.push_bind(v.to_owned());
                    }
                    AnyValue::Float32(v)=>{
                        sp.push_bind(v.to_owned());
                    }
                    AnyValue::Boolean(v)=>{
                        sp.push_bind(v.to_owned());
                    }
                    _=>{
                        sp.push_bind("");
                    }
                }
            }
        }
    });

    builder
}
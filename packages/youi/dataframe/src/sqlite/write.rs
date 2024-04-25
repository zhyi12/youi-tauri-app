use futures::executor::block_on;
use itertools::Itertools;
use polars_core::prelude::{DataFrame, PolarsResult as Result};
use polars_lazy::prelude::{IntoLazy, LazyFrame};

use super::*;

pub struct SqliteWriter{
    connect:String,
    table_name:String,
    column_names:Vec<String>,
    batch_size:Option<usize>
}

impl SqliteWriter{

    pub fn new(connect:&str,table_name:&str,column_names:Vec<String>,batch_size:Option<usize>)->Self{
        Self{
            connect:connect.to_string(),
            table_name:table_name.to_string(),
            column_names,
            batch_size
        }
    }

    pub fn finish(&mut self, df: &DataFrame,col_names:Vec<String>) -> Result<LazyFrame> {
        let pool = block_on(sqlx::SqlitePool::connect(&self.connect)).unwrap();
        let sep_names = &self.column_names.iter().map(|column|String::from(column)).join(",");
        let init_sql = format!("INSERT INTO {} ({}) ",self.table_name,sep_names);

        println!("init_sql {}",init_sql);
        write_impl::write(&pool,&init_sql,df,&col_names,self.batch_size.or(Some(1)).unwrap())?;

        let lazy  = df.clone().lazy();
        Ok(lazy)
    }
}
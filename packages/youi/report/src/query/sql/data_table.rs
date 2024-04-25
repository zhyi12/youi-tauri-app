use crate::dataset::{DataColumn, DataTable};
use crate::query::sql::to_table_sql;

impl DataTable {

    ///
    /// 单表sql语句
    ///
    pub fn sql(&self,columns:&Vec<&DataColumn>,global_filter_script:&str)->String{
        let column_names = columns.iter().map(|c|c.name_clone()).collect::<Vec<String>>();
        to_table_sql(self.name(),&column_names,global_filter_script)
    }
}
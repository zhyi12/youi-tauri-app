use itertools::Itertools;
use crate::{Report, Cube, DataCube, DataTable, DataGrid};
use crate::utils::build_dimension_column_expression;

///
/// dls 查询语句相关
///
impl Report {
    ///
    /// dsl查询
    ///
    pub fn dsl_query<F, H>(self: &mut Self, mut f: F, mut expression_processor: H) -> DataGrid
        where
            F: FnMut(String, &Cube) -> DataCube,
            H: FnMut(&str) -> String
    {
        //查询模型
        let query_model = self.build_query_model();
        //
        let data_cubes = query_model.cubes.as_ref().unwrap().iter().map(|cube| {
            let mut dsl = String::new();
            //立方体数据表集合
            let cube_tables = query_model.find_cube_dataset(cube);
            //生成数据集dsl
            dsl.push_str(&dataset_to_dsl(&cube_tables, &mut expression_processor));

            let s = cube_select_dsl(cube, &mut expression_processor);
            //println!("{}",s);
            dsl.push_str(&s);

            //生成汇总语句
            dsl.push_str(&cube.dsl());
            dsl.push_str("df");

            //
            let data_cube = f(dsl, &cube);
            data_cube
        }).collect::<Vec<DataCube>>();

        //输出表格
        self.render(&query_model, &data_cubes)
    }
}

///
///
///
impl Cube {
    ///
    /// 生成dsl语句
    ///
    pub fn dsl(self: &Self) -> String {

        // 分组字段集合
        let group_by = self.get_groups().iter()
            .map(|g| format!("{}", g.name)).join(",");
        //计量项集合
        let measure_items = self.get_measure_items();
        //dsl 汇总表达式
        let agg_exprs = measure_items
            .iter()
            .map(|item| format!("col(\"{}\").{}().alias(\"{}\")",
                                item.name.as_ref().unwrap(),
                                item.aggregate.as_ref().unwrap(), item.id
            ))
            .join(",");

        format!("df = df.agg(\"{}\",[{}]);", group_by, agg_exprs)
    }
}

impl DataTable {
    ///
    ///
    ///
    pub fn dsl(self: &Self) -> String {
        format!("let df_{}=read_{}(\"{}\");", self.name, self.reader, self.uri)
    }
}

///
/// 数据集转dsl
/// TODO 查询条件处理
///
fn dataset_to_dsl<F>(dataset: &Vec<&DataTable>, _x: &mut F) -> String
    where F: FnMut(&str) -> String {
    let mut dsl = String::new();

    let len = dataset.len();
    // 数据表查询脚本
    for idx in 0..len {
        let table = dataset[&len - idx - 1];
        dsl.push_str(&table.dsl());
    }

    //
    // 生成 join 脚本
    // let df = t1;
    // df = t1.left_join(t2);
    // df = df.left_join(t3);
    // ...
    //
    for idx in 0..len {
        let table = dataset[len - idx - 1];
        if idx == 0 {
            dsl.push_str(&format!("let df = df_{};", table.name));
        }
        if idx < &len - 1 {
            let join_table = dataset[len - idx - 2];
            let join_by = join_table.columns.iter()
                .filter(|column| {
                    match column.primary_key {
                        Some(b) => b,
                        None => false
                    }
                })
                .map(|column| format!("{}", column.name.as_ref().unwrap_or(&String::new())))
                .collect::<Vec<String>>().join(",");

            dsl.push_str(&format!("df = df.left_join(df_{},\"{}\",\"{}\");", join_table.name, join_by, join_by));
        }
    }

    dsl
}

fn cube_select_dsl<F>(cube: &Cube, expression_processor: &mut F) -> String
    where F: FnMut(&str) -> String {
    let measure_exprs = cube.get_measure_items().iter().map(|item|format!("col(\"{}\")",item.name.as_ref().unwrap())).join(",");
    let group_col_exprs = cube.get_groups().iter().map(|dimension| build_dimension_column_expression(dimension, expression_processor)).join(",");
    format!("df = df.select([{},{}]);",measure_exprs,group_col_exprs)
}
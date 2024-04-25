use itertools::Itertools;
use crate::cube::Cube;
use crate::item::{Item, MeasureItem};
use crate::query::dataset::QueryDataset;
use crate::{ColumnSelect, ReportModel};

///
/// 立方体执行
///
pub struct CubeExecutor<'a>{

    ///
    /// 查询用数据集
    ///
    pub(crate) query_dataset:&'a QueryDataset<'a>,
    ///
    /// 立方体
    ///
    pub(crate) cube:&'a Cube,
    ///
    /// 立方体用到的表名
    ///
    pub(crate) cube_table_names:Vec<String>,
    ///
    /// 立方体输出的列名
    /// Vec(表名,列名)
    ///
    pub(crate) cube_out_columns:Vec<(String, String)>,

    ///
    /// 全局过滤脚本
    ///
    pub(crate) global_filter_script:String,

    ///
    /// 计量列集合
    ///
    pub(crate) measure_items:&'a Vec<Item>,

    ///
    /// 立方体需要汇总的计量集合，包括从计量计算项中提取的计量
    ///
    pub(crate) agg_measure_items:Vec<MeasureItem>,

    ///
    /// 立方体的分组输出列
    ///
    pub(crate) group_select:Vec<ColumnSelect>,

}

impl<'a> CubeExecutor<'a> {
    ///
    ///
    ///
    pub fn new(
        model:&'a ReportModel,
        query_dataset:&'a QueryDataset<'a>,
               cube:&'a Cube,
               cube_table_names:Vec<String>,
               cube_out_columns:Vec<(String,String)>)->Self{

        // 立方体引用的计量
        let opt_measure_items = cube.find_measure_items();
        // 多数据集合并生成的列
        let union_group_select = model.find_dataset_union_group_select();
        // 分组输出列
        let group_select = cube.find_group_select(&union_group_select);
        //
        let agg_measure_items = match opt_measure_items {
            None=>vec![],
            Some(measure_items)=>parse_agg_measure_items(measure_items)
        };

        Self{
            // model,
            query_dataset,
            cube,
            cube_table_names,
            cube_out_columns,
            measure_items:opt_measure_items.unwrap(),
            agg_measure_items,
            group_select,
            global_filter_script:"".to_string()
        }
    }
}

///
/// 输出的计量列
/// 1、计量列
/// 2、计算计量公式中提取的计量列
///
fn parse_agg_measure_items(measure_items:&Vec<Item>)->Vec<MeasureItem>{
    measure_items
        .iter()
        .map(|item|match item {
            Item::Measure(m) => m.parse_measure_items(),
            _=>vec![]
        })
        .flatten()
        .sorted()
        .dedup()
        .collect::<Vec<MeasureItem>>()
}


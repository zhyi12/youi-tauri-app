use std::collections::HashMap;
use itertools::Itertools;
use petgraph::Graph;
use petgraph::prelude::DiGraph;
use crate::{ReportQueryModel, Dimension, Item, Report, constants::DIMENSION_MEASURE, Cube, Rank, DataTable, DataColumn};

impl Cube {

    ///
    /// 获取计量项
    ///
    pub fn get_measure_items(self:&Self) -> Vec<Item>{
        let measure_dimension = self.dimensions.iter()
            .filter(|dim|dim.name.as_str() == DIMENSION_MEASURE)
            .next();

        match measure_dimension{
            Some(dim)=>{
                dim.items.as_ref().unwrap().iter().map(|item|item.clone()).collect::<Vec<Item>>()
            }
            None=>{
                Vec::<Item>::new()
            }
        }
    }

    ///
    /// 获取分组维度集合
    ///
    pub fn get_groups(self:&Self) -> Vec<Dimension>{
        self.dimensions.iter()
            .filter(|dim|dim.name.as_str() != DIMENSION_MEASURE)
            .map(|dim|dim.clone()).collect::<Vec<Dimension>>()
    }

    ///
    /// 获取相关数据表名集合
    ///
    pub fn get_ref_tables(self:&Self) ->Vec<String>{
        self.dimensions
            .iter()
            .map(|dim|{
                if dim.name == DIMENSION_MEASURE{
                    dim.items.as_ref().unwrap().iter()
                        .map(|item|item.table_name.as_ref().unwrap_or(&String::new()).to_string())
                        .collect::<Vec<String>>()
                }else{
                    vec![dim.table_name.as_ref().unwrap_or(&String::new()).to_string()]
                }
            }).flatten()
            .dedup()
            .filter(|name|name != "")
            .collect::<Vec<String>>()
    }
}

///
/// 行、列维度
///
impl Rank {
    ///
    /// 合并相同维度ID的维度项
    ///
    pub fn merge_dimension_items(&self, other: &Rank) -> Rank{
        //
        let merged_dimensions = self.dimensions.iter().map(|dimension|{
            //
            let mut merged_dimension = dimension.clone();

            let other_dimension:Option<&Dimension> = (&other.dimensions).iter().filter(|dim|dim.id == dimension.id).find_or_first(|_|true);

            match other_dimension{
                Some(d)=>{
                    merged_dimension.merge_items(&d.items.as_ref().unwrap());
                }
                None => {}
            }

            merged_dimension
        }).collect();

        Rank{
            index:self.index.clone(),
            dimensions:merged_dimensions
        }
    }

}

///
/// report cube builder
///
impl <'a> ReportQueryModel<'a> {

    ///
    /// 从报表构建立方体
    ///
    pub fn from(report:&'a Report)->Self{

        let extended_dimension_map = parse_extended_dimension_map(report);
        //主栏行元数据集合
        let main_ranks = parse_main_ranks(report);
        //合并维度项
        let main_merged_ranks = merge_cube_ranks(&main_ranks);

        //宾栏列元数据集合
        let slave_ranks = parse_slave_ranks(report,&extended_dimension_map);
        let slave_merged_ranks = merge_cube_ranks(&slave_ranks);

        //立方体
        let cubes:Vec<Cube> = parse_cubes(&main_merged_ranks,&slave_merged_ranks);

        //解析数据集
        let dataset = parse_dataset(report.dataset.as_ref().unwrap());

        ReportQueryModel {
            main_ranks:Some(main_ranks),
            slave_ranks: Some(slave_ranks),
            cubes: Some(cubes),
            dataset,
            extended_dimension_map
        }
    }
}

impl <'a> ReportQueryModel<'a> {
    ///
    /// 立方体的dataset
    ///
    pub fn find_cube_dataset(self:&Self,cube:&Cube)->Vec<&DataTable>{
        let table_names = cube.get_ref_tables();
        self.dataset.iter()
            .filter(|table|table_names.contains(&(table.name)))
            .collect::<Vec<&DataTable>>()
    }
}

///
///
///
pub struct JoinLine{
    pub from:String,
    pub to:String,
}

///
///
///
fn parse_dataset(dataset:&Vec<DataTable>)->Vec<DataTable>{
    let mut root:Graph<DataTable,JoinLine> = DiGraph::new();
    add_to_graph(&mut root,dataset,&0);

    root.raw_nodes().iter().map(|node|{
        println!("{},层级：{}",node.weight.name,node.weight.level.unwrap());
        node.weight.clone()
    }).collect::<Vec<DataTable>>()
}

///
/// 迭代获取全部的数据集
///
fn add_to_graph(graph:&mut Graph<DataTable,JoinLine>,dataset:&Vec<DataTable>,parent_level:&usize){

    let level = parent_level+1;
    dataset.iter().for_each(|table|{
        let columns = table.columns.iter()
            .map(|column|column.clone())
            .collect::<Vec<DataColumn>>();

        graph.add_node(DataTable{
            name:table.name.to_string(),
            reader:table.reader.to_string(),
            uri:table.uri.to_string(),
            level: Some(level),
            columns,
            sub_tables: None,
            join_tables: None
        });

        // 从表
        match &table.join_tables {
            None => {}
            Some(t) => {
                println!("join tables {}",t.len());
                add_to_graph(graph, t, &level);
            }
        };
        // 子表
        match &table.sub_tables {
            None => {}
            Some(t) => {
                add_to_graph(graph, t, &level);
            }
        };
    });
}

///
/// 主宾栏交叉，解析立方体集合
///
fn parse_cubes(main_ranks:&Vec<Rank>,slave_ranks:&Vec<Rank>)->Vec<Cube>{
    main_ranks.into_iter().cartesian_product(slave_ranks).map(|c|{
        let (main_rank,slave_rank) = c;

        let mut dimensions:Vec<Dimension> =
            (&main_rank.dimensions).into_iter().map(|dim|dim.clone()).collect::<Vec<Dimension>>();

        let mut slave_dimensions =  (&slave_rank.dimensions).into_iter().map(|dim|dim.clone()).collect::<Vec<Dimension>>();

        dimensions.append(&mut slave_dimensions);

        Cube{
            dimensions
        }
    }).collect::<Vec<Cube>>()
}
///
/// 按主栏行合并元数据，需要先处理缩进
///
fn parse_main_ranks(report:&Report)->Vec<Rank>{
    let slave_rows = report.slave_rows.unwrap();
    let main_rows = report.rows.as_ref().unwrap().len() as i32;

    let mut main_ranks:Vec<Rank> = vec![];
    for index in slave_rows..main_rows{
        let mut dimensions:Vec<Dimension> = vec![];

        let cell = report.find_cell(index as usize, 0);

        match cell {
            Some(c)=>dimensions.append(&mut c.find_dimensions()),
            None=>{}
        }

        main_ranks.push(Rank{
            index:index.clone(),
            dimensions
        });
    }
    main_ranks
}

///
///
///
fn parse_extended_dimension_map(report:&Report)->HashMap<(usize,usize),&Vec<Dimension>>{
    let mut extended_dimension_map:HashMap<(usize,usize),&Vec<Dimension>> = HashMap::new();
    //合并单元格处理

    //拐角处理
    report.corners.as_ref().unwrap().iter().for_each(|area|{
        let opt_cell = report.find_cell(area.start_row,area.start_col);
        match opt_cell {
            None => {}
            Some(cell) => {
                match &cell.dimensions {
                    None => {}
                    Some(dimensions) => {
                        let coordinates = area.get_coordinates();
                        coordinates.iter().for_each(|coordinate|{
                            extended_dimension_map.insert(*coordinate, dimensions);
                        });
                    }
                }
            }
        }
    });

    extended_dimension_map
}

///
/// 按主栏列合并元数据，需要先处理元数据扩展，包括拐角元数据扩展、合并单元格元数据自动扩展。
///
fn parse_slave_ranks(report:&Report,extended_dimension_map:&HashMap<(usize,usize),&Vec<Dimension>>)->Vec<Rank>{
    //
    let main_columns = report.main_columns.unwrap();
    let slave_rows = report.slave_rows.unwrap();
    let col_count = report.col_models.as_ref().unwrap().len() as i32;

    let mut slave_ranks:Vec<Rank> = vec![];
    //遍历宾栏列，解析宾栏列维度
    for col_idx in main_columns..col_count{
        let mut dimensions:Vec<Dimension> = vec![];
        //合并多行的维度
        for row_idx in 0..slave_rows{
            let extended_key = (row_idx as usize,col_idx as usize);
            let cell = report.find_cell(row_idx as usize, col_idx as usize);
            match cell {
                Some(c)=>{
                    match &c.dimensions {
                        None => {
                            if extended_dimension_map.contains_key(&extended_key) {
                                let opt_dimensions = extended_dimension_map.get(&extended_key);
                                match opt_dimensions {
                                    None => {}
                                    Some(ds) => {
                                        dimensions.append(&mut (*ds).clone());
                                    }
                                }
                            }
                        }
                        Some(ds) => {
                            dimensions.append(&mut ds.clone());
                        }
                    }
                },
                None=>{ }
            }
        }
        //去除重复的计量项

        slave_ranks.push(Rank{
            index:col_idx,
            dimensions
        });
    }
    slave_ranks
}

///
/// 合并行、列维度集合的维度项
///
fn merge_cube_ranks(ranks:&Vec<Rank>) -> Vec<Rank>{
    // 按照具有相同维度组合的Rank分组
    let groups = ranks.into_iter()
        .filter(|rank|!rank.dimensions.is_empty())
        .group_by(|rank|build_dimensions_key(&rank.dimensions));
    //Rank组合并
    let mut grouped_ranks:Vec<Rank> = vec![];
    //遍历Rank组，并执行相同维度的维度项的合并
    for (_, group) in &groups{
        let ranks = &group.collect::<Vec<&Rank>>();
        let mut merged_rank:Rank = ranks[0].clone();
        //按照维度集合key执行多个rank的维度合并
        for idx in 1..ranks.len(){
            merged_rank = merged_rank.merge_dimension_items(&ranks[idx]);
        }
        grouped_ranks.push(merged_rank);
    }

    grouped_ranks
}
///
///
///
fn build_dimensions_key(dimensions:&Vec<Dimension>)->String{
    dimensions.into_iter()
        .map(|dim|String::from(&dim.id))
        .sorted()
        .join(",")
        .to_string()
}


use std::collections::{HashMap, HashSet};
use itertools::Itertools;
use log::debug;
use crate::dataset::{DataColumn, DataTable, Node};
use crate::filter::{CubeFilter, JoinMode};
use crate::ReportModel;
use crate::tree::Tree;

///
/// 查询数据集
///
/// join、union
///
pub struct QueryDataset<'a>{

    pub(crate) model: Option<&'a ReportModel>,

    pub(crate) name:&'a str,

    ///
    /// 数据集的数据表map<表名、数据表>
    ///
    pub(crate) table_map:HashMap<String,&'a DataTable>,

    ///
    /// 列map
    ///
    pub(crate) column_map:HashMap<String,&'a DataColumn>,

    ///
    /// 数据集的数据表树
    ///
    tree:Tree<Node>

}

impl<'a> From<&'a DataTable> for QueryDataset<'a> {

    ///
    ///
    ///
    fn from(table: &'a DataTable) -> Self {
        let tree = table.to_tree(false);
        let table_map = table.to_join_table_map();
        let column_map = parse_column_map(&table_map);

        Self{
            name:table.name(),
            table_map,
            column_map,
            tree,
            model:None
        }
    }

}
///
/// 用于数据集中的数据列查找
///
fn parse_column_map<'a>(table_map:&HashMap<String,&'a DataTable>)->HashMap<String,&'a DataColumn>{
    HashMap::from_iter(table_map
        .iter()
        .map(|(_,t)|
            t.columns
                .iter()
                .map(|c|(c.name_clone(),c))
        ).flatten()
    )
}

impl<'a> QueryDataset<'a>{

    pub fn set_model(mut self,model:&'a ReportModel)->Self{
        self.model = Some(model);
        self
    }
}

impl<'a> QueryDataset<'a>{

    pub fn tree(&self) -> &Tree<Node> {
        &self.tree
    }

    pub fn name(&self)->&str{
        self.name
    }

    ///
    /// 根据表名集合获取立方体过滤
    ///
    pub fn find_cube_filter(&self,table_names:&Vec<String>)->Option<&CubeFilter>{
        //
        match self.model {
            None => None,
            Some(model) => model.find_cube_filter(table_names)
        }
    }
    ///
    /// 全部列集合
    ///
    pub(crate) fn to_column_map(&self,cube_query_columns:&Vec<(String,String)>)->HashMap<String,Vec<&DataColumn>>{
        cube_query_columns
            .into_iter()
            .map(|(table_name,column_name)|{
                let opt_column = self.column_map.get(column_name);
                match opt_column {
                    None=>vec![],
                    Some(column)=>vec![(table_name.to_string(),*column)]
                }
            })
            .flatten()
            .into_group_map()
    }
    ///
    /// 构建数据集的唯一查询
    /// 1、查找最末尾的非Union的数据表叶子节点
    /// 2、如果数据表为从表，关联全部同级节点
    ///
    pub fn build_one_query(&self)->Option<CubeQuery>{
        let table_paths = self.find_table_paths();
        //最后一条路径
        let opt_last_path = table_paths.last();
        match opt_last_path {
            None => None,
            Some(last_path) => {
                //
                let table_names = last_path.iter().map(|node|match node{
                    Node::Table(t) => vec![t.name_clone()],
                    _=>vec![]
                }).flatten().collect::<Vec<String>>();
                //立方体过滤
                let cube_filter = self.find_cube_filter(&table_names);

                let count = last_path.len();

                let join_tables = if count >1 {
                    let parent_node = last_path[count-2];
                    let parent_join_id = match parent_node {
                        Node::Join(x)=>x.id(),
                        _=>""
                    };
                    table_paths
                        .iter()
                        .map(|paths|{
                            let count = paths.len();
                            let name = paths[count-1].name();
                            if count>1 && paths[count-2].id() == parent_join_id && name != &table_names[table_names.len()-1]{
                                vec![name.to_string()]
                            }else{
                                vec![]
                            }
                        }).flatten().collect::<Vec<String>>()
                }else{
                    vec![]
                };
                self.build_cube_query(&table_names,&join_tables,cube_filter)
            }
        }
    }

    ///
    /// 获取非Union的叶子节点数据表连接路径集合
    ///
    /// 数据集树结构
    /// T1
    ///    从表
    ///       T2
    ///       T3
    ///         合并表
    ///            T4
    ///
    /// 叶子节点表路径为 T1/T2 T1/T3
    ///
    ///
    fn find_table_paths(&self)->Vec<Vec<&Node>>{
        self.tree.find_node_paths()
            .iter()
            .filter(|nodes|nodes.iter().find(|node|match node {
                Node::Union(_) => true,
                _ => false,
            }).is_none())
            .cloned()
            .collect::<Vec<Vec<&Node>>>()
    }

    ///
    /// 单一查询输出列
    ///
    pub(crate) fn find_one_query_columns(&self,cube_query:&mut CubeQuery,cube_out_columns:&Vec<(String,String)>)->Vec<(String,String)>{
        let cube_column_names:HashSet<String> = HashSet::from_iter(cube_out_columns
            .iter()
            .map(|(_,name)|name.to_string())
            .collect::<Vec<String>>());
        // debug!("cube_out_columns :{:?}",cube_out_columns);
        cube_query.query_tables
            .iter_mut()
            .map(|q|{
                let opt_table = self.table_map.get(&q.name);
                match opt_table {
                    None => vec![],
                    Some(table) => {
                        let mut columns = table.columns.iter().map(|c|{
                            if c.is_primary_key() || cube_column_names.contains(c.name()){
                                vec![(table.name_clone(),c.name_clone())]
                            }else{
                                vec![]
                            }
                        }).flatten().collect::<Vec<(String,String)>>();

                        //更新QueryTable数据列名集合
                        q.set_column_names(columns.iter().map(|(_,name)|name.to_string()).collect::<Vec<String>>());

                        //当前表的同级连接表处理
                        columns.extend(self.find_join_table_columns(q,&cube_column_names));

                        columns
                    }
                }
            }).flatten()
            .collect::<Vec<(String,String)>>()
    }

    ///
    /// 单一查询模式下，获取同级join表的列信息集合
    ///
    fn find_join_table_columns(&self,q:&QueryTable,columns:&HashSet<String>)->Vec<(String,String)>{
        match &q.join_tables {
            None => vec![],
            Some(join_tables) => {
                join_tables.iter().map(|name|{
                    let opt_table = self.table_map.get(name);
                    let mut table_columns = match opt_table {
                        None=>vec![],
                        Some(table)=>{
                            table.columns.iter()
                                .map(|column|{
                                    if columns.contains(&column.name) && !q.primary_keys.contains(&column.name){
                                        vec![(table.name_clone(),column.name.to_string())]
                                    }else{
                                        vec![]
                                    }
                                })
                                .flatten()
                                .collect::<Vec<(String,String)>>()
                        }
                    };
                    //主键
                    table_columns.extend(q.primary_keys.iter().map(|key|(name.to_string(),key.to_string())));

                    table_columns
                }).flatten().collect::<Vec<(String,String)>>()
            }
        }
    }
}

impl<'a> QueryDataset<'a>{

    ///
    /// @param table_names 立方体查询路径表名集合
    /// @param leaf_join_tables 叶子表相同从属的表
    ///
    pub fn build_cube_query(&self,table_names:&Vec<String>,leaf_join_tables:&Vec<String>,cube_filter:Option<&'a CubeFilter>)->Option<CubeQuery>{
        //
        let nodes = self.tree.nodes();
        //获取立方体用到的表的树节点所在的位置
        let node_indexes = (0..nodes.len()).filter(|idx|{
            match &nodes[*idx]{
                Node::Table(table)=>table_names.contains(&table.name_clone()),
                _=>false
            }
        })
            .collect::<Vec<usize>>();
        //获取连续的join表路径
        let paths = self.tree.paths(node_indexes[node_indexes.len()-1],node_indexes[0]);
        debug!("table tree:{:?}",self.tree);
        // debug!("{:?},tables {:?},paths: {:?}",node_indexes,table_names,paths);
        //按树路径逆序的表名集合，从下往上left join
        let mut query_tables = paths
            .iter()
            .filter(|node|match node {
                Node::Table(_)=>true,
                _=>false
            })
            .map(|node|match node {
                Node::Table(t)=>self.build_query_table(t.name()),
                _=>None
            })
            .filter(|opt|opt.is_some())
            .map(|opt|opt.unwrap())
            .collect::<Vec<QueryTable>>();

        if !leaf_join_tables.is_empty() && !query_tables.is_empty(){
            query_tables.first_mut().unwrap().set_join_tables(leaf_join_tables.iter().cloned().collect::<Vec<String>>());
        }

        // 计量
        // 分组

        Some(CubeQuery::new(query_tables,cube_filter))
    }

    ///
    /// 构建查询表，用于生成立方体数据查询脚本
    ///
    fn build_query_table(&self,name:&str)->Option<QueryTable>{
        let opt_table = self.table_map.get(name);
        match opt_table{
            None => None,
            Some(table) => {
                Some(QueryTable::new(name.to_string(),
                    table.primary_keys(),
                    table.union_table_names()))
            }
        }
    }

    ///
    /// 立方体查询的数据表连接方式
    ///
    pub(crate) fn cube_query_join_mode(&self,_cube_query:&CubeQuery)->Option<JoinMode>{


        None
    }
}

///
/// 立方体查询
///
///
pub struct CubeQuery<'a>{

    ///
    /// 立方体用到的数据表集合
    ///
    pub(crate) query_tables:Vec<QueryTable>,

    ///
    /// 立方体过滤
    ///
    pub(crate) cube_filter:Option<&'a CubeFilter>,


}

impl<'a> CubeQuery<'a> {
    pub fn new(query_tables:Vec<QueryTable>,cube_filter:Option<&'a CubeFilter>)->Self{
        Self{
            query_tables,
            cube_filter
        }
    }
}

impl<'a>  CubeQuery<'a> {

    pub fn table_names(&self)->Vec<&str>{
        self.query_tables
            .iter()
            .map(|t|t.name.as_str())
            .collect::<Vec<&str>>()
    }

    ///
    ///
    ///
    pub fn leaf_join_tables(&self) -> Option<&Vec<String>> {
        if self.query_tables.len()>0{
            self.query_tables[0].join_tables.as_ref()
        }else{
            None
        }
    }

    ///
    /// 返回顺序的多表名称
    ///
    pub fn cube_query_left_joins(&self)->Vec<LeftJoin>{
        let count = self.query_tables.len();

        (0..count-1).map(|idx|{
            let query_table = &self.query_tables[idx];
            let join_table = &self.query_tables[idx+1];
            let mut tables = vec![];
            if idx == 0 {
                //从表
                match &query_table.join_tables {
                    None => {}
                    Some(join_tables) => {
                        tables.extend(join_tables.iter().map(|jt|{
                            LeftJoin{
                                on: query_table.primary_keys.clone(),
                                name:jt.to_string()
                            }
                        }));
                    }
                };
            }
            tables.push(LeftJoin{
                on:join_table.primary_keys.clone(),
                name:join_table.name.to_string()
            });
            tables
        }).flatten().collect::<Vec<LeftJoin>>()
    }

}

///
///
///
pub struct LeftJoin{
    pub(crate) on:Vec<String>,
    pub(crate) name:String
}

#[derive(Debug)]
pub struct QueryTable{
    ///
    /// 表名
    ///
    pub(crate) name:String,
    ///
    /// 主键名集合
    ///
    pub(crate) primary_keys:Vec<String>,
    ///
    /// 合并表集合
    ///
    pub(crate) union_tables:Vec<String>,

    ///
    /// 连接表（从表节点下的同级引用的表）
    ///
    pub(crate) join_tables:Option<Vec<String>>,

    ///
    /// 当前表列名集合
    ///
    pub(crate) column_names:Vec<String>,
}

impl QueryTable {
    pub fn new(name:String,primary_keys:Vec<String>,union_tables:Vec<String>)->Self{
        Self{
            name,
            primary_keys,
            union_tables,
            join_tables:None,
            column_names:vec![]
        }
    }
}

impl QueryTable{
    ///
    ///
    ///
    pub fn set_join_tables(&mut self,join_tables:Vec<String>){
        self.join_tables = Some(join_tables);
    }
    ///
    ///
    ///
    pub fn set_column_names(&mut self,column_names:Vec<String>){
        self.column_names = column_names;
    }
}

impl QueryTable {
    ///
    /// 查询输出列集合
    ///
    pub fn out_columns(&self,prefix:&str)->Vec<String>{
        let mut out_columns = self.column_names
            .iter()
            .map(|n|format!("{}{}",prefix,n))
            .collect::<Vec<String>>();
        //是否包含leaf_union_col
        if !self.union_tables.is_empty(){
            out_columns.push(format!("{}leaf_union_col",prefix));
        }
        out_columns
    }
}
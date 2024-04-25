use std::fmt::Formatter;
use hashbrown::HashMap;
use grid::Grid;
use serde::{Serialize, Deserialize, Serializer};
use crate::{Cell,Area,Formula};

///
/// 
/// 
#[derive(Serialize, Deserialize,Debug)]
pub struct TableList{
    ///
    /// 
    /// 
    tables:Vec<Table>,

    ///
    /// 处理后的表名称
    ///
    #[serde(skip_serializing,skip_deserializing)]
    table_names:HashMap<String,String>,
}

///
///
///
impl TableList {

    ///
    ///
    ///
    pub fn new(tables:Vec<Table>)->Self{
        let table_names = HashMap::from_iter((0..tables.len()).map(|idx|{
            let table_name = &tables[idx].name;
            (table_name.to_string(),format!("TTB_{}",idx))
        }).collect::<Vec<(String,String)>>());

        Self{
            tables,
            table_names
        }
    }
}

impl TableList {

    ///
    /// 更新数据
    ///
    pub fn update(&mut self,data_map:&HashMap<String,Grid<Cell>>){
        self.tables.iter_mut().for_each(|table|{
            let key = self.table_names.get(&table.name).unwrap();
            let opt_data = data_map.get(key);
            match opt_data{
                Some(data)=>{
                    table.update_grid(data.clone());
                }
                None=>{}
            }
        });
    }
}

impl TableList {

    ///
    ///
    ///
    pub fn find_formula_list(&self)->Vec<Formula>{
        self.tables.iter()
            .map(|table|table.find_formulas().iter().map(|f|f.clone()))
            .flatten()
            .collect::<Vec<Formula>>()
    }

    ///
    ///
    ///
    pub fn find_data_map(&self)->HashMap<String,Grid<Cell>>{
        HashMap::from_iter(
            self.tables.iter()
                .map(|table|(self.table_names.get(&table.name).unwrap().to_string(),table.find_grid().clone()))
                .collect::<Vec<(String,Grid<Cell>)>>()
        )
    }

    ///
    ///
    ///
    pub fn find_table_names(&self)->&HashMap<String,String>{
        &self.table_names
    }
}


///
/// 表格
///
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Table{
    pub(crate) name:String,
    pub(crate) title:Option<String>,
    pub(crate) rows:Option<Vec<Row>>,
    pub(crate) grid:Option<Grid<Cell>>,
    pub(crate) merged_cells:Option<Vec<Area>>,
    pub(crate) col_models:Option<Vec<ColModel>>,
    pub(crate) formulas:Option<Vec<Formula>>
}

impl Table {
    pub fn new(name:String,grid:Grid<Cell>,formulas:Vec<Formula>)->Self{
        Self{
            name:name.to_string(),
            title:Some(name.to_string()),
            rows:None,
            grid:Some(grid),
            merged_cells:None,
            col_models:None,
            formulas:Some(formulas)
        }
    }
}

impl Table {

    ///
    /// 更新数据
    ///
    pub fn update_grid(&mut self,grid:Grid<Cell>){
        self.grid = Some(grid);
    }
}

impl Table {
    pub fn find_title(&self) -> &str {
        self.title.as_ref().unwrap()
    }
    ///
    ///
    ///
    pub fn find_grid(&self)->&Grid<Cell>{
        self.grid.as_ref().unwrap()
    }
    ///
    ///
    ///
    pub fn find_formulas(&self)->&Vec<Formula>{
        self.formulas.as_ref().unwrap()
    }
}

#[derive(Serialize,Deserialize,Debug)]
pub struct ColModel{
    ///
    /// 列宽度
    ///
    pub(crate) width:Option<f64>
}

///
///
///
#[derive(Debug,Serialize, Deserialize)]
pub struct Row{
    pub(crate) height:Option<f64>,
    pub(crate) cells:Vec<Cell>
}

///
/// fmt Debug
///
impl std::fmt::Debug for Table{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.serialize_str("\n")?;
        match &self.grid {
            None => {}
            Some(grid) => {
                grid.fmt(f)?;
            }
        };
        f.serialize_str("\n")?;
        Ok(())
    }
}
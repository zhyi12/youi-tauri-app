use std::collections::HashMap;
use itertools::Itertools;
use crate::{Cell, DataCube, DataGrid, Dimension, Item, Report, ReportQueryModel, Row};

impl Report {
    ///
    ///
    ///
    pub fn render(self: &Self,model: &ReportQueryModel, data_cubes: &Vec<DataCube>) -> DataGrid {
        //查询结果的维度项
        let mut dimension_items_map: HashMap<String, Vec<Item>> = HashMap::new();

        data_cubes.iter().for_each(|data_cube| {
            data_cube.dimensions.iter().for_each(|dim| {
                dimension_items_map.insert(dim.id.to_string(), dim.items.clone().unwrap());
            });
        });

        //使用数据中的维度项填充主宾栏的扩展型元数据
        let mut data_grid = self.expand_to_data_grid(model,&dimension_items_map);

        //填充立方体数据
        data_grid.render_by_cubes(data_cubes);

        data_grid
    }

    ///
    /// 填充维度项
    ///
    pub fn expand_to_data_grid(self: &Self,model: &ReportQueryModel,
                            dimension_items_map: &HashMap<String, Vec<Item>>) -> DataGrid {
        // 构建数据行，并使用数据中的维度项替换默认分组的项
        let rows = self.rows.as_ref().unwrap().iter().map(|row| Row {
            height: row.height.clone(),
            cells: row.cells.iter().map(|cell| {
                render_cell(cell, dimension_items_map)
            }).collect::<Vec<Cell>>()
        }).collect::<Vec<Row>>();

        let mut data_grid =
            DataGrid::new(rows, self.slave_rows.unwrap() as usize, self.main_columns.unwrap() as usize);

        //展开宾栏
        self.expand_grid_by_slave(&mut data_grid,model);
        //展开主栏
        self.expand_grid_by_main(&mut data_grid);

        data_grid
    }

    ///
    /// 宾栏维度项展开
    ///
    fn expand_grid_by_slave(self: &Self, data_grid: &mut DataGrid,model: &ReportQueryModel) {
        let col_count = data_grid.frozen_columns + self.col_models.as_ref().unwrap().len() - 1;
        let row_count = self.rows.as_ref().unwrap().len();

        let mut grid_col_count = col_count;
        //
        for idx in data_grid.frozen_columns..col_count {
            let col_idx = col_count - idx + 1 - data_grid.frozen_columns;

            //可展开的维度集合
            let mut col_expandable_dimensions:Vec<&Dimension> = vec![];
            let mut expandable_row_indexes:Vec<usize> = vec![];
            for row_idx in 0..data_grid.frozen_rows {
                // 从单元格获取可展开的维度
                //  每个宾栏单元格中最多存在一个可展开的维度
                let cell_dimensions = data_grid.find_cell_dimensions(row_idx,col_idx);
                //
                match cell_dimensions {
                    None => {}
                    Some((multi_item_dimensions,_one_item_dimensions)) => {
                        let expandable_dimension = multi_item_dimensions.first();

                        match expandable_dimension {
                            None => {}
                            Some(dimension) => {
                                col_expandable_dimensions.push(dimension);
                                expandable_row_indexes.push(row_idx);
                            }
                        }
                    }
                }
            }
            let i = 1;
            let expand_col_count = col_expandable_dimensions.iter().map(|d|d.items.as_ref().unwrap().len())
                .fold(i,|a,b|a*b);
            //宾栏列扩展
            if expand_col_count>1{
                grid_col_count += expand_col_count;
                let cross_item_list = dimensions_item_cartesian(&col_expandable_dimensions, false);

                (0..row_count).for_each(|idx|{
                    if expandable_row_indexes.contains(&idx){

                        let opt_cell = self.find_cell(idx,col_idx);
                        let cord = (idx,col_idx);
                        //拐角扩展
                        let opt_extended_dimension = model.extended_dimension_map.get(&cord);
                        //
                        let item_idx = (0..expandable_row_indexes.len())
                            .filter(|i|expandable_row_indexes[*i] == idx).next().unwrap();

                        let replace_cells = cross_item_list.iter().map(|cross|Cell{
                            text: Some(cross[item_idx].text.as_ref().unwrap_or(&cross[item_idx].id).to_string()),
                            dimensions: Some(build_expand_cell_dimensions(opt_cell.unwrap(),&cross[item_idx],&opt_extended_dimension)),
                            cross_item: None
                        }).collect::<Vec<Cell>>();
                       data_grid.replace_cols(idx,col_idx,replace_cells);
                    }else{
                        let replace_cells = cross_item_list.iter().map(|_|Cell{
                            text: None,
                            dimensions: None,
                            cross_item: None
                        }).collect::<Vec<Cell>>();
                        data_grid.replace_cols(idx,col_idx,replace_cells);
                    }
                });
            }
        }

        data_grid.columns = grid_col_count;
    }

    ///
    /// 主栏维度项展开
    ///
    fn expand_grid_by_main(self: &Self, data_grid: &mut DataGrid) {
        let row_count = self.rows.as_ref().unwrap().len();
        for idx in data_grid.frozen_rows..row_count {
            //从后向前展开行
            let row_idx = row_count - idx - 1 + data_grid.frozen_rows;
            let row = &data_grid.rows[row_idx];
            let cell_dimensions = data_grid.find_cell_dimensions(row_idx,0);

            match cell_dimensions {
                None => {}
                Some((multi_item_dimensions,_one_item_dimensions)) => {
                    let i = 1;
                    let mut expand_row_count = multi_item_dimensions.iter().map(|d|d.items.as_ref().unwrap().len())
                        .fold(i,|a,b|a*b);

                    if expand_row_count>1{
                        //维度项交叉
                        let cross_item_list = dimensions_item_cartesian(&multi_item_dimensions,false);
                        expand_row_count = cross_item_list.len();

                        let expand_rows = (0..expand_row_count)
                            .map(|idx| Row {
                                height: row.height.clone(),
                                cells: expand_row_cells(row,&cross_item_list[idx]),
                            }).collect::<Vec<Row>>();
                        //行替换
                        data_grid.replace_rows(row_idx,expand_rows);
                    }
                }
            }
        }
    }
}

///
///
///
fn build_expand_cell_dimensions(cell:&Cell, expanded_item:&Item, extend_dimensions:&Option<&&Vec<Dimension>>) ->Vec<Dimension>{
    let mut dimensions = cell.dimensions.as_ref().unwrap().iter()
        .map(|dim|{
            let expanded_item_dim_id = expanded_item.dim_id.as_ref().unwrap();
            let expanded_item_dim_name = expanded_item.dim_name.as_ref().unwrap();
            if &dim.id == expanded_item_dim_id.as_str(){
                Dimension{
                    id:dim.id.to_string(),
                    name: dim.name.to_string(),
                    table_name: None,
                    dimension_type: dim.dimension_type.clone(),
                    text: None,
                    items: Some(vec![Item::new(&expanded_item.id)
                        .set_dim_id(expanded_item_dim_id.to_string())
                        .set_dim_name(expanded_item_dim_name.to_string())
                        .set_text(expanded_item.text.as_ref().unwrap_or(expanded_item_dim_id).to_string())
                        ])
                }
            }else{
                dim.clone()
            }
        }).collect::<Vec<Dimension>>();

    match extend_dimensions{
        None => {}
        Some(e) => {
            dimensions.splice(0..0,e.iter().map(|d|d.clone()).collect::<Vec<Dimension>>());
        }
    }

    dimensions
}
///
/// 行展开单元格
///
fn expand_row_cells(row:&Row,cross_item:&Vec<Item>)->Vec<Cell>{

    (0..row.cells.len()).map(|jdx|{
       if jdx ==0 {

           let dimensions = cross_item.iter().map(|item|
               Dimension::new(item.dim_id.as_ref().unwrap().as_str(),item.dim_name.as_ref().unwrap().as_str())
                   .set_items(vec![Item::new(item.id.as_str()).set_text(item.text.as_ref().unwrap_or(&item.id).to_string())])).collect::<Vec<Dimension>>();

           Cell{
               text: match jdx { 0=>Some(cross_item.iter().map(|item|item.text.as_ref().unwrap_or(&item.id)).join("/")),_=>None },
               cross_item:None,
               dimensions:Some(dimensions),
           }

       }else{
           Cell{
               text: match jdx { 0=>Some(cross_item.iter().map(|item|item.id.to_string()).join("/")),_=>None },
               cross_item:None,
               dimensions:None,
           }
       }

    }).collect::<Vec<Cell>>()
}

///
///
///
fn dimensions_item_cartesian<'a>(dimensions:&'a Vec<&'a Dimension>,need_group_sum:bool) -> Vec<Vec<Item>> {
    (0..dimensions.len()).map(|idx|{
        let dimension = dimensions[idx];
        let mut items = dimension.items.as_ref().unwrap().iter().map(|item| {
            let mut dim_item = item.clone();
            //设置dimension id
            dim_item.dim_id = Some(dimension.id.to_string());
            dim_item.dim_name = Some(dimension.name.to_string());
            dim_item
        }).collect::<Vec<Item>>();
        if need_group_sum && idx>0{
            items.splice(0..0,vec![Item::new("groupSum")]);
        }
        items
    }).multi_cartesian_product().collect::<Vec<Vec<Item>>>()
}
///
///
///
fn render_cell(cell:&Cell,dimension_items_map:&HashMap<String,Vec<Item>>)->Cell{
    match &cell.dimensions {
        None => cell.clone(),
        Some(dimensions) => {
            Cell{
                text:cell.text.clone(),
                dimensions:Some(render_cell_dimensions(dimensions,dimension_items_map)),
                cross_item:None
            }
        }
    }
}

///
///
///
fn render_cell_dimensions(dimensions:&Vec<Dimension>,dimension_items_map:&HashMap<String,Vec<Item>>)->Vec<Dimension>{
    dimensions.iter().map(|dimension|{
        if dimension_items_map.contains_key(&dimension.id)
            && (dimension.items.is_none() || dimension.items.as_ref().unwrap().is_empty()){
            let items = dimension_items_map.get(&dimension.id);

            match items {
                None => {}
                Some(items) => {
                    return Dimension {
                        id: dimension.id.to_string(),
                        name: dimension.name.to_string(),
                        table_name: dimension.table_name.clone(),
                        dimension_type: dimension.dimension_type.clone(),
                        text: dimension.text.clone(),
                        items: Some(items.iter().map(|item|item.clone()).collect::<Vec<Item>>())
                    }
                }
            }
        }
        dimension.clone()
    }).collect::<Vec<Dimension>>()

}


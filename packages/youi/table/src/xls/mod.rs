use std::fs::File;
use std::io::BufReader;
use grid::Grid;
use crate::{Row, Table,Cell, Area, ColModel};
use youi_calamine::{Error, Reader, Range, DataType, open_workbook_auto, Sheets};
use crate::formula::Formula;

///
/// 从 xlsx 文件解析 table集合
///
pub fn read_tables(path:&str) -> Result<Vec<Table>,Error>{
    let mut sheets = open_workbook_auto(path)?;

    Ok(sheets.worksheets().iter().map(|(name,r)|{
        parse_sheet(&name,&r,&mut sheets).unwrap()
    }).collect::<Vec<Table>>())
}
///
///
///
fn parse_sheet(name:&str,r:&Range<DataType>, sheets: &mut Sheets<BufReader<File>>) ->Result<Table,Error>{
    let table_rows:Vec<Row> = r.rows().map(|_|{
        Row{
            height:None,
            cells:vec![]
        }
    }).collect::<Vec<Row>>();

    let start = r.start().unwrap();

    let row_start = start.0 as usize;
    let col_start = start.1 as usize;
    let row_count = r.height();
    let col_count = r.width();
    let mut grid:Grid<Cell> = Grid::new(row_count+row_start,r.width()+col_start);

    (0..row_count).for_each(|row_idx|{
        (0..col_count).for_each(|col_idx|{
            let opt = grid.get_mut(row_idx+row_start,col_idx+col_start);
            match opt {
                None => {}
                Some(cell) => {
                    cell.text = Some(r.get((row_idx,col_idx)).unwrap().clone())
                }
            }
        });
    });

    //根据文字自动计算列宽度
    let col_models = (0..r.width()).map(|_idx|{
        ColModel{width:Some(100.)}
    }).collect::<Vec<ColModel>>();

    let mut merged_cells = vec![];
    //合并单元格
    match sheets {
        Sheets::Xls(xls) => {
            let merged = (*xls).worksheet_merge_cells(name).unwrap()?;
            merged_cells = merged.iter().map(|d|to_area(d.start(),d.end())).collect::<Vec<Area>>()
        }
        Sheets::Xlsx(xlsx) => {
            let merged = (*xlsx).worksheet_merge_cells(name).unwrap()?;
            merged_cells = merged.iter().map(|d|to_area(d.start(),d.end())).collect::<Vec<Area>>()
        }
        Sheets::Xlsb(_) => {}
        Sheets::Ods(_) => {}
    }

    //公式
    let formulas = parse_formula(name,sheets);

    Ok(Table{
        name:name.replace("-","_"),
        title:Some(name.to_string()),
        rows:Some(table_rows),
        merged_cells:Some(merged_cells),
        col_models:Some(col_models),
        grid:Some(grid),
        formulas:Some(formulas)
    })
}
///
///
///
fn parse_formula(name:&str,sheets: &mut Sheets<BufReader<File>>)->Vec<Formula>{
    match sheets.worksheet_formula(name){
        Ok(range) => {
            let start = range.start().unwrap();
            range.used_cells().map(|(row,col,f)|{
                let mut formula = Formula::new(name,(row as u32+start.0+1,col as u32+start.1+1),
                             &f.get_formula_expression());
                let ref_area = f.get_ref_area();
                if !ref_area.is_empty(){
                    formula.set_ref_area(ref_area);
                }
                formula
            }).collect::<Vec<Formula>>()
        }
        Err(_) => vec![]
    }
}
///
///
///
fn to_area(start:(u32,u32),end:(u32,u32))->Area{
    Area{
        start_row:start.0,
        end_row: end.0,
        start_col: start.1,
        end_col:  end.1
    }
}


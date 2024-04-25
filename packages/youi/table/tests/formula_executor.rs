use grid::grid;
use rhai::{Engine, exported_module};
use youi_table::{TableList, Table, Cell, Formula};
use youi_table::xls::read_tables;
use youi_table::formula::{grid_module,FormulaExecutor};

///
/// 测试excel文件读取的公式
///
#[test]
pub fn test_excel_executor(){

    let engine = create_engine();

    let result_table_list = read_tables("../../demo/data/table/formula.xlsx").unwrap();

    let mut executor = FormulaExecutor::new(TableList::new(result_table_list));
    //
    let start = std::time::SystemTime::now();
    executor.execute(&engine);
    print!("公式执行耗时：{:?}毫秒", std::time::SystemTime::now().duration_since(start).unwrap().as_millis());

}
///
/// 测试多级公式执行
///
#[test]
pub fn test_formula_executor(){
    let engine = create_engine();
    let tables = TableList::new(vec![
        Table::new("grid1".to_string(),grid![
            [Cell::from("0100"),Cell::from(21.0),Cell::from(21.5),Cell::from(99.1),Cell::from(99.0)]
            [Cell::from("0200"),Cell::from(31.0),Cell::from(32.10),Cell::from(99.5),Cell::from(99.0)]
            [Cell::from("0300"),Cell::from(41.0),Cell::from(42.10),Cell::from(103.0),Cell::from(103.0)]
            [Cell::from("0400"),Cell::from(61.0),Cell::from(62.01),Cell::from(101.0),Cell::from(101.0)]
        ],vec![

        ]),
        Table::new("grid2".to_string(),grid![
            [Cell::from("0100"),Cell::from(1.1),Cell::from(92.0),Cell::from(0.),Cell::from(0.0),Cell::from(0.0),Cell::from(0.0)]
            [Cell::from("0200"),Cell::from(1.2),Cell::from(92.1),Cell::from(0.0),Cell::from(0.0),Cell::from(0.0),Cell::from(0.0)]
            [Cell::from("0300"),Cell::from(1.3),Cell::from(92.3),Cell::from(0.0),Cell::from(0.0),Cell::from(0.0),Cell::from(0.0)]
            [Cell::from("0400"),Cell::from(1.4),Cell::from(92.4),Cell::from(0.0),Cell::from(0.0),Cell::from(0.0),Cell::from(0.0)]
        ],vec![
            Formula::new("grid2",(1,4),"VLOOKUP(A1,'grid1'!A1:E4,2,0)"),
            Formula::new("grid2",(2,4),"VLOOKUP(A2,'grid1'!A1:E4,2,0)"),
            Formula::new("grid2",(3,4),"VLOOKUP(A3,'grid1'!A1:E4,2,0)"),
            Formula::new("grid2",(4,4),"VLOOKUP(A4,'grid1'!A1:E4,2,0)"),
            Formula::new("grid2",(1,5),"VLOOKUP(A1,'grid1'!A1:E4,3,0)"),
            Formula::new("grid2",(2,5),"VLOOKUP(A2,'grid1'!A1:E4,3,0)"),
            Formula::new("grid2",(3,5),"VLOOKUP(A3,'grid1'!A1:E4,3,0)"),
            Formula::new("grid2",(4,5),"VLOOKUP(A4,'grid1'!A1:E4,3,0)"),

            Formula::new("grid2",(1,6),"B1*D1"),
            Formula::new("grid2",(2,6),"B2*D2"),
            Formula::new("grid2",(3,6),"B3*D3"),
            Formula::new("grid2",(4,6),"B4*D4"),

            Formula::new("grid2",(1,7),"F1/100"),
            Formula::new("grid2",(2,7),"F2/100"),
            Formula::new("grid2",(3,7),"F3/100"),
            Formula::new("grid2",(4,7),"F4/100"),
        ]),
    ]);

    let mut executor = FormulaExecutor::new(tables);
    //
    let start = std::time::SystemTime::now();
    let result = executor.execute(&engine);
    println!("{:?}",result);
    println!("公式执行耗时：{:?}毫秒", std::time::SystemTime::now().duration_since(start).unwrap().as_millis());
}

///
///
///
fn create_engine()->Engine{
    youi_test::enable_test_log();

    let mut engine:Engine  = Engine::new_raw();

    let module = exported_module!(grid_module);
    engine.register_global_module(module.into());
    engine
}

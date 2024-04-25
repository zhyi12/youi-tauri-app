use std::fs::File;
use regex::Regex;
use youi_report::{ReportModel, ReportWidget};

#[test]
pub fn report(){

    // let path = "../../demo/data/report/report_model.json";
    // let file = File::open(path).unwrap();
    // let model = serde_json::from_reader::<File, ReportModel>(file).unwrap();
    // //
    // let cube_list = model.data_cube_list();

}

///
///
///
fn mock_report_model(rows:usize,cols:usize)->ReportModel{
    ReportModel::from(ReportWidget::new(rows,cols))
}
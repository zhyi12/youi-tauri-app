use serde::{Deserialize, Serialize};

use youi_report::{Dataset, Group, MeasureItem, ReportModel, ReportWidget};
use youi_report::dimension::{Dimension, Measure};
use youi_report::item::Item;
use crate::ChartOption;

///
/// 图表
///
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Chart{

    ///
    /// 报表数据集
    ///
    dataset:Option<Dataset>,

    ///
    /// 分组维度
    ///
    group_name:String,

    ///
    /// 计量项
    ///
    measure_items:Vec<MeasureItem>,

    ///
    ///
    ///
    pub(crate) option:ChartOption,

}


impl Chart {
    ///
    /// 创建报表模型
    ///
    pub fn create_report_model(&self) -> ReportModel{
        // 主栏1列，宾栏1行
        let mut widget = ReportWidget::new(2,2);

        // 主栏元数据
        widget.add_dimension(1,0,self.group_dimension());
        // 宾栏元数据
        widget.add_dimension(0,1,self.measure_dimension());

        let mut model = ReportModel::from(widget);

        // 设置数据集
        if self.dataset.is_some(){
            model.set_dataset(self.dataset.as_ref().unwrap().clone());
        }

        model
    }

}

impl Chart {

    ///
    ///
    ///
    fn group_dimension(&self)->Dimension{

        let group =Group::new(&self.group_name,"table", &self.group_name,vec![]);

        Dimension::Group(group)
    }

    ///
    ///
    ///
    fn measure_dimension(&self)->Dimension{
        let items = self.measure_items.iter().map(|item|Item::Measure(item.clone())).collect::<Vec<Item>>();
        Dimension::Measure(Measure::from(items))
    }

}
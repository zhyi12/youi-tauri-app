use itertools::Itertools;
use std::fmt::Write;
use serde::{Serialize, Deserialize};
use crate::dsl::DslNode;
use crate::measure::MeasureItem;
use crate::step::StepInfo;

///
/// 汇总
///
#[derive(Serialize, Deserialize,Debug)]
#[serde(rename_all = "camelCase")]
pub struct Agg{
    ///
    ///
    ///
    #[serde(flatten)]
    pub(crate) info:StepInfo,

    ///
    /// 分组列名集合
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    group_names:Option<Vec<String>>,

    ///
    /// 计量项集合
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    measure_items:Option<Vec<MeasureItem>>
}

///
///
///
impl DslNode for Agg {

    ///
    ///
    ///
    fn to_dsl(&self,input_step_id:Option<&str>) -> crate::error::QueryResult<String> {
        // 检查输入
        self.check_input(input_step_id,self.info.text.as_ref().unwrap_or(&"Agg".to_string()))?;
        let input_id = input_step_id.unwrap();

        Ok(match (&self.measure_items,&self.group_names){
            (Some(measure_items),Some(group_names))=>{

                let unique_measure_items = measure_items.iter().unique_by(|f|format!("{}_{}",f.name,f.aggregate))
                    .collect::<Vec<&MeasureItem>>();

                let unique_group_names = group_names.iter()
                    .unique().collect::<Vec<&String>>();

                let mut measure_expr = unique_measure_items.iter()
                    .fold(String::new(),|mut output,c|{
                        let _ = write!(output,"col(\"{}\").{}().alias(\"{0}_{1}\"),",c.name,c.aggregate);
                        output
                    });
                measure_expr.pop();
                // group sort
                let mut orders_expr = unique_group_names.iter().fold(String::new(), |mut output, b| {
                    let _ = write!(output, "col(\"{}\"),",b);
                    output
                });
                orders_expr.pop();

                format!("let {} = {}.agg(\"{}\",[{}]).sort_by_exprs([{}],\"{}\")",self.build_df_id(&self.info.id),self.build_df_id(input_id),
                        unique_group_names.iter().join(","),
                        measure_expr,orders_expr,
                        unique_group_names.iter().map(|n|"false".to_string()).join(","))
            },
            _=>"".to_string()
        })
    }
}
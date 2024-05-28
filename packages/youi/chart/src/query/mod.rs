use polars_core::prelude::DataFrame;
use crate::Chart;

pub struct ChartQuery<'a>{

    chart:&'a mut Chart,
    ///
    /// 数据集
    ///
    df:Option<&'a mut DataFrame>,

}

impl<'a> From<&'a mut Chart> for ChartQuery<'a>{
    fn from(chart: &'a mut Chart) -> Self {
        Self{
            chart,
            df:None
        }
    }
}

impl<'a> ChartQuery<'a>{

    pub fn set_df(&mut self,df:&'a mut DataFrame){
        self.df = Some(df);
    }
}

impl<'a> ChartQuery<'a> {

    ///
    ///
    ///
    pub fn replace_dataset(&mut self){
        match &self.df {
            None => { }
            Some(df) => {
                let mut df = (*df).clone();
                self.chart.option.add_df(&mut df)
            }
        };
    }

}

use regex::{Captures, Regex, Replacer};
use serde::{Deserialize, Serialize};

///
/// 和计量组合的分组项
/// 1、时间框架（组合时间框架计量列）
/// 2、指标分组项 （）
///
#[derive(Serialize, Deserialize,Clone,Debug,PartialEq,PartialOrd,Eq)]
pub struct MeasurePart{
    ///
    ///
    ///
    pub(crate) id:String,

    ///
    ///
    ///
    pub(crate) text:String,

    ///
    /// 指定组合的计量
    ///
    pub(crate) measure_item_name:Option<String>,

    ///
    /// 计量计算表达式
    /// col(\"{measure}_001\")
    /// (col(\"{measure}\") - col(\"{measure}_001\"))/col(\"{measure}_001\")*100
    /// col(\"{measure}_duty\")
    ///
    pub(crate) expression:Option<String>

}

impl MeasurePart {
    ///
    /// 计量计算表达式
    ///
    pub fn build_expression(&self,measure_name:&str)->String{
        let empty = "".to_string();
        let expression = self.expression.as_ref()
            .unwrap_or(&empty);
        let agg_reg:Regex  = Regex::new("_\\w+$").unwrap();
        let column_name = agg_reg.replace(measure_name,"").to_string();
        let agg = &measure_name[column_name.len()+1..measure_name.len()];

        if expression.is_empty(){
            format!("col(\"{}_{}_{}\")",column_name,self.id,agg)
        }else{
            //计量名替换
            let reg = Regex::new("col\\(\"\\{measure\\}[_]?(?<part>\\w+)?\"\\)").unwrap();
            reg.replace_all(expression,MeasureReplacer{
                column_name:&column_name,
                agg
            }).to_string()
        }
    }
}

struct MeasureReplacer<'a>{
    column_name:&'a str,
    agg:&'a str,
}

impl<'a> Replacer for MeasureReplacer<'a>{
    ///
    ///
    ///
    fn replace_append(&mut self, caps: &Captures<'_>, dst: &mut String) {
        dst.push_str("col(\"");
        dst.push_str(self.column_name);
        match caps.name("part") {
            None => {}
            Some(part) => {
                dst.push_str("_");
                dst.push_str(part.as_str());
            }
        };
        dst.push_str("_");
        dst.push_str(self.agg);
        dst.push_str("\")");
    }
}


#[test]
pub fn measure_replace_test(){
    let expression = "(col(\"{measure}\") - col(\"{measure}_001\"))/col(\"{measure}_001\")*100";
    //"\{measure\}[_\\w0-9]+""
    let reg = Regex::new("col\\(\"\\{measure\\}[_]?(?<part>\\w+)?\"\\)").unwrap();

    let expr = reg.replace_all(expression,MeasureReplacer{
        column_name:"employee",
        agg:"sum"
    }).to_string();
    println!("measure expr {expr}");
}
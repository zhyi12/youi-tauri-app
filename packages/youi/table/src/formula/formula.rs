use serde::{Serialize,Deserialize};
///
/// 公式
///
#[derive(Debug,Serialize, Deserialize,Clone)]
pub struct Formula{
    pub(crate) id:String,
    ///
    ///
    ///
    pub(crate) sheet_name:String,
    ///
    /// 表格公式
    ///
    pub(crate) expression:String,

    ///
    /// 所在位置
    ///
    pub(crate) index:(u32,u32),

    ///
    /// 执行公式
    ///
    pub(crate) exec_expression:Option<String>,

    ///
    /// 公式扩展的区域（对应excel的公式拖动扩展）
    ///
    pub(crate) ref_area:Option<String>,
}

impl Formula {
    ///
    ///
    ///
    pub fn new(sheet_name:&str,index:(u32,u32),expression:&str)->Self{
        Self{
            id: formula_id(sheet_name,index),
            sheet_name: sheet_name.to_string(),
            index,
            expression: expression.to_string(),
            exec_expression: None,
            ref_area:None
        }
    }
}
///
///
///
pub fn formula_id(sheet_name:&str,index:(u32,u32))->String{
    format!("f_{}.{}_{}",sheet_name,index.0,index.1)
}

impl Formula {
    ///
    ///
    ///
    pub fn set_exec_expression(&mut self,exec_expression:&str){
        self.exec_expression = Some(exec_expression.to_string());
    }
    ///
    ///
    ///
    pub fn set_ref_area(&mut self,ref_area:String){
        self.ref_area = Some(ref_area);
    }
}

#[test]
pub fn test_formula(){

    Formula::new("grid2",(2,2),"VLOOKUP(grid2!A2,grid1!A2:C3,2,2)+B2+SUM(B2:B3)");

    //GET(3,2)
    //AREA(1,2,1,2)
    // "VLOOKUP(grid2!A2,grid1!A2:C3,2,2)+B2+SUM(B2:B3)",
    // "VLOOKUP(grid2!A2,grid1!A2:C3,2,2)",
    // "B2+B3",
    // "SUM(B2:B3)",
    // "COUNTA(A2:C3)",
    // "AVERAGE(B3,C4,C2:C3)",
    // "IF(A2==\"0600\",B2,B3)",
    // "IF(B2==1,B2,B3)",
    // "IF(B2==2,B2,B3)",
    // "CONCAT(1,\"My\",\"Book\",B3)",
    // "ROUND(C2)+ROUND(C3)",
    // "ROUND(C2+C3)",
    // "LEFT(\"my name is \",2)",
    // "LEFT(A2,3)",
    // "RIGHT(A2,3)",
    // "grid2.SET(2,2,A2)"
}
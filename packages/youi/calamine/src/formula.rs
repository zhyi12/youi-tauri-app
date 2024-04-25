use crate::CellType;


#[derive(Debug,Clone,Default,PartialEq)]
pub struct Formula{

    pub(crate) expression:String,

    ///
    /// G3:G66
    ///
    pub(crate) ref_area:Option<String>,

}

impl Formula {

    pub fn new(expression:String)->Self{
        Self{
            expression,
            ref_area:None,
        }
    }
}

impl Formula {
    ///
    ///
    ///
    pub fn formula(mut self,ref_area:String)->Self{
        self.ref_area = Some(ref_area);
        self
    }
}

impl Formula {

    pub fn get_formula_expression(&self)->String{
        self.expression.replace("_xlfn.","")
    }

    pub fn get_ref_area(&self)->String{
        match &self.ref_area {
            None => "".to_string(),
            Some(ref_area) => ref_area.to_string()
        }
    }
}

impl CellType for Formula {}
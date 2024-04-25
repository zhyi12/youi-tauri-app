mod measure;
mod dimension;

///
///
///
#[derive(Debug,Clone,PartialEq, Eq,Ord, PartialOrd)]
pub struct ColumnSelect {
    ///
    /// 列ID
    ///
    pub(crate) id:String,
    ///
    /// 列名
    ///
    pub(crate) name:String,

    ///
    /// 列表达式
    ///
    pub(crate) column_expression:Option<String>,
    ///
    /// <值、表达式>
    ///
    value_mapping:Vec<(String,String)>,

    ///
    /// 计量表达式
    ///
    pub(crate) measure_expression:Option<String>,
}

impl ColumnSelect {

    pub fn new(id:String,name:String,column_expression:Option<String>,value_mapping:Vec<(String,String)>)->Self{
        Self{
            id,
            name,
            column_expression,
            value_mapping,
            measure_expression:None
        }
    }
}

impl ColumnSelect {

    pub fn id(&self)->&str{
        &self.id
    }

    pub fn id_clone(&self)->String{
        self.id.to_string()
    }

    pub fn name_clone(&self)->String{
        self.name.to_string()
    }

    pub fn column_expression(&self)->Option<&String>{
        self.column_expression.as_ref()
    }
    ///
    ///
    ///
    pub fn has_value_mapping(&self)->bool{
        !self.value_mapping.is_empty()
    }

    pub fn has_column_expression(&self)->bool{
        self.column_expression.is_some()
    }
    ///
    ///
    ///
    pub fn value_mapping(&self)->&Vec<(String,String)>{
        &self.value_mapping
    }
    ///
    ///
    ///
    pub fn select_name(&self) ->String{
        if self.has_value_mapping() || self.column_expression.is_some(){
            self.id.to_string()
        }else{
            self.name_clone()
        }
    }
}

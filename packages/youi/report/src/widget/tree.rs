use crate::tree::LevelNode;

///
/// 树节点
///
#[derive(Debug)]
pub struct CellNode{
    row_index:u32,
    col_index:u32,
    level:u32,
    text:String
}

impl CellNode {

    pub fn new(row_index:u32,
               col_index:u32,
               level:u32,
               text:String)->Self{
        Self{
            row_index,
            col_index,
            level,
            text
        }
    }
}

impl CellNode {

    pub fn row_index(&self)->u32{
        self.row_index
    }

    pub fn col_index(&self)->u32{
        self.col_index
    }

}
///
///
///
impl LevelNode for CellNode {

    fn get_id(&self) -> String {
        format!("C_{}_{}",self.row_index,self.col_index)
    }
    ///
    ///
    ///
    fn get_level(&self) -> u32 {
        self.level
    }

    fn get_text(&self)->String{
        self.text.clone()
    }
}
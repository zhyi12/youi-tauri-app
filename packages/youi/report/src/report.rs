use crate::{Report,Cell, ReportQueryModel};

///
///
///
impl Report {

    ///
    /// 执行立方体数据查询
    ///
    pub fn build_query_model(self:&Self)->ReportQueryModel{
        //构建查询模型
        ReportQueryModel::from(self)
    }

    ///
    /// 根据行列定位查找单元格
    ///
    pub fn find_cell(self:&Self, row_index:usize, column_index:usize) -> Option<&Cell> {
        match &self.rows {
            None => {}
            Some(r) => {
                if r.len() > row_index {
                    let row = &r[row_index];
                    if row.cells.len() > column_index{
                        return Some(&row.cells[column_index]);
                    }
                }
            }
        }
        None
    }

}

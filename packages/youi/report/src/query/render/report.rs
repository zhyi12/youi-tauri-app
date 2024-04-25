use grid::Grid;
use crate::cell::Cell;
use crate::error::ReportResult as Result;
use crate::query::widget::ReportWidgetExecutor;

///
///
///
pub struct ReportWidgetRender<'a>{
    ///
    ///
    ///
    executor:&'a ReportWidgetExecutor<'a>,

    ///
    /// 输出表格
    ///
    grid:Grid<Cell>,
}

impl<'a> From<&'a ReportWidgetExecutor<'a>> for ReportWidgetRender<'a>{
    fn from(executor: &'a ReportWidgetExecutor) -> Self {
        //复制表格
        let grid = executor.widget.table.grid.clone();
        Self{
            executor,
            grid
        }
    }
}

impl<'a>  ReportWidgetRender<'a>{

    ///
    ///
    ///
    pub fn render(&mut self)->Result<()>{

        //行、列扩展

        Ok(())
    }



}
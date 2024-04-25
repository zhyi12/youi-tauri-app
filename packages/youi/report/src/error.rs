
use thiserror::Error;

pub type ReportResult<T> = Result<T, ReportError>;

#[derive(Error, Debug)]
pub enum ReportError {

    #[error("{0}列在数据表{1}中不存在.")]
    ColumnNotFount(String,String),

    ///
    /// 列表达式解析异常
    ///
    #[error(transparent)]
    ExpressionParseError(#[from] rhai::ParseError),

    #[error("函数名应该是and或or，当前为{0}.")]
    NotConnectFunc(String)
}

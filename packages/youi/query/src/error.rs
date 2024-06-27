use thiserror::Error;

pub type QueryResult<T> = Result<T, QueryError>;

#[derive(Error, Debug)]
pub enum QueryError {

    #[error("{0}前置步骤不能为空.")]
    NeedPrevStep(String),

    #[error(transparent)]
    FmtWriteError(#[from] std::fmt::Error),
}

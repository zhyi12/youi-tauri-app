use thiserror::Error;

pub type ServiceResult<T> = Result<T,ServiceError>;

#[derive(Error, Debug)]
pub enum ServiceError {
    ///
    ///
    ///
    #[error(transparent)]
    Sql(#[from] sqlx::Error),

}
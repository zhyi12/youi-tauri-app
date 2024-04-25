use thiserror::Error;

pub type Result<T> = std::result::Result<T, Arrow2DestinationError>;

#[derive(Error, Debug)]
pub enum Arrow2DestinationError {
    // #[error(transparent)]
    // ArrowError(#[from] polars_arrow::e),

    #[error(transparent)]
    PolarsError(#[from] polars_core::error::PolarsError),

    #[error(transparent)]
    ConnectorXError(#[from] crate::errors::ConnectorXError),

    /// Any other errors that are too trivial to be put here explicitly.
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

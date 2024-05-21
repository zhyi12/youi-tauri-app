

use thiserror::Error;

pub type GeoResult<T> = Result<T,GeoError>;

#[derive(Error, Debug)]
pub enum GeoError {

    #[error(transparent)]
    GeozeroError(#[from] geozero::error::GeozeroError),

}
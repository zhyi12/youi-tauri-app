
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DataframeError {

    #[error("{0}文件未找到.")]
    FileNotFound(String),

}

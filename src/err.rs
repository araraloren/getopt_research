
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("invalid option string: `{0}`")]
    InvalidOptionStr(String),

    #[error("invalid option type: `{0}`")]
    InvalidOptionType(String),
}

use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("invalid option string: `{0}`")]
    InvalidOptionStr(String),

    #[error("invalid option type: `{0}`")]
    InvalidOptionType(String),

    #[error("invalid option value `{0}`: `{1}`")]
    InvaldOptionValue(String, String),

    #[error("no available argument left")]
    InvalidNextArgument,

    #[error("option type can not be null")]
    NullOptionType,

    #[error("option name can not be null")]
    NullOptionName,

    #[error("given option type not support deactivate style")]
    NotSupportDeactivateStyle,

    #[error("the given type is exists: `{0}`")]
    DuplicateOptionType(String),

    #[error("`{0}` need an argument")]
    ArgumentRequired(String),

    #[error("option `{0}` is force required")]
    OptionForceRequired(String),
}

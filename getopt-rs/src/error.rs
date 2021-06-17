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

    #[error("invalid option id: `{0}`")]
    InvaldOptionId(String),

    #[error("no available argument left")]
    InvalidNextArgument,

    #[error("option type can not be null")]
    NullOptionType,

    #[error("option name can not be null")]
    NullOptionName,

    #[error("utils `{0}` not support deactivate style")]
    UtilsNotSupportDeactivateStyle(String),

    #[error("utils `{0}` not support current type: `{1}`")]
    UtilsNotSupportTypeName(String, String),

    #[error("the given type is exists: `{0}`")]
    DuplicateOptionType(String),

    #[error("`{0}` need an argument")]
    ArgumentRequired(String),

    #[error("option `{0}` is force required")]
    OptionForceRequired(String),

    #[error("need non-option: `{0}`")]
    NonOptionForceRequired(String),

    #[error("invalid callback type for id(`{0}`): `{1}`")]
    InvalidCallbackType(String, String),

    #[error("catch io error: `{0}`")]
    CatchIOError(#[from] std::io::Error),

    #[error("catch error: `{0}`")]
    RaisedError(String),
}

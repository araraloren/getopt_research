
use std::any::Any;
use std::fmt::Debug;

use crate::id;
use crate::error::Error;
use crate::error::Result;

#[derive(Debug, Clone)]
pub enum Style {
    /// Boolean style is the option looks like
    /// "-o", "--option", "-option"
    Boolean,

    /// Argument style is the option looks like
    /// "-o 1", "-o=1", "--option=1", "-option=1"
    Argument,

    /// Multiple style are some option looks like
    /// "-abcd" means "-a", "-b", "-c", "-d"
    Multiple,

    /// Not a option style
    NonOption,

    Null,
}


#[derive(Debug)]
pub enum OptValue {
    /// Signed integer value
    Int(i64),

    /// Unsigned integer value
    Uint(u64),

    /// Double float value
    Flt(f64),

    /// String value
    Str(String),

    /// Boolean value
    Bool(bool),

    /// An vector can hold multiple value
    Arr(Vec<String>),

    /// Any type
    Any(Box<dyn Any>),

    Null,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OptIndex {
    Forward(u64),

    Backward(u64),

    Null
}


/// Type hold specify information for an option type
pub trait Type {
    /// Unique type name for current option type
    fn type_name(&self) -> &str;

    /// Boolean option that initialized with true,
    /// user can set(disable) them by using "-/o"
    fn is_deactivate_style(&self) -> bool;

    /// Is the option need argument
    fn is_need_argument(&self) -> bool;

    /// Convert the type to `Any` which can downcast to real type
    fn as_any(&self) -> &dyn Any;

    /// Retrun true if the option compatible with the style
    fn match_style(&self, style: Style) -> bool;
}

pub trait Identifier {
    /// Get an unique identifier for current option
    fn id(&self) -> id::Identifier;
}

pub trait Name {
    /// Get the name of current option,
    /// it will return "a" for "-a"
    fn name(&self) -> &str;

    /// Get the prefix of current option, 
    /// it will return "-" for "-a"
    fn prefix(&self) -> &str;

    /// Set the name for current option
    fn set_name(&mut self, s: &str);

    /// Set the prefix for current option
    fn set_prefix(&mut self, s: &str);

    /// Return true if name equal
    fn match_name(&self, s: &str) -> bool;

    /// Return true if the prefix equal
    fn match_prefix(&self, s: &str) -> bool;
}

pub trait Alias {
    /// Get the alias for current option
    fn alias(&self) -> Option<&Vec<(&str, &str)>>;

    /// Add an new alias for current option
    fn add_alias(&mut self, prefix: &str, name: &str);

    /// Remove an alias of current option, return true if remove successful
    fn rem_alias(&mut self, prefix: &str, name: &str) -> bool;

    /// Return true if the option has the alias name
    fn match_alias(&self, prefix: &str, name: &str) -> bool;
}

pub trait Optional {
    /// Return true if the option is optional
    fn optional(&self) -> bool;

    /// Set the option to optional
    fn set_optional(&mut self, b: bool);

    /// Return true if the value equal
    fn match_optional(&self, b: bool) -> bool;
}

pub trait Value {
    /// Get current value of the option
    fn value(&self) -> &OptValue;

    /// Get default value of the option
    fn default_value(&self) -> &OptValue;

    /// Set the option value
    fn set_value(&mut self, v: OptValue);

    /// Set the option default value
    fn set_default_value(&mut self, v: OptValue);

    /// Parse the string to option value
    fn parse_value(&self, v: Option<&str>) -> Option<OptValue>;
}

pub trait Index {
    fn index(&self) -> OptIndex;

    fn set_index(&mut self, index: OptIndex);

    fn match_index(&self, total: u64, current: u64) -> bool;
}

pub trait Opt: Type + Identifier + Name + Alias + Optional + Value + Index + Debug { }

impl OptValue {
    pub fn parse_int(s: &str) -> Result<Self> {
        match s.parse::<i64>() {
            Ok(value) => Ok(Self::from_int(value)),
            Err(e) => Err(Error::InvaldOptionValue(String::from(s), format!("{:?}", e))),
        }
    }

    pub fn parse_uint(s: &str) -> Result<Self> {
        match s.parse::<u64>() {
            Ok(value) => Ok(Self::from_uint(value)),
            Err(e) => Err(Error::InvaldOptionValue(String::from(s), format!("{:?}", e))),
        }
    }

    pub fn from_int<T: Into<i64>>(t: T) -> Self {
        Self::Int(t.into())
    }

    pub fn from_uint<T: Into<u64>>(t: T) -> Self {
        Self::Uint(t.into())
    }

    pub fn from_str<T: Into<String>>(t: T) -> Self {
        Self::Str(t.into())
    }

    pub fn from_bool<T: Into<bool>>(t: T) -> Self {
        Self::Bool(t.into())
    }

    pub fn from_arr<T: Into<Vec<String>>>(t: T) -> Self {
        Self::Arr(t.into())
    }

    pub fn from_any<T: Into<Box<dyn Any>>>(t: T) -> Self {
        Self::Any(t.into())
    }

    pub fn null() -> Self {
        Self::Null
    }

    /// Return None if the value is not an OptValue::Int
    pub fn as_int(&self) -> Option<&i64> {
        match self {
            Self::Int(v) => Some(v),
            _ => None,
        }
    }

    /// Return None if the value is not an OptValue::Uint
    pub fn as_uint(&self) -> Option<&u64> {
        match self {
            Self::Uint(v) => Some(v),
            _ => None,
        }
    }

    /// Return None if the value is not an OptValue::Str
    pub fn as_str(&self) -> Option<&String> {
        match self {
            Self::Str(v) => Some(v),
            _ => None,
        }
    }

    /// Return None if the value is not an OptValue::Bool
    pub fn as_bool(&self) -> Option<&bool> {
        match self {
            Self::Bool(v) => Some(v),
            _ => None,
        }
    }

    /// Return None if the value is not an OptValue::Bool and OptValue::Null
    pub fn as_bool_or_null(&self) -> Option<&bool> {
        match self {
            Self::Bool(v) => Some(v),
            Self::Null => Some(&false),
            _ => None
        }
    }

    /// Return None if the value is not an OptValue::Arr
    pub fn as_arr(&self) -> Option<&Vec<String>> {
        match self {
            Self::Arr(v) => Some(v),
            _ => None,
        }
    }

    /// Return None if the value is not an OptValue::Any
    pub fn as_any(&self) -> Option<&Box<dyn Any>> {
        match self {
            Self::Any(v) => Some(v),
            _ => None,
        }
    }

    pub fn is_int(&self) -> bool {
        match self {
            Self::Int(_) => true,
            _ => false,
        }
    }

    pub fn is_uint(&self) -> bool {
        match self {
            Self::Uint(_) => true,
            _ => false,
        }
    }

    pub fn is_str(&self) -> bool {
        match self {
            Self::Str(_) => true,
            _ => false,
        }
    }

    pub fn is_bool(&self) -> bool {
        match self {
            Self::Bool(_) => true,
            _ => false,
        }
    }

    pub fn is_arr(&self) -> bool {
        match self {
            Self::Arr(_) => true,
            _ => false,
        }
    }

    pub fn is_any(&self) -> bool {
        match self {
            Self::Any(_) => true,
            _ => false,
        }
    }

    pub fn is_null(&self) -> bool {
        match self {
            Self::Null => true,
            _ => false,
        }
    }
}

impl Default for OptValue {
    fn default() -> Self {
        OptValue::Null
    }
}

impl OptIndex {
    pub fn new(index: i32) -> Self {
        if index > 0 {
            Self::Forward(index as u64)
        }
        else {
            Self::Backward((-index) as u64)
        }
    }

    pub fn is_null(&self) -> bool {
        match self {
            Self::Null => true,
            _ => false,
        }
    }
}

impl Default for OptIndex {
    fn default() -> Self {
        OptIndex::Null
    }
}

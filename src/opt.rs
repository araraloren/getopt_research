use std::any::Any;
use std::clone::Clone;
use std::convert::Into;
use std::fmt::Debug;

use crate::err::Error;
use crate::proc::Info;

#[derive(Debug, Clone)]
pub enum Style {
    Boolean, // option -a

    Argument, // option has argument -a <param>

    Multiple, // multiple option -abc

    NonOption, // not an option
}

#[derive(Debug)]
pub enum OptValue {
    Int(i64),

    Uint(u64),

    Flt(f64),

    Str(String),

    Bool(bool),

    Arr(Vec<String>),

    Any(Box<dyn std::any::Any>),

    Null,
}

pub trait Type {
    fn type_name(&self) -> &str;

    fn match_style(&self, style: Style) -> bool;

    fn is_deactivate(&self) -> bool;

    fn is_need_argument(&self) -> bool;

    fn as_any(&self) -> &dyn Any;
}

pub trait Identifier {
    fn id(&self) -> u64;
}

pub trait Name {
    fn name(&self) -> &str;

    fn set_name(&mut self, s: &str);

    fn match_name(&self, s: &str) -> bool;
}

pub trait Prefix {
    fn prefix(&self) -> &str;

    fn set_prefix(&mut self, s: &str);

    fn match_prefix(&self, s: &str) -> bool;
}

pub trait Optional {
    fn optional(&self) -> bool;

    fn set_optional(&mut self, b: bool);

    fn match_optional(&self, b: bool) -> bool;
}

pub trait Value {
    fn value(&self) -> &OptValue;

    fn set_value(&mut self, v: OptValue);

    fn parse_value(&self, v: Option<&String>) -> Option<OptValue>;
}

pub trait Opt: Type + Identifier + Name + Prefix + Optional + Value + Debug {}

#[derive(Debug)]
pub struct CommonInfo {
    id: u64,
}

impl CommonInfo {
    pub fn new(id: u64) -> Self {
        Self { id }
    }
}

impl Info for CommonInfo {
    fn id(&self) -> u64 {
        self.id
    }
}

impl OptValue {
    pub fn parse_int(s: &str) -> Result<Self, Error> {
        match s.parse::<i64>() {
            Ok(value) => Ok(Self::from_int(value)),
            Err(e) => Err(Error::InvaldOptionValue(String::from(s))),
        }
    }

    pub fn parse_uint(s: &str) -> Result<Self, Error> {
        match s.parse::<u64>() {
            Ok(value) => Ok(Self::from_uint(value)),
            Err(e) => Err(Error::InvaldOptionValue(String::from(s))),
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

    pub fn as_int(&self) -> Option<&i64> {
        match self {
            Self::Int(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_uint(&self) -> Option<&u64> {
        match self {
            Self::Uint(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_str(&self) -> Option<&String> {
        match self {
            Self::Str(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_bool(&self) -> Option<&bool> {
        match self {
            Self::Bool(v) => Some(v),
            Self::Null => Some(&false),
            _ => None,
        }
    }

    pub fn as_arr(&self) -> Option<&Vec<String>> {
        match self {
            Self::Arr(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_any(&self) -> Option<&Box<dyn Any>> {
        match self {
            Self::Any(v) => Some(v),
            _ => None,
        }
    }

    pub fn is_null(&self) -> bool {
        match self {
            Self::Null => true,
            _ => false,
        }
    }
}

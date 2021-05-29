
use std::fmt::Debug;
use std::any::Any;

use crate::callback::CallbackType;
use crate::id::Identifier as IIdentifier;
use crate::utils::{Utils, CreateInfo};
use crate::proc::Info;
use crate::error::{Error, Result};

/// The option style type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Style {
    /// Boolean style is the option such as "-o", "--option", "-option", "-/a", etc.
    /// It not need argument, and support deactivate style.
    /// The option type [`BoolOpt`](crate::opt::bool::BoolOpt) support current style.
    Boolean,

    /// Argument style is the option such as "-o 1", "-o=1", "--option=1", "-option=1", etc.
    /// It need an argument, and not support deactivate style.
    /// The option type [`IntOpt`](crate::opt::int::IntOpt), [`StrOpt`](crate::opt::str::StrOpt),
    /// etc. support current style.
    Argument,

    /// Multiple style are some option such as "-abcd".
    /// It means Boolean style option "-a", "-b", "-c" and "-d", etc.
    /// All the Boolean style option will support this style.
    Multiple,

    /// NonOption style.
    /// Non-option type [`PosNonOpt`](crate::nonopt::pos::PosNonOpt) support current style.
    Pos,

    /// NonOption style.
    /// Non-option type [`CmdNonOpt`](crate::nonopt::cmd::CmdNonOpt) support current style.
    Cmd,

    /// NonOption style.
    /// Non-option type [`MainNonOpt`](crate::nonopt::main::MainNonOpt) support current style.
    Main,

    Null,
}

/// The option value type.
/// 
/// It support `i64`, `u64`, `String` and `Vec<String>`, etc.
/// Even it support a `Box<dyn Any>` for any type implemented [`Any`](std::any::Any).
/// [`OptValue`](crate::opt::OptValue) implement [`Clone`] for any type expect `Box<dyn Any>`.
/// You need provide a [`CloneHelper`] when your option type target a `Box<dyn Any>` value.
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
    Array(Vec<String>),

    /// Any type
    Any(Box<dyn Any>),

    Null,
}


/// The index of non-option arguments.
///
/// It is base on one, zero is means [`NonOptIndex::AnyWhere`].
/// For example, given command line arguments like `["rem", "-c", 5, "--force", "lucy"]`.
/// After parser process the option `-c` and `--force`, 
/// the left argument is non-option arguments `rem@1` and `lucy@2`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NonOptIndex {
    Forward(i64),

    Backward(i64),

    AnyWhere,

    Null
}

impl NonOptIndex {
    /// Return [`Self::Forward`] if index bigger than zero.
    /// Return [`Self::Backward`] if index little than zero.
    /// Otherwise return [`Self::AnyWhere`].
    pub fn new(index: i64) -> Self {
        if index > 0 {
            Self::Forward(index)
        }
        else if index < 0 {
            Self::Backward(-index)
        }
        else {
            Self::AnyWhere
        }
    }

    pub fn null() -> Self {
        Self::Null
    }

    pub fn calc_index(&self, total: i64) -> Option<i64> {
        match self {
            NonOptIndex::Forward(offset) => {
                if *offset <= total {
                    return Some(*offset)
                }
            }
            NonOptIndex::Backward(offset) => {
                let realindex = total - *offset + 1;
                
                if realindex > 0 {
                    return Some(realindex);
                }
            }
            NonOptIndex::AnyWhere => {
                return Some(0);
            }
            _ => { }
        }
        None
    }

    pub fn is_null(&self) -> bool {
        match self {
            Self::Null => true,
            _ => false,
        }
    }
}

impl Default for NonOptIndex {
    fn default() -> Self {
        Self::Null
    }
}

/// Currently `OptionInfo` only hold a option identifier.
/// `Parser` can get the option from `Set` using this identifier.
#[derive(Debug)]
pub struct OptionInfo {
    id: IIdentifier,
}

impl OptionInfo {
    pub fn new(id: IIdentifier) -> Self {
        Self {
            id
        }
    }
}

impl Info for OptionInfo {
    fn id(&self) -> IIdentifier {
        self.id
    }
}

/// Some specify interface of an option type.
///
/// For example,
/// ```no_run
/// use getopt_rs::opt::*;
/// use getopt_rs::error::Result;
/// use std::any::Any;
/// 
/// struct O;
///
/// impl Type for O {
///     fn type_name(&self) -> &str {
///         "o"
///     }
///
///     fn is_deactivate_style(&self) -> bool { true }
///
///     fn is_style(&self, style: Style) -> bool {
///         style == Style::Boolean
///     }
/// 
///     fn check(&self) -> Result<bool> {
///         Ok(true)
///     }
/// 
///     fn as_any(&self) -> &dyn Any {
///         self
///     }
/// }
///
/// let opt = O;
///
/// assert_eq!(opt.type_name(), "o");
/// assert_eq!(opt.is_deactivate_style(), true);
/// assert_eq!(opt.is_style(Style::Boolean), true);
///
/// ```
pub trait Type {
    /// Unique type name of current option type
    fn type_name(&self) -> &str;

    /// Boolean option that initialized with true,
    /// user can set(disable) them by using "-/o"
    fn is_deactivate_style(&self) -> bool;

    /// Retrun true if the option compatible with the style
    fn is_style(&self, style: Style) -> bool;

    /// Check if the option is valid after parsed
    fn check(&self) -> Result<bool>;

    fn as_any(&self) -> &dyn Any;
}

/// The unique identifier interface of an option.
///
/// For example,
/// ```no_run
/// use getopt_rs::opt::*;
/// use getopt_rs::id::Identifier as IIdentifier;
/// 
/// struct O(pub IIdentifier);
///
/// impl Identifier for O {
///     fn id(&self) -> IIdentifier {
///         self.0.clone()
///     }
///
///     fn set_id(&mut self, id: IIdentifier) {
///         self.0 = id;
///     }
/// }
///
/// let mut opt = O(IIdentifier::new(42));
///
/// assert_eq!(opt.id(), IIdentifier::new(42));
/// opt.set_id(IIdentifier::new(1));
/// assert_eq!(opt.id(), IIdentifier::new(1));
/// ```
pub trait Identifier {
    /// Get an unique identifier of current option
    fn id(&self) -> IIdentifier;

    /// Set identifier to `id`
    fn set_id(&mut self, id: IIdentifier);
}

/// The callback interface of an option.
///
/// For example,
/// ```no_run
/// use getopt_rs::opt::*;
/// use getopt_rs::callback::*;
/// 
/// struct O(pub CallbackType, pub bool);
/// 
/// impl Callback for O {
///     fn callback_type(&self) -> CallbackType {
///         self.0.clone()
///     }
///     
///     fn set_callback_type(&mut self, callback_type: CallbackType) {
///         self.0 = callback_type
///     }
/// 
///     fn is_need_invoke(&self) -> bool {
///         self.1
///     }
/// 
///     fn set_need_invoke(&mut self, invoke: bool) {
///         self.1 = invoke;
///     }
/// }
/// 
/// let mut opt = O(CallbackType::Value, false);
/// 
/// assert_eq!(opt.callback_type(), CallbackType::Value);
/// assert_eq!(opt.is_need_invoke(), false);
/// 
/// // Something happend, such as option is set by user.
/// opt.set_need_invoke(true);
/// 
/// // In other side, `Parser` will call the callback bind to current option `Identifier`.
/// // And set the invoke flag to false.
/// opt.set_need_invoke(false);
///
/// ```
pub trait Callback {
    /// Get callback type of current option
    fn callback_type(&self) -> CallbackType;

    /// Get callback type of current option
    fn set_callback_type(&mut self, callback_type: CallbackType);

    /// Return true if the callback need invoke
    fn is_need_invoke(&self) -> bool;

    /// Set invoke flag
    fn set_need_invoke(&mut self, invoke: bool);
}

/// The name and prefix interface of an option.
/// 
/// For example,
/// ```no_run
/// use getopt_rs::opt::*;
/// 
/// struct O(pub String, pub String);
/// 
/// impl Name for O {
///     fn name(&self) -> &str {
///         self.0.as_str()
///     }
/// 
///     fn prefix(&self) -> &str {
///         self.1.as_str()
///     }
///     fn set_name(&mut self, s: &str) {
///         self.0 = s.to_owned();
///     }
///
///     fn set_prefix(&mut self, s: &str) {
///         self.1 = s.to_owned();
///     }
///
///     fn match_name(&self, s: &str) -> bool {
///         self.0.as_str() == s
///     }
///
///     fn match_prefix(&self, s: &str) -> bool {
///         self.1.as_str() == s
///     }
/// }
/// 
/// let mut opt = O("count".to_owned(), "--".to_owned());
/// 
/// assert_eq!(opt.match_name("count"), true);
/// assert_eq!(opt.match_prefix("--"), true);
/// 
/// opt.set_name("c");
/// opt.set_prefix("-");
/// assert_eq!(opt.match_name("c"), true);
/// assert_eq!(opt.match_prefix("-"), true);
/// ```
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

/// The alias interface of an option.
/// 
/// For example,
/// ```no_run
/// use getopt_rs::opt::*;
/// 
/// struct O(pub Vec<(String, String)>);
/// 
/// impl Alias for O {
///     fn alias(&self) -> Option<&Vec<(String, String)>> {
///         Some(self.0.as_ref())
///     }
///
///     fn add_alias(&mut self, prefix: &str, name: &str) {
///         self.0.push((prefix.to_owned(), name.to_owned()));
///     }
///
///     fn rem_alias(&mut self,  prefix: &str, name: &str) -> bool {
///         for index in 0 .. self.0.len() {
///             let alias = &self.0[index];
///             if alias.0 == prefix && alias.1 == name {
///                 self.0.remove(index);
///                 return true;
///             }
///         }
///         false
///     }
///
///     fn match_alias(&self, prefix: &str, name: &str) -> bool {
///         self.0.iter()
///              .find(|&a| a.0 == prefix  && a.1 == name)
///              .is_some()
///     }
/// }
/// 
/// let mut opt = O(vec![]);
/// 
/// opt.add_alias("--", "count");
/// assert_eq!(opt.match_alias("--", "count"), true);
/// 
/// opt.rem_alias("--", "count");
/// assert_eq!(opt.match_alias("--", "count"), false);
/// ```
pub trait Alias {
    /// Get the alias for current option
    fn alias(&self) -> Option<&Vec<(String, String)>>;

    /// Add an new alias for current option
    fn add_alias(&mut self, prefix: &str, name: &str);

    /// Remove an alias of current option, return true if remove successful
    fn rem_alias(&mut self,  prefix: &str, name: &str) -> bool;

    /// Return true if the option has the alias name
    fn match_alias(&self, prefix: &str, name: &str) -> bool;
}

/// The optional interface of an option.
/// 
/// For example,
/// ```no_run
/// use getopt_rs::opt::*;
/// 
/// struct O(pub bool);
/// 
/// impl Optional for O {
///     fn optional(&self) -> bool {
///         self.0
///     }
///
///     fn set_optional(&mut self, b: bool) {
///         self.0 = b;
///     }
///
///     fn match_optional(&self, b: bool) -> bool {
///         self.0 == b
///     }
/// }
/// 
/// let mut opt = O(false);
/// 
/// assert_eq!(opt.match_optional(false), true);
/// 
/// opt.set_optional(true);
/// assert_eq!(opt.match_optional(true), true);
/// ```
pub trait Optional {
    /// Return true if the option is optional
    fn optional(&self) -> bool;

    /// Set the option to optional
    fn set_optional(&mut self, b: bool);

    /// Return true if the value equal
    fn match_optional(&self, b: bool) -> bool;
}

/// The value interface of an option.
/// 
/// For example,
/// ```no_run
/// use getopt_rs::opt::*;
/// use getopt_rs::error::*;
/// 
/// struct O {
///     value: OptValue,
/// 
///     default_value: OptValue,
/// };
/// 
/// impl O {
///     pub fn new() -> Self {
///         Self { value: OptValue::default(), default_value: OptValue::default() }
///     }
/// }
/// 
/// impl Value for O {
///    fn value(&self) -> &OptValue {
///        &self.value
///    }
///
///    fn default_value(&self) -> &OptValue {
///        &self.default_value
///    }
///
///    fn set_value(&mut self, value_para: OptValue) {
///        self.value = value_para;
///    }
///
///    fn set_default_value(&mut self, default_value_para: OptValue) {
///        self.default_value = default_value_para;
///    }
///
///    fn parse_value(&self, value_para: &str) -> Result<OptValue> {
///        return Ok(OptValue::from_str(value_para));
///    }
///
///    fn has_value(&self) -> bool {
///        self.value().is_str()
///    }
///
///    fn reset_value(&mut self) {
///        self.set_value(self.default_value().clone());
///    }
/// }
/// 
/// let mut opt = O::new();
/// 
/// assert_eq!(opt.value.is_null(), true);
/// 
/// opt.set_value(opt.parse_value("foo").unwrap());
/// assert_eq!(opt.value().as_str(), Some(&"foo".to_owned()));
/// 
/// opt.set_default_value(opt.parse_value("bar").unwrap());
/// assert_eq!(opt.default_value().as_str(), Some(&"bar".to_owned()));
/// 
/// opt.reset_value();
/// assert_eq!(opt.value().as_str(), Some(&"bar".to_owned()));
/// ```
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
    fn parse_value(&self, v: &str) -> Result<OptValue>;

    /// Return true if the option has value setted
    fn has_value(&self) -> bool;

    /// Reset value
    fn reset_value(&mut self);
}

/// The index interface of an non-option.
/// 
/// For example,
/// ```no_run
/// use getopt_rs::opt::*;
/// 
/// struct O(pub NonOptIndex);
/// 
/// impl Index for O {
///     fn index(&self) -> &NonOptIndex {
///         &self.0
///     }
///
///     fn set_index(&mut self, index: NonOptIndex) {
///         self.0 = index;
///     }
///
///     fn match_index(&self, total: i64, current: i64) -> bool {
///        if let Some(realindex) = self.index().calc_index(total) {
///            return realindex == 0 || realindex == current;
///        }
///        false
///     }
/// }
/// 
/// let mut opt = O(NonOptIndex::Forward(3));
/// 
/// assert_eq!(opt.match_index(6, 3), true);
/// assert_eq!(opt.match_index(3, 3), true);
/// assert_eq!(opt.match_index(2, 3), false);
/// 
/// opt.set_index(NonOptIndex::AnyWhere);
/// assert_eq!(opt.match_index(6, 3), true);
/// assert_eq!(opt.match_index(3, 3), true);
/// assert_eq!(opt.match_index(2, 3), true);
/// 
/// opt.set_index(NonOptIndex::Backward(3));
/// assert_eq!(opt.match_index(6, 4), true);
/// assert_eq!(opt.match_index(3, 1), true);
/// assert_eq!(opt.match_index(2, 1), false);
/// ```
pub trait Index {
    fn index(&self) -> &NonOptIndex;

    fn set_index(&mut self, index: NonOptIndex);

    fn match_index(&self, total: i64, current: i64) -> bool;
}

/// The option trait type, you need implement follow traits:
/// 
/// * [`Type`]
/// * [`Identifier`]
/// * [`Name`]
/// * [`Alias`]
/// * [`Optional`]
/// * [`Value`]
/// * [`Index`]
/// * [`Callback`]
pub trait Opt: Type + Identifier + Name + Alias + Optional + Value + Index + Callback + Debug { }

/// Helper function clone the any value
pub struct CloneHelper(Box< dyn Fn (&dyn Any) -> Box<dyn Any>>);

impl Debug for CloneHelper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AnyCloneHelper")
         .field("Fn", &String::from("..."))
         .finish()
    }
}

impl OptValue {
    pub fn parse_int(s: &str) -> Result<Self> {
        match s.parse::<i64>() {
            Ok(value) => Ok(Self::from_int(value)),
            Err(e) => Err(Error::InvaldOptionValue(s.to_owned(), format!("{:?}", e))),
        }
    }

    pub fn parse_uint(s: &str) -> Result<Self> {
        match s.parse::<u64>() {
            Ok(value) => Ok(Self::from_uint(value)),
            Err(e) => Err(Error::InvaldOptionValue(s.to_owned(), format!("{:?}", e))),
        }
    }

    pub fn parse_flt(s: &str) -> Result<Self> {
        match s.parse::<f64>() {
            Ok(value) => Ok(Self::from_flt(value)),
            Err(e) => Err(Error::InvaldOptionValue(s.to_owned(), format!("{:?}", e))),
        }
    }

    pub fn from_int<T: Into<i64>>(t: T) -> Self {
        Self::Int(t.into())
    }

    pub fn from_uint<T: Into<u64>>(t: T) -> Self {
        Self::Uint(t.into())
    }

    pub fn from_flt<T: Into<f64>>(t: T) -> Self {
        Self::Flt(t.into())
    }

    pub fn from_str<T: Into<String>>(t: T) -> Self {
        Self::Str(t.into())
    }

    pub fn from_bool<T: Into<bool>>(t: T) -> Self {
        Self::Bool(t.into())
    }

    pub fn from_vec<T: Into<Vec<String>>>(t: T) -> Self {
        Self::Array(t.into())
    }

    pub fn from_any<T: Any>(t: Box<T>) -> Self {
        Self::Any(t)
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

    /// Return None if the value is not an OptValue::Flt
    pub fn as_flt(&self) -> Option<&f64> {
        match self {
            Self::Flt(v) => Some(v),
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
    pub fn as_vec(&self) -> Option<&Vec<String>> {
        match self {
            Self::Array(v) => Some(v),
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

    /// Return None if the value is not an OptValue::Any
    pub fn downcast_ref<T: Any>(&self) -> Option<&T> {
        match self {
            Self::Any(v) => v.as_ref().downcast_ref::<T>(),
            _ => None,
        }
    }

    /// Return None if the value is not an OptValue::Int
    pub fn as_int_mut(&mut self) -> Option<&mut i64> {
        match self {
            Self::Int(v) => Some(v),
            _ => None,
        }
    }

    /// Return None if the value is not an OptValue::Uint
    pub fn as_uint_mut(&mut self) -> Option<&mut u64> {
        match self {
            Self::Uint(v) => Some(v),
            _ => None,
        }
    }

    /// Return None if the value is not an OptValue::Flt
    pub fn as_flt_mut(&mut self) -> Option<&mut f64> {
        match self {
            Self::Flt(v) => Some(v),
            _ => None,
        }
    }

    /// Return None if the value is not an OptValue::Str
    pub fn as_str_mut(&mut self) -> Option<&mut String> {
        match self {
            Self::Str(v) => Some(v),
            _ => None,
        }
    }

    /// Return None if the value is not an OptValue::Bool
    pub fn as_bool_mut(&mut self) -> Option<&mut bool> {
        match self {
            Self::Bool(v) => Some(v),
            _ => None,
        }
    }

    /// Return None if the value is not an OptValue::Arr
    pub fn as_vec_mut(&mut self) -> Option<&mut Vec<String>> {
        match self {
            Self::Array(v) => Some(v),
            _ => None,
        }
    }

    pub fn app_value(&mut self, s: String) -> &mut Self {
        match self {
            Self::Array(v) => {
                v.push(s);
            }
            _ => { }
        }
        self
    }

    /// Return None if the value is not an OptValue::Any
    pub fn as_any_mut(&mut self) -> Option<&mut Box<dyn Any>> {
        match self {
            Self::Any(v) => Some(v),
            _ => None,
        }
    }

    /// Return None if the value is not an OptValue::Any
    pub fn downcast_mut<T: Any>(&mut self) -> Option<&mut T> {
        match self {
            Self::Any(v) => v.as_mut().downcast_mut::<T>(),
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

    pub fn is_flt(&self) -> bool {
        match self {
            Self::Flt(_) => true,
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

    pub fn is_array(&self) -> bool {
        match self {
            Self::Array(_) => true,
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

    pub fn is<T: Any>(&self) -> bool {
        match self {
            Self::Any(v) => v.as_ref().is::<T>(),
            _ => false,
        }
    }

    pub fn clone_or(&self, clone_helper: &Option<CloneHelper>) -> OptValue {
        match self {
            Self::Any(av) => {
                if clone_helper.is_some() {
                    Self::Any(clone_helper.as_ref().unwrap().0(av.as_ref()))
                }
                else {
                    Self::Null
                }
            }
            _ => {
                self.clone()
            }
        }
    }
}

/// Clone the option value except `OptValue::Any`.
impl Clone for OptValue {
    fn clone(&self) -> Self {
        match self {
            Self::Int(iv) => { Self::Int(*iv) },

            Self::Uint(uv) => { Self::Uint(*uv) },

            Self::Flt(fv) => { Self::Flt(*fv) },

            Self::Str(sv) => { Self::Str(sv.clone()) },

            Self::Bool(bv) => { Self::Bool(*bv) },

            Self::Array(vv) => { Self::Array(vv.clone()) },

            Self::Null => { Self::Null },

            Self::Any(_) => {
                Self::Null
            }
        }
    }
}

impl Default for OptValue {
    fn default() -> Self {
        OptValue::Null
    }
}


#[macro_export]
macro_rules! opt_def {
    ($opt:ty, $($trait:ident),+) => (
        $(
            impl $trait for $opt { }
        )+

        impl Opt for $opt { }
    )
}

/// Create a `Identifier` implementation for type `$opt`.
/// 
/// For example,
/// ```no_run
/// use getopt_rs::opt::*;
/// use getopt_rs::id;
/// 
/// #[derive(Debug)]
/// pub struct StrOpt {
///     id: id::Identifier,
///
///     name: String,
///
///     prefix: String,
///
///     optional: bool,
///
///     value: OptValue,
///
///     alias: Vec<String>,
/// }
/// 
/// // `opt_identifier_def!(StrOpt, id, para)` will expand to 
/// 
/// impl Identifier for StrOpt {
///      fn id(&self) -> id::Identifier {
///         self.id
///      }
/// 
///     fn set_id(&mut self, para: id::Identifier) {
///         self.id = para
///     }
/// }
/// ```
#[macro_export]
macro_rules! opt_identifier_def {
    ($opt:ty,
     $identifier:ident,
     $identifier_para:ident,
    ) => (
        impl Identifier for $opt {
            fn id(&self) -> IIdentifier {
                self.$identifier
            }


            fn set_id(&mut self, $identifier_para: IIdentifier) {
                self.$identifier = $identifier_para
            }
        }
    )
}

/// Create a `Type` implementation for type `$opt`.
/// 
/// For example,
/// ```no_run
/// use getopt_rs::opt::*;
/// use getopt_rs::id;
/// use std::any::Any;
/// use getopt_rs::error::*;
/// 
/// #[derive(Debug)]
/// pub struct StrOpt {
///     id: id::Identifier,
///
///     name: String,
///
///     prefix: String,
///
///     optional: bool,
///
///     value: OptValue,
///
///     alias: Vec<String>,
/// }
/// 
/// // `opt_type_def!(StrOpt, current_type(), false, { style, Style::Argument })` will expand to 
/// 
/// impl Type for StrOpt {
///     fn type_name(&self) -> &str {
///         "str"
///     }
///
///     fn is_deactivate_style(&self) -> bool {
///         false
///     }
///
///     fn is_style(&self, style_para: Style) -> bool {
///         match style_para {
///             Style::Argument => true,
///             _ => false
///         }
///     }
/// 
///     fn check(&self) -> Result<bool> {
///         // comment this for compile error
///         // if (!self.optional()) && (!self.has_value()) {
///         //    return Err(Error::OptionForceRequired(self.name.to_owned()));
///         // }
///         Ok(true)
///     }
/// 
///     fn as_any(&self) -> &dyn Any {
///         self
///     }
/// }
/// ```
/// The `$deactivate` parameter is false in default.
#[macro_export]
macro_rules! opt_type_def {
    ($opt:ty,
     $type_name:expr,
     {
        $style_var:ident,
        $($style_pattern:pat),+
     }
    ) => (
        impl Type for $opt {
            fn type_name(&self) -> &str {
                $type_name
            }

            fn is_deactivate_style(&self) -> bool {
                false
            }

            fn is_style(&self, $style_var: Style) -> bool {
                match $style_var {
                    $(
                        $style_pattern => true,
                    )+
                    _ => false
                }
            }

            fn check(&self) -> Result<bool> {
                if (!self.optional()) && (!self.has_value()) {
                    return Err(Error::OptionForceRequired(format!("{}{}", self.prefix(), self.name())));
                }
                Ok(true)
            }

            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
        }
    );

    ($opt:ty,
     $type_name:expr,
     false,
     {
        $style_var:ident,
        $($style_pattern:pat),+
     }
    ) => (
        impl Type for $opt {
            fn type_name(&self) -> &str {
                $type_name
            }

            fn is_deactivate_style(&self) -> bool {
                false
            }

            fn is_style(&self, $style_var: Style) -> bool {
                match $style_var {
                    $(
                        $style_pattern => true,
                    )+
                    _ => false
                }
            }

            fn check(&self) -> Result<bool> {
                if (!self.optional()) && (!self.has_value()) {
                    return Err(Error::OptionForceRequired(format!("{}{}", self.prefix(), self.name())));
                }
                Ok(true)
            }

            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
        }
    );

    ($opt:ty,
     $type_name:expr,
     $deactivate_member:ident,
     {
        $style_var:ident,
        $($style_pattern:pat),+
     }
    ) => (
        impl Type for $opt {
            fn type_name(&self) -> &str {
                $type_name
            }

            fn is_deactivate_style(&self) -> bool {
                self.$deactivate_member
            }

            fn is_style(&self, $style_var: Style) -> bool {
                match $style_var {
                    $(
                        $style_pattern => true,
                    )+
                    _ => false
                }
            }

            fn check(&self) -> Result<bool> {
                if (!self.optional()) && (!self.has_value()) {
                    return Err(Error::OptionForceRequired(format!("{}{}", self.prefix(), self.name())));
                }
                Ok(true)
            }

            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
        }
    );
}

/// Create a `Callback` implementation for type `$opt`.
/// 
/// For example,
/// ```no_run
/// use getopt_rs::opt::*;
/// use getopt_rs::id;
/// use getopt_rs::callback::*;
/// 
/// #[derive(Debug)]
/// pub struct StrOpt {
///     id: id::Identifier,
///
///     name: String,
///
///     prefix: String,
///
///     optional: bool,
///
///     value: OptValue,
///
///     alias: Vec<String>,
/// 
///     callback: CallbackType,
/// }
/// 
/// // `opt_callback_def!(StrOpt, callback, callback_para, CallbackType::Value, CallbackType::Null)` will expand to 
/// 
/// impl Callback for StrOpt {
///     fn callback_type(&self) -> CallbackType {
///         self.callback.clone()
///     }
/// 
///     fn set_callback_type(&mut self, callback_para: CallbackType) {
///         self.callback = callback_para;
///     }
///
///     fn is_need_invoke(&self) -> bool {
///         ! self.callback.is_null()
///     }
///
///     fn set_need_invoke(&mut self, callback_para: bool) {
///         if callback_para {
///             self.callback = CallbackType::Value;
///         }
///         else {
///             self.callback = CallbackType::Null;
///         }
///     }
/// }
/// ```
#[macro_export]
macro_rules! opt_callback_def {
    ($opt:ty,
     $callback:ident,
     $callback_para:ident,
     $callback_true_value:expr,
     $callback_false_value:expr,
    ) => (
        impl Callback for $opt {
            fn callback_type(&self) -> CallbackType {
                self.$callback.clone()
            }

            fn set_callback_type(&mut self, $callback_para: CallbackType) {
                self.$callback = $callback_para;
            }

            fn is_need_invoke(&self) -> bool {
                ! self.$callback.is_null()
            }

            fn set_need_invoke(&mut self, $callback_para: bool) {
                if $callback_para {
                    self.$callback = $callback_true_value;
                }
                else {
                    self.$callback = $callback_false_value;
                }
            }
        }
    )
}

/// Create a `Name` implementation for type `$opt`.
///
/// For example,
/// ```no_run
/// use getopt_rs::opt::*;
/// use getopt_rs::id;
/// 
/// #[derive(Debug)]
/// pub struct StrOpt {
///     id: id::Identifier,
///
///     name: String,
///
///     prefix: String,
///
///     optional: bool,
///
///     value: OptValue,
///
///     alias: Vec<String>,
/// }
/// 
/// // `opt_name_def!(StrOpt, name, prefix, name_para, prefix_para)` will expand to
/// 
/// impl Name for StrOpt {
///     fn name(&self) -> &str {
///         &self.name
///     }
/// 
///     fn prefix(&self) -> &str {
///         &self.prefix
///     }
///
///     fn set_name(&mut self, name_para: &str) {
///         self.name = name_para.to_owned()
///     }
///
///     fn set_prefix(&mut self, prefix_para: &str) {
///         self.prefix = prefix_para.to_owned()
///     }
///
///     fn match_name(&self, name_para: &str) -> bool {
///         self.name() == name_para
///     }
///
///     fn match_prefix(&self, prefix_para: &str) -> bool {
///         self.prefix() == prefix_para
///     }
/// }
/// ```
#[macro_export]
macro_rules! opt_name_def {
    ($opt:ty,
     $name:ident,
     $name_para:ident,
    ) => (
        impl Name for $opt {
            fn name(&self) -> &str {
                &self.$name
            }

            fn prefix(&self) -> &str {
                ""
            }

            fn set_name(&mut self, $name_para: &str) {
                self.$name = $name_para.to_owned()
            }

            fn set_prefix(&mut self, _: &str) {
                
            }

            fn match_name(&self, $name_para: &str) -> bool {
                self.name() == $name_para
            }

            fn match_prefix(&self, prefix_para: &str) -> bool {
                self.prefix() == prefix_para
            }
        }
    );

    ($opt:ty,
     $prefix:ident,
     $name:ident,
     $prefix_para:ident,
     $name_para:ident,
    ) => (
        impl Name for $opt {
            fn name(&self) -> &str {
                &self.$name
            }

            fn prefix(&self) -> &str {
                &self.$prefix
            }

            fn set_name(&mut self, $name_para: &str) {
                self.$name = $name_para.to_owned()
            }

            fn set_prefix(&mut self, $prefix_para: &str) {
                self.$prefix = $prefix_para.to_owned()
            }

            fn match_name(&self, $name_para: &str) -> bool {
                self.name() == $name_para
            }

            fn match_prefix(&self, $prefix_para: &str) -> bool {
                self.prefix() == $prefix_para
            }
        }
    );
}

/// Create a `Optional` implementation for type `$opt`.
/// 
/// For example,
/// ```no_run
/// use getopt_rs::opt::*;
/// use getopt_rs::id;
/// 
/// #[derive(Debug)]
/// pub struct StrOpt {
///     id: id::Identifier,
///
///     name: String,
///
///     prefix: String,
///
///     optional: bool,
///
///     value: OptValue,
///
///     alias: Vec<String>,
/// }
/// 
/// // `opt_optional_def!(StrOpt, optional, optional_para)` will expand to
/// 
/// impl Optional for StrOpt {
///     fn optional(&self) -> bool {
///         self.optional
///     }
///
///     fn set_optional(&mut self, optional_para: bool) {
///         self.optional = optional_para
///     }
///
///     fn match_optional(&self, optional_para: bool) -> bool {
///         self.optional() == optional_para
///     }
/// }
/// ```
#[macro_export]
macro_rules! opt_optional_def {
    ($opt:ty,
     $optional:ident,
     $optional_para:ident,
    ) => (
        impl Optional for $opt {
            fn optional(&self) -> bool {
                self.$optional
            }

            fn set_optional(&mut self, $optional_para: bool) {
                self.$optional = $optional_para
            }

            fn match_optional(&self, $optional_para: bool) -> bool {
                self.optional() == $optional_para
            }
        }
    );
}

/// Create a `Alias` implementation for type `$opt`.
///
/// For example,
/// ```no_run
/// use getopt_rs::opt::*;
/// use getopt_rs::id;
/// 
/// #[derive(Debug)]
/// pub struct StrOpt {
///     id: id::Identifier,
///
///     name: String,
///
///     prefix: String,
///
///     optional: bool,
///
///     value: OptValue,
///
///     alias: Vec<(String, String)>,
/// }
/// 
/// // `opt_alias_def!(StrOpt, alias, prefix_para, name_para)` will expand to
/// 
/// impl Alias for StrOpt {
/// fn alias(&self) -> Option<&Vec<(String, String)>> {
///     Some(&self.alias)
/// }
///
/// fn add_alias(&mut self,  prefix_para: &str, name_para: &str) {
///     self.alias.push((prefix_para.to_owned(), name_para.to_owned()))
/// }
///
/// fn rem_alias(&mut self, prefix_para: &str, name_para: &str) -> bool {
///     if name_para != self.name && self.prefix != prefix_para {
///         for index in 0 .. self.alias.len() {
///             let alias = &self.alias[index];
///
///             if alias.0 == prefix_para && alias.1 == name_para {
///                 self.alias.remove(index);
///                 return true;
///             }
///         }
///     }
///     false 
/// }
///
/// fn match_alias(&self, prefix_para: &str, name_para: &str) -> bool {
///     self.alias.iter()
///               .find(|&a| a.0 == prefix_para && a.1 == name_para)
///               .is_some()
/// }
/// }
/// ```
#[macro_export]
macro_rules! opt_alias_def {
        ($opt:ty) => (
        impl Alias for $opt {
            fn alias(&self) -> Option<&Vec<(String, String)>> {
                None
            }

            fn add_alias(&mut self,  _prefix: &str, _name: &str) { }

            fn rem_alias(&mut self,  _prefix: &str, _name: &str) -> bool { false }

            fn match_alias(&self,  _prefix: &str, _name: &str) -> bool { false }
        }
    );

    ($opt:ty,
     $alias_member:ident,
     $prefix_para:ident,
     $name_para:ident,
    ) => (
        impl Alias for $opt {
            fn alias(&self) -> Option<&Vec<(String, String)>> {
                Some(&self.$alias_member)
            }

            fn add_alias(&mut self,  $prefix_para: &str, $name_para: &str) {
                self.$alias_member.push(($prefix_para.to_owned(), $name_para.to_owned()))
            }

            fn rem_alias(&mut self, $prefix_para: &str, $name_para: &str) -> bool {
                if $name_para != self.name() && self.prefix() != $prefix_para {
                    for index in 0 .. self.$alias_member.len() {
                        let alias = &self.$alias_member[index];

                        if alias.0 == $prefix_para && alias.1 == $name_para {
                            self.$alias_member.remove(index);
                            return true;
                        }
                    }
                }
                false 
            }

            fn match_alias(&self, $prefix_para: &str, $name_para: &str) -> bool {
                self.$alias_member.iter()
                                  .find(|&a| a.0 == $prefix_para && a.1 == $name_para)
                                  .is_some()
            }
        }
    );
}

/// Create a `Index` implementation for type `$opt`.
///
/// For example,
/// ```no_run
/// use getopt_rs::opt::*;
/// use getopt_rs::id;
/// 
/// #[derive(Debug)]
/// pub struct StrOpt {
///     id: id::Identifier,
///
///     name: String,
///
///     prefix: String,
///
///     optional: bool,
///
///     value: OptValue,
///
///     alias: Vec<String>,
/// 
///     index: NonOptIndex,
/// }
/// 
/// // `opt_index_def!(StrOpt, index, index_para)` will expand to
/// 
/// impl Index for StrOpt {
///     fn index(&self) -> &NonOptIndex {
///         &self.index
///     }
///
///     fn set_index(&mut self, index_para: NonOptIndex) {
///         self.index = index_para
///     }
///
///     fn match_index(&self, total: i64, current: i64) -> bool {
///        if let Some(realindex) = self.index().calc_index(total) {
///            return realindex == 0 || realindex == current;
///        }
///        false
///     }
/// }
/// ```
#[macro_export]
macro_rules! opt_index_def {
        ($opt:ty) => (
        impl Index for $opt {
            fn index(&self) -> &NonOptIndex {
                &NonOptIndex::Null
            }

            fn set_index(&mut self, _: NonOptIndex) { }

            /// using for option
            fn match_index(&self, _: i64, _: i64) -> bool {
                true
            }
        }
    );

    ($opt:ty,
     $index:ident,
     $index_para:ident,
    ) => (
        impl Index for $opt {
            fn index(&self) -> &NonOptIndex {
                &self.$index
            }

            fn set_index(&mut self, $index_para: NonOptIndex) {
                self.$index = $index_para
            }

            fn match_index(&self, total: i64, current: i64) -> bool {
                if let Some(realindex) = self.index().calc_index(total) {
                    return realindex == 0 || realindex == current;
                }
                false
            }
        }
    );
}

pub mod str {
    use crate::opt::*;
    use crate::id::Identifier as IIdentifier;

    pub fn current_type() -> &'static str {
        "str"
    }

    pub trait Str: Opt { }

    /// StrOpt target the value to [`String`], 
    /// 
    /// * The option type name is `str`.
    /// * The option is not support deactivate style.
    /// * The option accept the style [`Style::Argument`].
    /// * In default, the option is `optional`, it can be change through the [`set_optional`](crate::opt::Optional::set_optional).
    /// * The option need an [`OptValue::Str`] argument, the default value is [`OptValue::default()`].
    /// * The option support multiple alias with different prefix and name.
    /// * The option support callback type [`CallbackType::Value`].
    ///
    /// User can set it at `anywhere` of command line argument, using the string `-s "value"`, `-s=value`, `--str "value"`, etc.
    #[derive(Debug)]
    pub struct StrOpt {
        id: IIdentifier,

        name: String,

        prefix: String,

        optional: bool,

        value: OptValue,

        default_value: OptValue,

        alias: Vec<(String, String)>,

        callback: CallbackType,
    }

    impl StrOpt {
        pub fn new(id: IIdentifier, name: String, prefix: String, optional: bool, default_value: OptValue) -> Self {
            Self {
                id,
                name,
                prefix,
                optional,
                value: default_value.clone_or(&None),
                default_value,
                alias: vec![],
                callback: CallbackType::Null,
            }
        }
    }

    opt_def!(StrOpt, Str);

    opt_type_def!(
        StrOpt, 
        current_type(),
        false,
        { style, Style::Argument }
    );

    opt_callback_def!(
        StrOpt,
        callback,
        callback,
        CallbackType::Value,
        CallbackType::Null,
    );

    opt_identifier_def!(
        StrOpt,
        id,
        para,
    );

    opt_name_def!(
        StrOpt,
        prefix,
        name,
        prefix,
        name,
    );

    opt_optional_def!(
        StrOpt,
        optional,
        optional,
    );

    opt_alias_def!(
        StrOpt,
        alias,
        prefix,
        name,
    );

    opt_index_def!( StrOpt );

    impl Value for StrOpt {
        fn value(&self) -> &OptValue {
            &self.value
        }

        fn default_value(&self) -> &OptValue {
            &self.default_value
        }

        fn set_value(&mut self, value_para: OptValue) {
            self.value = value_para;
        }

        fn set_default_value(&mut self, default_value_para: OptValue) {
            self.default_value = default_value_para;
        }

        fn parse_value(&self, value_para: &str) -> Result<OptValue> {
            return Ok(OptValue::from_str(value_para));
        }

        fn has_value(&self) -> bool {
            self.value().is_str()
        }

        fn reset_value(&mut self) {
            self.set_value(self.default_value().clone());
        }
    }

    /// Default [`Utils`] implementation for [`StrOpt`].
    #[derive(Debug)]
    pub struct StrUtils;

    impl StrUtils {
        pub fn new() -> Self {
            Self {}
        }
    }

    impl Utils for StrUtils {
        fn type_name(&self) -> &str {
            current_type()
        }

        fn is_support_deactivate_style(&self) -> bool {
            false
        }

        /// Create an [`StrOpt`] using option information [`CreateInfo`].
        /// 
        /// ```no_run
        /// use getopt_rs::utils::{Utils, CreateInfo};
        /// use getopt_rs::opt::str::*;
        /// use getopt_rs::id::*;
        /// 
        /// let prefixs = vec![String::from("--")];
        /// let utils = StrUtils::new();
        /// let ci = CreateInfo::parse("--name=str!", &prefixs).unwrap();
        /// let _opt = utils.create(Identifier::new(1), &ci);
        /// ```
        fn create(&self, id: IIdentifier, ci: &CreateInfo) -> Result<Box<dyn Opt>> {
            if ci.is_deactivate_style() {
                if ! self.is_support_deactivate_style() {
                    return Err(Error::UtilsNotSupportDeactivateStyle(ci.get_type_name().to_owned()));
                }
            }
            if ci.get_type_name() != self.type_name() {
                return Err(Error::UtilsNotSupportTypeName(self.type_name().to_owned(), ci.get_type_name().to_owned()))
            }
            
            assert_eq!(ci.get_type_name(), self.type_name());

            let mut opt = Box::new(StrOpt::new(
                id,
                ci.get_name().to_owned(),
                ci.get_prefix().to_owned(),
                ci.is_optional(),
                ci.get_default_value().clone_or(&None),
            ));

            let alias = ci.get_alias();

            if alias.len() > 0 {
                for a in alias.iter() {
                    opt.add_alias(&a.0, &a.1);
                }
            }

            Ok(opt)
        }

        fn gen_info(&self, opt: &dyn Opt) -> Box<dyn Info> {
            Box::new(OptionInfo::new(opt.id()))
        }
    }
}

pub mod bool {
    use crate::opt::*;
    use crate::id::Identifier as IIdentifier;

    pub fn current_type() -> &'static str {
        "bool"
    }

    pub trait Bool: Opt { }

    /// BoolOpt target the value to [`bool`](prim@bool), 
    /// 
    /// * The option type name is `bool`.
    /// * The option is support deactivate style.
    /// * The option accept the style [`Style::Boolean`] and [`Style::Multiple`].
    /// * In default, the option is `optional`, it can be change through the [`set_optional`](crate::opt::Optional::set_optional).
    /// * The option need an [`OptValue::Bool`] argument, the default value is [`OptValue::default()`].
    /// * The option support multiple alias with different prefix and name.
    /// * The option support callback type [`CallbackType::Value`].
    ///
    /// User can set it at `anywhere` of command line argument, using the string `-q`, `-s`, `-qs`, `--quite`, `--slient`, etc.
    #[derive(Debug)]
    pub struct BoolOpt {
        id: IIdentifier,

        name: String,

        prefix: String,

        optional: bool,

        value: OptValue,

        default_value: OptValue,

        deactivate_style: bool,

        alias: Vec<(String, String)>,

        callback: CallbackType,
    }

    impl BoolOpt {
        pub fn new(id: IIdentifier, name: String, prefix: String, optional: bool, deactivate_style: bool) -> Self {
            let default_value = if deactivate_style {
                    OptValue::from_bool(true)
                } else {
                    OptValue::from_bool(false)
            };
            Self {
                id,
                name,
                prefix,
                optional,
                value: default_value.clone_or(&None),
                default_value,
                deactivate_style,
                alias: vec![],
                callback: CallbackType::Null,
            }
        }
    }

    opt_def!(BoolOpt, Bool);

    opt_type_def!(
        BoolOpt, 
        current_type(),
        deactivate_style,
        { style, Style::Boolean, Style::Multiple }
    );

    opt_callback_def!(
        BoolOpt,
        callback,
        callback,
        CallbackType::Value,
        CallbackType::Null,
    );

    opt_identifier_def!(
        BoolOpt,
        id,
        para,
    );

    opt_name_def!(
        BoolOpt,
        prefix,
        name,
        prefix,
        name,
    );

    opt_optional_def!(
        BoolOpt,
        optional,
        optional,
    );

    opt_alias_def!(
        BoolOpt,
        alias,
        prefix,
        name,
    );

    opt_index_def!( BoolOpt );

    impl Value for BoolOpt {
        fn value(&self) -> &OptValue {
            &self.value
        }

        fn default_value(&self) -> &OptValue {
            &self.default_value
        }

        fn set_value(&mut self, value_para: OptValue) {
            self.value = value_para;
        }

        fn set_default_value(&mut self, default_value_para: OptValue) {
            self.default_value = default_value_para;
        }

        fn parse_value(&self, _: &str) -> Result<OptValue> {
            Ok(OptValue::from_bool(! self.is_deactivate_style()))
        }

        /// For [`BoolOpt`], it need return true if current value is not equal default value
        fn has_value(&self) -> bool {
            self.value().as_bool() != self.default_value().as_bool()
        }

        fn reset_value(&mut self) {
            let value = match self.default_value() {
                OptValue::Bool(value) => {
                    Some(value.clone())
                }
                _ => {
                    None
                }
            };

            // Set to default value if exists.
            // Or set to true if the option is deactivate style, otherwise false
            self.set_value(OptValue::from_bool(value.unwrap_or(! self.is_deactivate_style())));
        }
    }

    /// Default [`Utils`] implementation for [`BoolOpt`].
    #[derive(Debug)]
    pub struct BoolUtils;

    impl BoolUtils {
        pub fn new() -> Self {
            Self {}
        }
    }

    impl Utils for BoolUtils {
        fn type_name(&self) -> &str {
            current_type()
        }

        fn is_support_deactivate_style(&self) -> bool {
            true
        }

        /// Create an [`BoolOpt`] using option information [`CreateInfo`].
        /// 
        /// ```no_run
        /// use getopt_rs::utils::{Utils, CreateInfo};
        /// use getopt_rs::opt::bool::*;
        /// use getopt_rs::id::*;
        /// 
        /// let prefixs = vec![String::from("--")];
        /// let utils = BoolUtils::new();
        /// let ci = CreateInfo::parse("--name=bool!", &prefixs).unwrap();
        /// let _opt = utils.create(Identifier::new(1), &ci);
        /// ```
        fn create(&self, id: IIdentifier, ci: &CreateInfo) -> Result<Box<dyn Opt>> {
            if ci.is_deactivate_style() {
                if ! self.is_support_deactivate_style() {
                    return Err(Error::UtilsNotSupportDeactivateStyle(ci.get_name().to_owned()));
                }
            }
            if ci.get_type_name() != self.type_name() {
                return Err(Error::UtilsNotSupportTypeName(self.type_name().to_owned(), ci.get_type_name().to_owned()))
            }
            
            assert_eq!(ci.get_type_name(), self.type_name());

            let mut opt = Box::new(BoolOpt::new(
                id,
                ci.get_name().to_owned(),
                ci.get_prefix().to_owned(),
                ci.is_optional(),
                ci.is_deactivate_style(),
            ));

            let alias = ci.get_alias();

            if alias.len() > 0 {
                for a in alias.iter() {
                    opt.add_alias(&a.0, &a.1);
                }
            }

            Ok(opt)
        }

        fn gen_info(&self, opt: &dyn Opt) -> Box<dyn Info> {
            Box::new(OptionInfo::new(opt.id()))
        }
    }
}

pub mod array {
    use crate::opt::*;
    use crate::id::Identifier as IIdentifier;

    pub fn current_type() -> &'static str {
        "array"
    }

    pub trait Array: Opt { }

    /// ArrayOpt target the value to [`Vec<String>`], 
    /// 
    /// * The option type name is `array`.
    /// * The option is not support deactivate style.
    /// * The option accept the style [`Style::Argument`].
    /// * In default, the option is `optional`, it can be change through the [`set_optional`](crate::opt::Optional::set_optional).
    /// * The option need an [`OptValue::Array`] argument, the default value is [`OptValue::default()`].
    /// * The option support multiple alias with different prefix and name.
    /// * The option support callback type [`CallbackType::Value`].
    ///
    /// User can set it at `anywhere` of command line argument, using the string `-a "foo"`, `-a "bar"`, `-a=foo`, `--append=bar`, `--append=foo`, etc.
    /// Set value to `ArrayOpt` will append the value to it.
    #[derive(Debug)]
    pub struct ArrayOpt {
        id: IIdentifier,

        name: String,

        prefix: String,

        optional: bool,

        value: OptValue,

        default_value: OptValue,

        alias: Vec<(String, String)>,

        callback: CallbackType,
    }

    impl ArrayOpt {
        pub fn new(id: IIdentifier, name: String, prefix: String, optional: bool, default_value: OptValue) -> Self {
            Self {
                id,
                name,
                prefix,
                optional,
                value: default_value.clone_or(&None),
                default_value,
                alias: vec![],
                callback: CallbackType::Null,
            }
        }
    }

    opt_def!(ArrayOpt, Array);

    opt_type_def!(
        ArrayOpt, 
        current_type(),
        false,
        { style, Style::Argument }
    );

    opt_callback_def!(
        ArrayOpt,
        callback,
        callback,
        CallbackType::Value,
        CallbackType::Null,
    );

    opt_identifier_def!(
        ArrayOpt,
        id,
        para,
    );

    opt_name_def!(
        ArrayOpt,
        prefix,
        name,
        prefix,
        name,
    );

    opt_optional_def!(
        ArrayOpt,
        optional,
        optional,
    );

    opt_alias_def!(
        ArrayOpt,
        alias,
        prefix,
        name,
    );

    opt_index_def!( ArrayOpt );

    impl Value for ArrayOpt {
        fn value(&self) -> &OptValue {
            &self.value
        }

        fn default_value(&self) -> &OptValue {
            &self.default_value
        }

        /// WARNING! 
        /// This function will append the `value` to option's value
        fn set_value(&mut self, value_para: OptValue) {
            let mut value_para = value_para;

            if value_para.is_array() {
                if self.value.is_null() {
                    self.value = OptValue::from_vec(vec![]);
                }
                self.value
                .as_vec_mut()
                .unwrap()
                .append(value_para.as_vec_mut().unwrap());
            }
        }

        fn set_default_value(&mut self, default_value_para: OptValue) {
            self.default_value = default_value_para;
        }
        
        fn parse_value(&self, value: &str) -> Result<OptValue> {
            let mut realv = OptValue::from_vec(vec![]);

            realv.app_value(value.to_owned());
            
            Ok(realv)
        }

        fn has_value(&self) -> bool {
            self.value().is_array()
        }

        fn reset_value(&mut self) {
            self.value = OptValue::default();
            self.set_value(self.default_value().clone());
        }
    }

    /// Default [`Utils`] implementation for [`ArrayOpt`].
    #[derive(Debug)]
    pub struct ArrayUtils;

    impl ArrayUtils {
        pub fn new() -> Self {
            Self {}
        }
    }

    impl Utils for ArrayUtils {
        fn type_name(&self) -> &str {
            current_type()
        }

        fn is_support_deactivate_style(&self) -> bool {
            false
        }

        /// Create an [`ArrayOpt`] using option information [`CreateInfo`].
        /// 
        /// ```no_run
        /// use getopt_rs::utils::{Utils, CreateInfo};
        /// use getopt_rs::opt::array::*;
        /// use getopt_rs::id::*;
        /// 
        /// let prefixs = vec![String::from("--")];
        /// let utils = ArrayUtils::new();
        /// let ci = CreateInfo::parse("--name=array!", &prefixs).unwrap();
        /// let _opt = utils.create(Identifier::new(1), &ci);
        /// ```
        fn create(&self, id: IIdentifier, ci: &CreateInfo) -> Result<Box<dyn Opt>> {
            if ci.is_deactivate_style() {
                if ! self.is_support_deactivate_style() {
                    return Err(Error::UtilsNotSupportDeactivateStyle(ci.get_name().to_owned()));
                }
            }
            if ci.get_type_name() != self.type_name() {
                return Err(Error::UtilsNotSupportTypeName(self.type_name().to_owned(), ci.get_type_name().to_owned()))
            }
            
            assert_eq!(ci.get_type_name(), self.type_name());

            let mut opt = Box::new(ArrayOpt::new(
                id,
                ci.get_name().to_owned(),
                ci.get_prefix().to_owned(),
                ci.is_optional(),
                ci.get_default_value().clone_or(&None),
            ));

            let alias = ci.get_alias();

            if alias.len() > 0 {
                for a in alias.iter() {
                    opt.add_alias(&a.0, &a.1);
                }
            }

            Ok(opt)
        }

        fn gen_info(&self, opt: &dyn Opt) -> Box<dyn Info> {
            Box::new(OptionInfo::new(opt.id()))
        }
    }
}

pub mod int {
    use crate::opt::*;
    use crate::id::Identifier as IIdentifier;

    pub fn current_type() -> &'static str {
        "int"
    }

    pub trait Int: Opt { }

    /// IntOpt target the value to [`i64`], 
    /// 
    /// * The option type name is `int`.
    /// * The option is not support deactivate style.
    /// * The option accept the style [`Style::Argument`].
    /// * In default, the option is `optional`, it can be change through the [`set_optional`](crate::opt::Optional::set_optional).
    /// * The option need an [`OptValue::Int`] argument, the default value is [`OptValue::default()`].
    /// * The option support multiple alias with different prefix and name.
    /// * The option support callback type [`CallbackType::Value`].
    ///
    /// User can set it at `anywhere` of command line argument, using the string `-c 1`, `-c -3`, `-c=2`, `--count 4`, `--count -4`, `--count=8`, etc.
    #[derive(Debug)]
    pub struct IntOpt {
        id: IIdentifier,

        name: String,

        prefix: String,

        optional: bool,

        value: OptValue,

        default_value: OptValue,

        alias: Vec<(String, String)>,

        callback: CallbackType,
    }

    impl IntOpt {
        pub fn new(id: IIdentifier, name: String, prefix: String, optional: bool, default_value: OptValue) -> Self {
            Self {
                id,
                name,
                prefix,
                optional,
                value: default_value.clone_or(&None),
                default_value,
                alias: vec![],
                callback: CallbackType::Null,
            }
        }
    }

    opt_def!(IntOpt, Int);

    opt_type_def!(
        IntOpt, 
        current_type(),
        false,
        { style, Style::Argument }
    );

    opt_callback_def!(
        IntOpt,
        callback,
        callback,
        CallbackType::Value,
        CallbackType::Null,
    );

    opt_identifier_def!(
        IntOpt,
        id,
        para,
    );

    opt_name_def!(
        IntOpt,
        prefix,
        name,
        prefix,
        name,
    );

    opt_optional_def!(
        IntOpt,
        optional,
        optional,
    );

    opt_alias_def!(
        IntOpt,
        alias,
        prefix,
        name,
    );

    opt_index_def!( IntOpt );

    impl Value for IntOpt {
        fn value(&self) -> &OptValue {
            &self.value
        }

        fn default_value(&self) -> &OptValue {
            &self.default_value
        }

        fn set_value(&mut self, value_para: OptValue) {
            self.value = value_para;
        }

        fn set_default_value(&mut self, default_value_para: OptValue) {
            self.default_value = default_value_para;
        }

        fn parse_value(&self, value_para: &str) -> Result<OptValue> {
            return OptValue::parse_int(value_para);
        }

        fn has_value(&self) -> bool {
            self.value().is_int()
        }

        fn reset_value(&mut self) {
            self.set_value(self.default_value().clone());
        }
    }

    /// Default [`Utils`] implementation for [`IntOpt`].
    #[derive(Debug)]
    pub struct IntUtils;

    impl IntUtils {
        pub fn new() -> Self {
            Self {}
        }
    }

    impl Utils for IntUtils {
        fn type_name(&self) -> &str {
            current_type()
        }

        fn is_support_deactivate_style(&self) -> bool {
            false
        }

        /// Create an [`IntOpt`] using option information [`CreateInfo`].
        /// 
        /// ```no_run
        /// use getopt_rs::utils::{Utils, CreateInfo};
        /// use getopt_rs::opt::int::*;
        /// use getopt_rs::id::*;
        /// 
        /// let prefixs = vec![String::from("--")];
        /// let utils = IntUtils::new();
        /// let ci = CreateInfo::parse("--name=int!", &prefixs).unwrap();
        /// let _opt = utils.create(Identifier::new(1), &ci);
        /// ```
        fn create(&self, id: IIdentifier, ci: &CreateInfo) -> Result<Box<dyn Opt>> {
            if ci.is_deactivate_style() {
                if ! self.is_support_deactivate_style() {
                    return Err(Error::UtilsNotSupportDeactivateStyle(ci.get_name().to_owned()));
                }
            }
            if ci.get_type_name() != self.type_name() {
                return Err(Error::UtilsNotSupportTypeName(self.type_name().to_owned(), ci.get_type_name().to_owned()))
            }
            
            assert_eq!(ci.get_type_name(), self.type_name());

            let mut opt = Box::new(IntOpt::new(
                id,
                ci.get_name().to_owned(),
                ci.get_prefix().to_owned(),
                ci.is_optional(),
                ci.get_default_value().clone_or(&None),
            ));

            let alias = ci.get_alias();

            if alias.len() > 0 {
                for a in alias.iter() {
                    opt.add_alias(&a.0, &a.1);
                }
            }

            Ok(opt)
        }

        fn gen_info(&self, opt: &dyn Opt) -> Box<dyn Info> {
            Box::new(OptionInfo::new(opt.id()))
        }
    }
}

pub mod uint {
    use crate::opt::*;
    use crate::id::Identifier as IIdentifier;

    pub fn current_type() -> &'static str {
        "uint"
    }

    pub trait Uint: Opt { }

    /// UintOpt target the value to [`u64`], 
    /// 
    /// * The option type name is `uint`.
    /// * The option is not support deactivate style.
    /// * The option accept the style [`Style::Argument`].
    /// * In default, the option is `optional`, it can be change through the [`set_optional`](crate::opt::Optional::set_optional).
    /// * The option need an [`OptValue::Uint`] argument, the default value is [`OptValue::default()`].
    /// * The option support multiple alias with different prefix and name.
    /// * The option support callback type [`CallbackType::Value`].
    ///
    /// User can set it at `anywhere` of command line argument, using the string `-c 1`, `-c=2`, `--count 4`, `--count=8`, etc.
    #[derive(Debug)]
    pub struct UintOpt {
        id: IIdentifier,

        name: String,

        prefix: String,

        optional: bool,

        value: OptValue,

        default_value: OptValue,

        alias: Vec<(String, String)>,

        callback: CallbackType,
    }

    impl UintOpt {
        pub fn new(id: IIdentifier, name: String, prefix: String, optional: bool, default_value: OptValue) -> Self {
            Self {
                id,
                name,
                prefix,
                optional,
                value: default_value.clone_or(&None),
                default_value,
                alias: vec![],
                callback: CallbackType::Null,
            }
        }
    }

    opt_def!(UintOpt, Uint);

    opt_type_def!(
        UintOpt, 
        current_type(),
        false,
        { style, Style::Argument }
    );

    opt_callback_def!(
        UintOpt,
        callback,
        callback,
        CallbackType::Value,
        CallbackType::Null,
    );

    opt_identifier_def!(
        UintOpt,
        id,
        para,
    );

    opt_name_def!(
        UintOpt,
        prefix,
        name,
        prefix,
        name,
    );

    opt_optional_def!(
        UintOpt,
        optional,
        optional,
    );

    opt_alias_def!(
        UintOpt,
        alias,
        prefix,
        name,
    );

    opt_index_def!( UintOpt );

    impl Value for UintOpt {
        fn value(&self) -> &OptValue {
            &self.value
        }

        fn default_value(&self) -> &OptValue {
            &self.default_value
        }

        fn set_value(&mut self, value_para: OptValue) {
            self.value = value_para;
        }

        fn set_default_value(&mut self, default_value_para: OptValue) {
            self.default_value = default_value_para;
        }

        fn parse_value(&self, value_para: &str) -> Result<OptValue> {
            return OptValue::parse_uint(value_para);
        }

        fn has_value(&self) -> bool {
            self.value().is_uint()
        }

        fn reset_value(&mut self) {
            self.set_value(self.default_value().clone());
        }
    }

    /// Default [`Utils`] implementation for [`UintOpt`].
    #[derive(Debug)]
    pub struct UintUtils;

    impl UintUtils {
        pub fn new() -> Self {
            Self {}
        }
    }

    impl Utils for UintUtils {
        fn type_name(&self) -> &str {
            current_type()
        }

        fn is_support_deactivate_style(&self) -> bool {
            false
        }

        /// Create an [`UintOpt`] using option information [`CreateInfo`].
        /// 
        /// ```no_run
        /// use getopt_rs::utils::{Utils, CreateInfo};
        /// use getopt_rs::opt::uint::*;
        /// use getopt_rs::id::*;
        /// 
        /// let prefixs = vec![String::from("--")];
        /// let utils = UintUtils::new();
        /// let ci = CreateInfo::parse("--name=uint!", &prefixs).unwrap();
        /// let _opt = utils.create(Identifier::new(1), &ci);
        /// ```
        fn create(&self, id: IIdentifier, ci: &CreateInfo) -> Result<Box<dyn Opt>> {
            if ci.is_deactivate_style() {
                if ! self.is_support_deactivate_style() {
                    return Err(Error::UtilsNotSupportDeactivateStyle(ci.get_name().to_owned()));
                }
            }
            if ci.get_type_name() != self.type_name() {
                return Err(Error::UtilsNotSupportTypeName(self.type_name().to_owned(), ci.get_type_name().to_owned()))
            }
            
            assert_eq!(ci.get_type_name(), self.type_name());

            let mut opt = Box::new(UintOpt::new(
                id,
                ci.get_name().to_owned(),
                ci.get_prefix().to_owned(),
                ci.is_optional(),
                ci.get_default_value().clone_or(&None),
            ));

            let alias = ci.get_alias();

            if alias.len() > 0 {
                for a in alias.iter() {
                    opt.add_alias(&a.0, &a.1);
                }
            }

            Ok(opt)
        }

        fn gen_info(&self, opt: &dyn Opt) -> Box<dyn Info> {
            Box::new(OptionInfo::new(opt.id()))
        }
    }
}

pub mod flt {
    use crate::opt::*;
    use crate::id::Identifier as IIdentifier;

    pub fn current_type() -> &'static str {
        "flt"
    }

    pub trait Flt: Opt { }

    /// FltOpt target the value to [`f64`], 
    /// 
    /// * The option type name is `flt`.
    /// * The option is not support deactivate style.
    /// * The option accept the style [`Style::Argument`].
    /// * In default, the option is `optional`, it can be change through the [`set_optional`](crate::opt::Optional::set_optional).
    /// * The option need an [`OptValue::Flt`] argument, the default value is [`OptValue::default()`].
    /// * The option support multiple alias with different prefix and name.
    /// * The option support callback type [`CallbackType::Value`].
    ///
    /// User can set it at `anywhere` of command line argument, using the string `-c 1.1`, `-c -2.2`, `-c=2.4`, `--count 4.2`, `--count=8.8`, etc.
    #[derive(Debug)]
    pub struct FltOpt {
        id: IIdentifier,

        name: String,

        prefix: String,

        optional: bool,

        value: OptValue,

        default_value: OptValue,

        alias: Vec<(String, String)>,

        callback: CallbackType,
    }

    impl FltOpt {
        pub fn new(id: IIdentifier, name: String, prefix: String, optional: bool, default_value: OptValue) -> Self {
            Self {
                id,
                name,
                prefix,
                optional,
                value: default_value.clone_or(&None),
                default_value,
                alias: vec![],
                callback: CallbackType::Null,
            }
        }
    }

    opt_def!(FltOpt, Flt);

    opt_type_def!(
        FltOpt, 
        current_type(),
        false,
        { style, Style::Argument }
    );

    opt_callback_def!(
        FltOpt,
        callback,
        callback,
        CallbackType::Value,
        CallbackType::Null,
    );

    opt_identifier_def!(
        FltOpt,
        id,
        para,
    );

    opt_name_def!(
        FltOpt,
        prefix,
        name,
        prefix,
        name,
    );

    opt_optional_def!(
        FltOpt,
        optional,
        optional,
    );

    opt_alias_def!(
        FltOpt,
        alias,
        prefix,
        name,
    );

    opt_index_def!( FltOpt );

    impl Value for FltOpt {
        fn value(&self) -> &OptValue {
            &self.value
        }

        fn default_value(&self) -> &OptValue {
            &self.default_value
        }

        fn set_value(&mut self, value_para: OptValue) {
            self.value = value_para;
        }

        fn set_default_value(&mut self, default_value_para: OptValue) {
            self.default_value = default_value_para;
        }

        fn parse_value(&self, value_para: &str) -> Result<OptValue> {
            return OptValue::parse_flt(value_para);
        }

        fn has_value(&self) -> bool {
            self.value().is_flt()
        }

        fn reset_value(&mut self) {
            self.set_value(self.default_value().clone());
        }
    }

    /// Default [`Utils`] implementation for [`FltOpt`].
    #[derive(Debug)]
    pub struct FltUtils;

    impl FltUtils {
        pub fn new() -> Self {
            Self {}
        }
    }

    impl Utils for FltUtils {
        fn type_name(&self) -> &str {
            current_type()
        }

        fn is_support_deactivate_style(&self) -> bool {
            false
        }

        /// Create an [`FltOpt`] using option information [`CreateInfo`].
        /// 
        /// ```no_run
        /// use getopt_rs::utils::{Utils, CreateInfo};
        /// use getopt_rs::opt::flt::*;
        /// use getopt_rs::id::*;
        /// 
        /// let prefixs = vec![String::from("--")];
        /// let utils = FltUtils::new();
        /// let ci = CreateInfo::parse("--name=flt!", &prefixs).unwrap();
        /// let _opt = utils.create(Identifier::new(1), &ci);
        /// ```
        fn create(&self, id: IIdentifier, ci: &CreateInfo) -> Result<Box<dyn Opt>> {if ci.is_deactivate_style() {
                if ! self.is_support_deactivate_style() {
                    return Err(Error::UtilsNotSupportDeactivateStyle(ci.get_name().to_owned()));
                }
            }
            if ci.get_type_name() != self.type_name() {
                return Err(Error::UtilsNotSupportTypeName(self.type_name().to_owned(), ci.get_type_name().to_owned()))
            }
            
            assert_eq!(ci.get_type_name(), self.type_name());


            let mut opt = Box::new(FltOpt::new(
                id,
                ci.get_name().to_owned(),
                ci.get_prefix().to_owned(),
                ci.is_optional(),
                ci.get_default_value().clone_or(&None),
            ));

            let alias = ci.get_alias();

            if alias.len() > 0 {
                for a in alias.iter() {
                    opt.add_alias(&a.0, &a.1);
                }
            }

            Ok(opt)
        }

        fn gen_info(&self, opt: &dyn Opt) -> Box<dyn Info> {
            Box::new(OptionInfo::new(opt.id()))
        }
    }
}

pub mod example {
    use std::path::PathBuf;
    use crate::opt::*;
    use crate::id::Identifier as IIdentifier;

    pub fn current_type() -> &'static str {
        "path"
    }

    pub trait Path: Opt { }

    #[derive(Debug)]
    pub struct PathOpt {
        id: IIdentifier,

        name: String,

        prefix: String,

        optional: bool,

        value: OptValue,

        default_value: OptValue,

        alias: Vec<(String, String)>,

        callback: CallbackType,
    }

    impl PathOpt {
        pub fn new(id: IIdentifier, name: String, prefix: String, optional: bool, default_value: OptValue) -> Self {
            Self {
                id,
                name,
                prefix,
                optional,
                value: default_value.clone_or(&None),
                default_value,
                alias: vec![],
                callback: CallbackType::Null,
            }
        }
    }

    opt_def!(PathOpt, Path);

    opt_type_def!(
        PathOpt, 
        current_type(),
        false,
        { style, Style::Argument }
    );

    opt_callback_def!(
        PathOpt,
        callback,
        callback,
        CallbackType::Value,
        CallbackType::Null,
    );

    opt_identifier_def!(
        PathOpt,
        id,
        para,
    );

    opt_name_def!(
        PathOpt,
        prefix,
        name,
        prefix,
        name,
    );

    opt_optional_def!(
        PathOpt,
        optional,
        optional,
    );

    opt_alias_def!(
        PathOpt,
        alias,
        prefix,
        name,
    );

    opt_index_def!( PathOpt );

    impl Value for PathOpt {
        fn value(&self) -> &OptValue {
            &self.value
        }

        fn default_value(&self) -> &OptValue {
            &self.default_value
        }

        fn set_value(&mut self, value_para: OptValue) {
            self.value = value_para;
        }

        fn set_default_value(&mut self, default_value_para: OptValue) {
            self.default_value = default_value_para;
        }

        fn parse_value(&self, value_para: &str) -> Result<OptValue> {
            let pathbuf = PathBuf::from(value_para);

            if ! pathbuf.exists() {
                return Err(Error::InvaldOptionValue(value_para.to_owned(), format!("the path is not eixst")));
            }
            return Ok(OptValue::from_any(Box::new(pathbuf)));
        }

        fn has_value(&self) -> bool {
            self.value().is_any()
        }

        fn reset_value(&mut self) {
            self.set_value(self.default_value().clone());
        }
    }

    #[derive(Debug)]
    pub struct PathUtils;

    impl PathUtils {
        pub fn new() -> Self {
            Self {}
        }
    }

    impl Utils for PathUtils {
        fn type_name(&self) -> &str {
            current_type()
        }

        fn is_support_deactivate_style(&self) -> bool {
            false
        }

        fn create(&self, id: IIdentifier, ci: &CreateInfo) -> Result<Box<dyn Opt>> {
            if ci.is_deactivate_style() {
                if ! self.is_support_deactivate_style() {
                    return Err(Error::UtilsNotSupportDeactivateStyle(ci.get_name().to_owned()));
                }
            }
            if ci.get_type_name() != self.type_name() {
                return Err(Error::UtilsNotSupportTypeName(self.type_name().to_owned(), ci.get_type_name().to_owned()))
            }
            
            assert_eq!(ci.get_type_name(), self.type_name());
            
            let clone_helper: CloneHelper = CloneHelper(Box::new(
                |pathbuf: & dyn std::any::Any| {
                    Box::new(pathbuf.downcast_ref::<PathBuf>().unwrap().clone())
                }
            ));

            let mut opt = Box::new(PathOpt::new(
                id,
                ci.get_name().to_owned(),
                ci.get_prefix().to_owned(),
                ci.is_optional(),
                ci.get_default_value().clone_or(&Some(clone_helper)),
            ));

            let alias = ci.get_alias();

            if alias.len() > 0 {
                for a in alias.iter() {
                    opt.add_alias(&a.0, &a.1);
                }
            }

            Ok(opt)
        }

        fn gen_info(&self, opt: &dyn Opt) -> Box<dyn Info> {
            Box::new(OptionInfo::new(opt.id()))
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn make_opt_type_int_work() {
        let prefixs = vec!["--".to_owned()];
        let int_utils = int::IntUtils::new();
        
        assert_eq!(int_utils.type_name(), int::current_type());
        assert_eq!(int_utils.is_support_deactivate_style(), false);
        
        let ci = CreateInfo::parse("--opt=int!", &prefixs).unwrap();
        let mut opt = int_utils.create(IIdentifier::new(1), &ci).unwrap();

        assert_eq!(opt.type_name(), "int");
        assert_eq!(opt.is_deactivate_style(), false);
        assert_eq!(opt.is_style(Style::Argument), true);
        assert_eq!(opt.check().is_err(), true);

        assert_eq!(opt.id().get(), 1);
        opt.set_id(IIdentifier::new(42));
        assert_eq!(opt.id().get(), 42);

        assert_eq!(opt.callback_type(), CallbackType::Null);
        assert_eq!(opt.is_need_invoke(), false);
        opt.set_need_invoke(true);
        assert_eq!(opt.callback_type(), CallbackType::Value);
        assert_eq!(opt.is_need_invoke(), true);

        opt.add_alias("-", "c");
        assert_eq!(opt.alias(), Some(&vec![(String::from("-"), String::from("c"))]));
        assert_eq!(opt.match_alias("-", "c"), true);
        assert_eq!(opt.rem_alias("-", "c"), true);
        assert_eq!(opt.alias().as_ref().unwrap().len(), 0);

        assert_eq!(opt.index(), &NonOptIndex::Null);
        assert_eq!(opt.match_index(0, 0), true);
        opt.set_index(NonOptIndex::Forward(3));
        assert_eq!(opt.match_index(0, 0), true);

        assert_eq!(opt.name(), "opt");
        assert_eq!(opt.prefix(), "--");
        assert_eq!(opt.match_name("opt"), true);
        assert_eq!(opt.match_name("opv"), false);
        assert_eq!(opt.match_prefix("--"), true);
        assert_eq!(opt.match_prefix("-"), false);
        opt.set_name("count");
        opt.set_prefix("+");
        assert_eq!(opt.match_name("count"), true);
        assert_eq!(opt.match_name("opt"), false);
        assert_eq!(opt.match_prefix("+"), true);
        assert_eq!(opt.match_prefix("--"), false);

        assert_eq!(opt.optional(), false);
        assert_eq!(opt.match_optional(true), false);
        opt.set_optional(true);
        assert_eq!(opt.optional(), true);
        assert_eq!(opt.match_optional(true), true);

        assert_eq!(opt.value().is_null(), true);
        assert_eq!(opt.default_value().is_null(), true);
        assert_eq!(opt.has_value(), false);
        opt.set_value(opt.parse_value("99").unwrap());
        assert_eq!(opt.value().as_int(), Some(&99));
        opt.set_default_value(OptValue::from_int(-42));
        assert_eq!(opt.default_value().as_int(), Some(&-42));
        opt.reset_value();
        assert_eq!(opt.value().as_int(), Some(&-42));

        assert_eq!(opt.as_ref().as_any().is::<int::IntOpt>(), true);
    }

    #[test]
    fn make_opt_type_uint_work() {
        let prefixs = vec!["--".to_owned()];
        let uint_utils = uint::UintUtils::new();
        
        assert_eq!(uint_utils.type_name(), uint::current_type());
        assert_eq!(uint_utils.is_support_deactivate_style(), false);
        
        let ci = CreateInfo::parse("--opt=uint!", &prefixs).unwrap();
        let mut opt = uint_utils.create(IIdentifier::new(1), &ci).unwrap();

        assert_eq!(opt.type_name(), "uint");
        assert_eq!(opt.is_deactivate_style(), false);
        assert_eq!(opt.is_style(Style::Argument), true);
        assert_eq!(opt.check().is_err(), true);

        assert_eq!(opt.id().get(), 1);
        opt.set_id(IIdentifier::new(42));
        assert_eq!(opt.id().get(), 42);

        assert_eq!(opt.callback_type(), CallbackType::Null);
        assert_eq!(opt.is_need_invoke(), false);
        opt.set_need_invoke(true);
        assert_eq!(opt.callback_type(), CallbackType::Value);
        assert_eq!(opt.is_need_invoke(), true);

        opt.add_alias("-", "c");
        assert_eq!(opt.alias(), Some(&vec![(String::from("-"), String::from("c"))]));
        assert_eq!(opt.match_alias("-", "c"), true);
        assert_eq!(opt.rem_alias("-", "c"), true);
        assert_eq!(opt.alias().as_ref().unwrap().len(), 0);

        assert_eq!(opt.index(), &NonOptIndex::Null);
        assert_eq!(opt.match_index(0, 0), true);
        opt.set_index(NonOptIndex::Forward(3));
        assert_eq!(opt.match_index(0, 0), true);

        assert_eq!(opt.name(), "opt");
        assert_eq!(opt.prefix(), "--");
        assert_eq!(opt.match_name("opt"), true);
        assert_eq!(opt.match_name("opv"), false);
        assert_eq!(opt.match_prefix("--"), true);
        assert_eq!(opt.match_prefix("-"), false);
        opt.set_name("count");
        opt.set_prefix("+");
        assert_eq!(opt.match_name("count"), true);
        assert_eq!(opt.match_name("opt"), false);
        assert_eq!(opt.match_prefix("+"), true);
        assert_eq!(opt.match_prefix("--"), false);

        assert_eq!(opt.optional(), false);
        assert_eq!(opt.match_optional(true), false);
        opt.set_optional(true);
        assert_eq!(opt.optional(), true);
        assert_eq!(opt.match_optional(true), true);

        assert_eq!(opt.value().is_null(), true);
        assert_eq!(opt.default_value().is_null(), true);
        assert_eq!(opt.has_value(), false);
        opt.set_value(opt.parse_value("99").unwrap());
        assert_eq!(opt.value().as_uint(), Some(&99));
        opt.set_default_value(OptValue::from_uint(42u64));
        assert_eq!(opt.default_value().as_uint(), Some(&42));
        opt.reset_value();
        assert_eq!(opt.value().as_uint(), Some(&42));

        assert_eq!(opt.as_ref().as_any().is::<uint::UintOpt>(), true);
    }

    #[test]
    fn make_opt_type_flt_work() {
        let prefixs = vec!["--".to_owned()];
        let flt_utils = flt::FltUtils::new();
        
        assert_eq!(flt_utils.type_name(), flt::current_type());
        assert_eq!(flt_utils.is_support_deactivate_style(), false);
        
        let ci = CreateInfo::parse("--opt=flt!", &prefixs).unwrap();
        let mut opt = flt_utils.create(IIdentifier::new(1), &ci).unwrap();

        assert_eq!(opt.type_name(), "flt");
        assert_eq!(opt.is_deactivate_style(), false);
        assert_eq!(opt.is_style(Style::Argument), true);
        assert_eq!(opt.check().is_err(), true);

        assert_eq!(opt.id().get(), 1);
        opt.set_id(IIdentifier::new(42));
        assert_eq!(opt.id().get(), 42);

        assert_eq!(opt.callback_type(), CallbackType::Null);
        assert_eq!(opt.is_need_invoke(), false);
        opt.set_need_invoke(true);
        assert_eq!(opt.callback_type(), CallbackType::Value);
        assert_eq!(opt.is_need_invoke(), true);

        opt.add_alias("-", "c");
        assert_eq!(opt.alias(), Some(&vec![(String::from("-"), String::from("c"))]));
        assert_eq!(opt.match_alias("-", "c"), true);
        assert_eq!(opt.rem_alias("-", "c"), true);
        assert_eq!(opt.alias().as_ref().unwrap().len(), 0);

        assert_eq!(opt.index(), &NonOptIndex::Null);
        assert_eq!(opt.match_index(0, 0), true);
        opt.set_index(NonOptIndex::Forward(3));
        assert_eq!(opt.match_index(0, 0), true);

        assert_eq!(opt.name(), "opt");
        assert_eq!(opt.prefix(), "--");
        assert_eq!(opt.match_name("opt"), true);
        assert_eq!(opt.match_name("opv"), false);
        assert_eq!(opt.match_prefix("--"), true);
        assert_eq!(opt.match_prefix("-"), false);
        opt.set_name("count");
        opt.set_prefix("+");
        assert_eq!(opt.match_name("count"), true);
        assert_eq!(opt.match_name("opt"), false);
        assert_eq!(opt.match_prefix("+"), true);
        assert_eq!(opt.match_prefix("--"), false);

        assert_eq!(opt.optional(), false);
        assert_eq!(opt.match_optional(true), false);
        opt.set_optional(true);
        assert_eq!(opt.optional(), true);
        assert_eq!(opt.match_optional(true), true);

        assert_eq!(opt.value().is_null(), true);
        assert_eq!(opt.default_value().is_null(), true);
        assert_eq!(opt.has_value(), false);
        opt.set_value(opt.parse_value("99.9").unwrap());
        assert_eq!(opt.value().as_flt(), Some(&99.9));
        opt.set_default_value(OptValue::from_flt(42.6));
        assert_eq!(opt.default_value().as_flt(), Some(&42.6));
        opt.reset_value();
        assert_eq!(opt.value().as_flt(), Some(&42.6));

        assert_eq!(opt.as_ref().as_any().is::<flt::FltOpt>(), true);
    }

    #[test]
    fn make_opt_type_bool_work() {
        let prefixs = vec!["--".to_owned()];
        let bool_utils = bool::BoolUtils::new();
        
        assert_eq!(bool_utils.type_name(), bool::current_type());
        assert_eq!(bool_utils.is_support_deactivate_style(), true);
        
        let ci = CreateInfo::parse("--opt=bool!/", &prefixs).unwrap();
        let mut opt = bool_utils.create(IIdentifier::new(1), &ci).unwrap();

        assert_eq!(opt.type_name(), "bool");
        assert_eq!(opt.is_deactivate_style(), true);
        assert_eq!(opt.is_style(Style::Boolean), true);
        assert_eq!(opt.is_style(Style::Multiple), true);
        assert_eq!(opt.check().is_err(), true);

        assert_eq!(opt.id().get(), 1);
        opt.set_id(IIdentifier::new(42));
        assert_eq!(opt.id().get(), 42);

        assert_eq!(opt.callback_type(), CallbackType::Null);
        assert_eq!(opt.is_need_invoke(), false);
        opt.set_need_invoke(true);
        assert_eq!(opt.callback_type(), CallbackType::Value);
        assert_eq!(opt.is_need_invoke(), true);

        opt.add_alias("-", "c");
        assert_eq!(opt.alias(), Some(&vec![(String::from("-"), String::from("c"))]));
        assert_eq!(opt.match_alias("-", "c"), true);
        assert_eq!(opt.rem_alias("-", "c"), true);
        assert_eq!(opt.alias().as_ref().unwrap().len(), 0);

        assert_eq!(opt.index(), &NonOptIndex::Null);
        assert_eq!(opt.match_index(0, 0), true);
        opt.set_index(NonOptIndex::Forward(3));
        assert_eq!(opt.match_index(0, 0), true);

        assert_eq!(opt.name(), "opt");
        assert_eq!(opt.prefix(), "--");
        assert_eq!(opt.match_name("opt"), true);
        assert_eq!(opt.match_name("opv"), false);
        assert_eq!(opt.match_prefix("--"), true);
        assert_eq!(opt.match_prefix("-"), false);
        opt.set_name("count");
        opt.set_prefix("+");
        assert_eq!(opt.match_name("count"), true);
        assert_eq!(opt.match_name("opt"), false);
        assert_eq!(opt.match_prefix("+"), true);
        assert_eq!(opt.match_prefix("--"), false);

        assert_eq!(opt.optional(), false);
        assert_eq!(opt.match_optional(true), false);
        opt.set_optional(true);
        assert_eq!(opt.optional(), true);
        assert_eq!(opt.match_optional(true), true);

        assert_eq!(opt.value().is_null(), false);
        assert_eq!(opt.default_value().is_null(), false);
        assert_eq!(opt.value().is_bool(), true);
        assert_eq!(opt.default_value().is_bool(), true);
        assert_eq!(opt.has_value(), false);
        opt.set_value(OptValue::from_bool(false));
        assert_eq!(opt.value().as_bool(), Some(&false));
        opt.set_default_value(OptValue::from_bool(true));
        assert_eq!(opt.default_value().as_bool(), Some(&true));
        opt.reset_value();
        assert_eq!(opt.value().as_bool(), Some(&true));

        assert_eq!(opt.as_ref().as_any().is::<bool::BoolOpt>(), true);
    }

    #[test]
    fn make_opt_type_array_work() {
        let prefixs = vec!["--".to_owned()];
        let bool_utils = array::ArrayUtils::new();
        
        assert_eq!(bool_utils.type_name(), array::current_type());
        assert_eq!(bool_utils.is_support_deactivate_style(), false);
        
        let ci = CreateInfo::parse("--opt=array!", &prefixs).unwrap();
        let mut opt = bool_utils.create(IIdentifier::new(1), &ci).unwrap();

        assert_eq!(opt.type_name(), "array");
        assert_eq!(opt.is_deactivate_style(), false);
        assert_eq!(opt.is_style(Style::Argument), true);
        assert_eq!(opt.check().is_err(), true);

        assert_eq!(opt.id().get(), 1);
        opt.set_id(IIdentifier::new(42));
        assert_eq!(opt.id().get(), 42);

        assert_eq!(opt.callback_type(), CallbackType::Null);
        assert_eq!(opt.is_need_invoke(), false);
        opt.set_need_invoke(true);
        assert_eq!(opt.callback_type(), CallbackType::Value);
        assert_eq!(opt.is_need_invoke(), true);

        opt.add_alias("-", "c");
        assert_eq!(opt.alias(), Some(&vec![(String::from("-"), String::from("c"))]));
        assert_eq!(opt.match_alias("-", "c"), true);
        assert_eq!(opt.rem_alias("-", "c"), true);
        assert_eq!(opt.alias().as_ref().unwrap().len(), 0);

        assert_eq!(opt.index(), &NonOptIndex::Null);
        assert_eq!(opt.match_index(0, 0), true);
        opt.set_index(NonOptIndex::Forward(3));
        assert_eq!(opt.match_index(0, 0), true);

        assert_eq!(opt.name(), "opt");
        assert_eq!(opt.prefix(), "--");
        assert_eq!(opt.match_name("opt"), true);
        assert_eq!(opt.match_name("opv"), false);
        assert_eq!(opt.match_prefix("--"), true);
        assert_eq!(opt.match_prefix("-"), false);
        opt.set_name("count");
        opt.set_prefix("+");
        assert_eq!(opt.match_name("count"), true);
        assert_eq!(opt.match_name("opt"), false);
        assert_eq!(opt.match_prefix("+"), true);
        assert_eq!(opt.match_prefix("--"), false);

        assert_eq!(opt.optional(), false);
        assert_eq!(opt.match_optional(true), false);
        opt.set_optional(true);
        assert_eq!(opt.optional(), true);
        assert_eq!(opt.match_optional(true), true);

        assert_eq!(opt.value().is_null(), true);
        assert_eq!(opt.default_value().is_null(), true);
        assert_eq!(opt.has_value(), false);
        opt.set_value(OptValue::from_vec(
            ["foo", "bar"].iter()
                          .map(|&i|String::from(i))
                          .collect::<Vec<String>>()
        ));
        assert_eq!(opt.value().as_vec(), 
            Some(&["foo", "bar"].iter().map(|&i|String::from(i)).collect()));
        opt.set_default_value(OptValue::from_vec(
            ["foo", "bar", "poi"].iter()
                .map(|&i|String::from(i))
                .collect::<Vec<String>>()
        ));
        assert_eq!(opt.default_value().as_vec(), 
            Some(&["foo", "bar", "poi"].iter().map(|&i|String::from(i)).collect::<Vec<String>>()));
        opt.reset_value();
        assert_eq!(opt.value().as_vec(), 
            Some(&["foo", "bar", "poi"].iter().map(|&i|String::from(i)).collect::<Vec<String>>()));

        assert_eq!(opt.as_ref().as_any().is::<array::ArrayOpt>(), true);
    }

    #[test]
    fn make_optvalue_work() {
        make_optvalue_int_work();
        make_optvalue_uint_work();
        make_optvalue_str_work();
        make_optvalue_null_work();
        make_optvalue_flt_work();
        make_optvalue_bool_work();
        make_optvalue_arr_work();
        make_optvalue_any_work();
    }

    fn make_optvalue_int_work() {
        let mut value = OptValue::from_int(25);

        assert!(value.is_int());
        assert_eq!(value.as_int(), Some(&25));
        assert_eq!(value.as_int_mut(), Some(&mut 25));

        let test_cases = &[ value.is_uint(), value.is_str(), value.is_null(),
                                      value.is_flt(), value.is_bool(), value.is_array(), value.is_any() ];

        for r in test_cases {
            assert!(! r);
        }

        let test_cases = &[ value.as_uint().is_none(), value.as_str().is_none(), value.as_bool_or_null().is_none(),
                                      value.as_flt().is_none(), value.as_bool().is_none(), value.as_vec().is_none(), value.as_any().is_none() ];

        for r in test_cases {
            assert!(r);
        }

        let mut value = OptValue::from_int(-25);

        assert!(value.is_int());
        assert_eq!(value.as_int(), Some(&-25));
        assert_eq!(value.as_int_mut(), Some(&mut -25));

        let test_cases = &[ value.is_uint(), value.is_str(), value.is_null(),
                                      value.is_flt(), value.is_bool(), value.is_array(), value.is_any() ];

        for r in test_cases {
            assert!(! r);
        }

        let test_cases = &[ value.as_uint().is_none(), value.as_str().is_none(), value.as_bool_or_null().is_none(),
                                      value.as_flt().is_none(), value.as_bool().is_none(), value.as_vec().is_none(), value.as_any().is_none() ];

        for r in test_cases {
            assert!(r);
        }
    }

    fn make_optvalue_uint_work() {
        let mut value = OptValue::from_uint(33u64);

        assert!(value.is_uint());
        assert_eq!(value.as_uint(), Some(&33));
        assert_eq!(value.as_uint_mut(), Some(&mut 33));

        let test_cases = &[ value.is_int(), value.is_str(), value.is_null(), value.is_flt(), value.is_bool(), value.is_array(), value.is_any() ];

        for r in test_cases {
            assert!(! r);
        }
    }

    fn make_optvalue_str_work() {
        let mut value = OptValue::from_str("value");

        assert!(value.is_str());
        assert_eq!(value.as_str(), Some(&String::from("value")));
        assert_eq!(value.as_str_mut(), Some(&mut String::from("value")));

        let test_cases = &[ value.is_int(), value.is_uint(), value.is_null(), value.is_flt(), value.is_bool(), value.is_array(), value.is_any() ];

        for r in test_cases {
            assert!(! r);
        }
    }

    fn make_optvalue_null_work() {
        let value = OptValue::null();

        assert!(value.is_null());
        assert_eq!(value.as_bool_or_null(), Some(&false));

        let test_cases = &[ value.is_int(), value.is_uint(), value.is_str(), value.is_flt(), value.is_bool(), value.is_array(), value.is_any() ];

        for r in test_cases {
            assert!(! r);
        }
    }

    fn make_optvalue_flt_work() {
        let mut value = OptValue::from_flt(1.7);

        assert!(value.is_flt());
        assert_eq!(value.as_flt(), Some(&1.7));
        assert_eq!(value.as_flt_mut(), Some(&mut 1.7));

        let test_cases = &[ value.is_int(), value.is_uint(), value.is_null(), value.is_str(), value.is_bool(), value.is_array(), value.is_any() ];

        for r in test_cases {
            assert!(! r);
        }
    }

    fn make_optvalue_bool_work() {
        let mut value = OptValue::from_bool(true);

        assert!(value.is_bool());
        assert_eq!(value.as_bool(), Some(&true));
        assert_eq!(value.as_bool_mut(), Some(&mut true));
        assert_eq!(value.as_bool_or_null(), Some(&true));

        let test_cases = &[ value.is_int(), value.is_uint(), value.is_null(), value.is_flt(), value.is_str(), value.is_array(), value.is_any() ];

        for r in test_cases {
            assert!(! r);
        }
    }

    fn make_optvalue_arr_work() {
        let mut data: Vec<String> = ["v1", "v2", "v3", "v4"].iter().map(|&v| { String::from(v)}).collect();
        let mut value = OptValue::from_vec(data.clone());

        assert!(value.is_array());
        assert_eq!(value.as_vec(), Some(&data));
        assert_eq!(value.as_vec_mut(), Some(&mut data));

        let test_cases = &[ value.is_int(), value.is_uint(), value.is_null(), value.is_str(), value.is_bool(), value.is_str(), value.is_any() ];

        for r in test_cases {
            assert!(! r);
        }
    }

    fn make_optvalue_any_work() {
        #[derive(Debug, Clone, PartialEq)]
        struct InnerData(i64);

        let mut data = Box::new(InnerData(42));
        let mut value = OptValue::from_any(data.clone());

        assert!(value.is_any());
        assert_eq!(value.as_any().unwrap().as_ref().downcast_ref::<InnerData>(), Some(data.as_ref()));
        assert_eq!(value.as_any_mut().unwrap().as_mut().downcast_mut::<InnerData>(), Some(data.as_mut()));

        let test_cases = &[ value.is_int(), value.is_uint(), value.is_null(), value.is_str(), value.is_bool(), value.is_array(), value.is_str() ];

        for r in test_cases {
            assert!(! r);
        }
    }
}
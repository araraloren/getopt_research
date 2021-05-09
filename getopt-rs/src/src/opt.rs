
use std::fmt::Debug;
use std::any::Any;

use crate::callback::CallbackType;
use crate::id::Identifier as IIdentifier;
use crate::utils::Utils;
use crate::utils::CreateInfo;
use crate::proc::Info;
use crate::error::Error;
use crate::error::Result;

#[derive(Debug, Clone, PartialEq, Eq)]
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

    /// NonOption style
    Pos,

    /// NonOption style
    Cmd,

    /// NonOption style
    Main,

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

///
/// `NonOptionIndex` is the index of non-option arguments.
/// It is base on one.
/// For example, given command line arguments like `["rem", "-c", 5, "--force", "lucy"]`
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
    pub fn new(index: i64) -> Self {
        if index > 0 {
            Self::Forward(index)
        }
        else if index < 0 {
            Self::Backward(index)
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
                if offset <= &total {
                    return Some(*offset)
                }
            }
            NonOptIndex::Backward(offset) => {
                let realindex = total + *offset;
                
                if realindex > 0 {
                    return Some(realindex);
                }
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

/// Type hold specify information of an option type
pub trait Type: Any {
    /// Unique type name of current option type
    fn type_name(&self) -> &str;

    /// Boolean option that initialized with true,
    /// user can set(disable) them by using "-/o"
    fn is_deactivate_style(&self) -> bool;

    /// Retrun true if the option compatible with the style
    fn is_style(&self, style: Style) -> bool;

    /// Check if everything is fine
    fn check(&self) -> Result<bool>;
}

pub trait Identifier {
    /// Get an unique identifier of current option
    fn id(&self) -> IIdentifier;

    /// Set identifier to `id`
    fn set_id(&mut self, id: IIdentifier);
}

pub trait Callback {
    /// Get callback type of current option
    fn callback_type(&self) -> CallbackType;

    /// Return true if the callback need invoke
    fn is_need_invoke(&self) -> bool;

    /// Set invoke flag
    fn set_need_invoke(&mut self, invoke: bool);
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
    fn alias(&self) -> Option<&Vec<(String, String)>>;

    /// Add an new alias for current option
    fn add_alias(&mut self, prefix: &str, name: &str);

    /// Remove an alias of current option, return true if remove successful
    fn rem_alias(&mut self,  prefix: &str, name: &str) -> bool;

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
    fn parse_value(&self, v: &str) -> Result<OptValue>;

    /// Return true if the option has value setted
    fn has_value(&self) -> bool;

    /// Reset value
    fn reset_value(&mut self);
}

pub trait Index {
    fn index(&self) -> &NonOptIndex;

    fn set_index(&mut self, index: NonOptIndex);

    fn match_index(&self, total: i64, current: i64) -> bool;
}

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

    pub fn from_arr<T: Into<Vec<String>>>(t: T) -> Self {
        Self::Arr(t.into())
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
    pub fn as_arr_mut(&mut self) -> Option<&mut Vec<String>> {
        match self {
            Self::Arr(v) => Some(v),
            _ => None,
        }
    }

    pub fn app_value(&mut self, s: String) -> &mut Self {
        match self {
            Self::Arr(v) => {
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

            Self::Arr(vv) => { Self::Arr(vv.clone()) },

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
///                         .find(|&a| a.0 == prefix_para && a.1 == name_para)
///                         .is_some()
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
///         match self.index() {
///             NonOptIndex::Forward(offset) => {
///                 if offset <= &total {
///                     return offset == &current;
///                 }
///             }
///             NonOptIndex::Backward(offset) => {
///                 let realindex = total - offset;
///                    
///                 if realindex > 0 {
///                     return realindex == current;
///                 }
///             }
///             NonOptIndex::AnyWhere => {
///                 return true;
///             }
///             _ => { }
///         }
///         false
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

            fn match_index(&self, _: i64, _: i64) -> bool {
                true /* option can be set in anywhere */
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
                    return realindex == current;
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

        fn create(&self, id: IIdentifier, ci: &CreateInfo) -> Result<Box<dyn Opt>> {
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
        pub fn new(id: IIdentifier, name: String, prefix: String, optional: bool, deactivate_style: bool, default_value: OptValue) -> Self {
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

        fn has_value(&self) -> bool {
            self.value().is_bool()
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

        fn create(&self, id: IIdentifier, ci: &CreateInfo) -> Result<Box<dyn Opt>> {
            let mut opt = Box::new(BoolOpt::new(
                id,
                ci.get_name().to_owned(),
                ci.get_prefix().to_owned(),
                ci.is_optional(),
                ci.is_deactivate_style(),
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

pub mod arr {
    use crate::opt::*;
    use crate::id::Identifier as IIdentifier;

    pub fn current_type() -> &'static str {
        "arr"
    }

    pub trait Arr: Opt { }

    #[derive(Debug)]
    pub struct ArrOpt {
        id: IIdentifier,

        name: String,

        prefix: String,

        optional: bool,

        value: OptValue,

        default_value: OptValue,

        alias: Vec<(String, String)>,

        callback: CallbackType,
    }

    impl ArrOpt {
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

    opt_def!(ArrOpt, Arr);

    opt_type_def!(
        ArrOpt, 
        current_type(),
        false,
        { style, Style::Argument }
    );

    opt_callback_def!(
        ArrOpt,
        callback,
        callback,
        CallbackType::Value,
        CallbackType::Null,
    );

    opt_identifier_def!(
        ArrOpt,
        id,
        para,
    );

    opt_name_def!(
        ArrOpt,
        prefix,
        name,
        prefix,
        name,
    );

    opt_optional_def!(
        ArrOpt,
        optional,
        optional,
    );

    opt_alias_def!(
        ArrOpt,
        alias,
        prefix,
        name,
    );

    opt_index_def!( ArrOpt );

    impl Value for ArrOpt {
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

        /// WARNING! 
        /// This function will append the `value` to option's value
        fn parse_value(&self, value: &str) -> Result<OptValue> {
            // Don't mssing any value
            let mut realv = self.value().clone();

            if realv.is_null() {
                realv = OptValue::Arr(vec![]);
            }
            realv.app_value(value.to_owned());
            
            Ok(realv)
        }

        fn has_value(&self) -> bool {
            self.value().is_arr()
        }

        fn reset_value(&mut self) {
            self.set_value(self.default_value().clone());
        }
    }

    #[derive(Debug)]
    pub struct ArrUtils;

    impl ArrUtils {
        pub fn new() -> Self {
            Self {}
        }
    }

    impl Utils for ArrUtils {
        fn type_name(&self) -> &str {
            current_type()
        }

        fn is_support_deactivate_style(&self) -> bool {
            true
        }

        fn create(&self, id: IIdentifier, ci: &CreateInfo) -> Result<Box<dyn Opt>> {
            let mut opt = Box::new(ArrOpt::new(
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

        fn create(&self, id: IIdentifier, ci: &CreateInfo) -> Result<Box<dyn Opt>> {
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

        fn create(&self, id: IIdentifier, ci: &CreateInfo) -> Result<Box<dyn Opt>> {
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

        fn create(&self, id: IIdentifier, ci: &CreateInfo) -> Result<Box<dyn Opt>> {
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
use std::fmt::Debug;

use crate::opt::*;
use crate::error::*;
use crate::set::Set;

/// Callback will be used by `option` type such as [`Bool`](opt::bool::Bool)
pub trait ValueCallback: Debug {
    fn call(&mut self, opt: &dyn Opt) -> Result<bool>;
}

/// Callback will be used by `non-option` type [`Pos`](crate::nonopt::pos::Pos)
pub trait IndexCallback: Debug {
    fn call(&mut self, set: &dyn Set, arg: &String) -> Result<bool>;
}

/// Callback will be used by `non-option` type [`Cmd`](crate::nonopt::cmd::Cmd) and [`Main`](crate::nonopt::main::Main)
pub trait MainCallback: Debug {
    fn call(&mut self, set: &dyn Set, args: &Vec<String>) -> Result<bool>;
}

#[derive(Debug)]
pub enum OptCallback {
    Value(Box<dyn ValueCallback>),
    Index(Box<dyn IndexCallback>),
    Main(Box<dyn MainCallback>),
    Null
}

impl OptCallback {

    pub fn from_value(cb: Box<dyn ValueCallback>) -> Self {
        Self::Value(cb)
    }

    pub fn from_index(cb: Box<dyn IndexCallback>) -> Self {
        Self::Index(cb)
    }

    pub fn from_main(cb: Box<dyn MainCallback>) -> Self {
        Self::Main(cb)
    }

    pub fn call_value(&mut self, opt: &dyn Opt) -> Result<bool> {
        match self {
            OptCallback::Value(cb) => {
                cb.as_mut().call(opt)
            }
            _ => {
                Ok(false)
            }
        }
    }

    pub fn call_index(&mut self, set: &dyn Set, arg: &String) -> Result<bool> {
        match self {
            OptCallback::Index(cb) => {
                cb.as_mut().call(set, arg)
            }
            _ => {
                Ok(false)
            }
        }
    }

    pub fn call_main(&mut self, set: &dyn Set, args: &Vec<String>) -> Result<bool> {
        match self {
            OptCallback::Main(cb) => {
                cb.as_mut().call(set, args)
            }
            _ => {
                Ok(false)
            }
        }
    }
}

/// [`CallbackType`] is using for [`Opt`] identify which type [`OptCallback`] need 
/// to be call when the [`Opt`] matched
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CallbackType {
    /// Identify the callback type [`OptCallback::Value`]
    Value,

    /// Identify the callback type [`OptCallback::Index`]
    Index,

    /// Identify the callback type [`OptCallback::Main`]
    Main,

    Null,
}

impl CallbackType {
    pub fn is_value(&self) -> bool {
        match self {
            Self::Value => true,
            _ => false,
        }
    }

    pub fn is_index(&self) -> bool {
        match self {
            Self::Index => true,
            _ => false,
        }
    }


    pub fn is_main(&self) -> bool {
        match self {
            Self::Main => true,
            _ => false,
        }
    }

    pub fn is_null(&self) -> bool {
        match self {
            Self::Null=> true,
            _ => false,
        }
    }
}

impl Default for CallbackType {
    fn default() -> Self {
        Self::Null
    }
}

/// Simple callback implementation for [`ValueCallback`]
pub struct SimpleValueCallback<T: FnMut(&dyn Opt) -> Result<bool>>(Box<T>);

impl<T: FnMut(&dyn Opt) -> Result<bool>> SimpleValueCallback<T> {
    pub fn new(cb: T) -> Self {
        Self(Box::new(cb))
    }
}

impl<T: FnMut(&dyn Opt) -> Result<bool>> Debug for SimpleValueCallback<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SimpleValueCallback")
         .field("FnMut", &String::from("..."))
         .finish()
    }
}

impl<T: FnMut(&dyn Opt) -> Result<bool>> ValueCallback for SimpleValueCallback<T> {
    fn call(&mut self, opt: &dyn Opt) -> Result<bool> {
        self.0.as_mut()(opt)
    }
}

/// Simple callback implementation for [`IndexCallback`]
pub struct SimpleIndexCallback<T: FnMut( &dyn Set, &String ) -> Result<bool>>(Box<T>);

impl<T: FnMut( &dyn Set, &String ) -> Result<bool>> SimpleIndexCallback<T> {
    pub fn new(cb: T) -> Self {
        Self(Box::new(cb))
    }
}

impl<T: FnMut( &dyn Set, &String ) -> Result<bool>> Debug for SimpleIndexCallback<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SimpleIndexCallback")
         .field("FnMut", &String::from("..."))
         .finish()
    }
}

impl<T: FnMut( &dyn Set, &String ) -> Result<bool>> IndexCallback for SimpleIndexCallback<T> {
    fn call(&mut self, set: &dyn Set, arg: &String) -> Result<bool> {
        self.0.as_mut()(set, arg)
    }
}

/// Simple callback implementation for [`MainCallback`]
pub struct SimpleMainCallback<T: FnMut( &dyn Set, &Vec<String> ) -> Result<bool> >(Box<T>);

impl<T: FnMut( &dyn Set, &Vec<String> ) -> Result<bool> > SimpleMainCallback<T> {
    pub fn new(cb: T) -> Self {
        Self(Box::new(cb))
    }
}

impl<T: FnMut( &dyn Set, &Vec<String> ) -> Result<bool> > Debug for SimpleMainCallback<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SimpleIndexCallback")
         .field("FnMut", &String::from("..."))
         .finish()
    }
}

impl<T: FnMut( &dyn Set, &Vec<String> ) -> Result<bool> > MainCallback for SimpleMainCallback<T> {
    fn call(&mut self, set: &dyn Set, args: &Vec<String>) -> Result<bool> {
        self.0.as_mut()(set, args)
    }
}
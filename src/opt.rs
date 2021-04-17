
use std::fmt::Debug;
use std::clone::Clone;

use crate::proc::Info;
use crate::proc::Proc;

#[derive(Debug, Clone)]
pub enum Style { 
    Setter(bool), // option -a

    Argument, // option has argument -a <param>

    Multiple, // multiple option -abc

    NonOption, // not an option
}

pub trait Type {
    fn type_name(&self) -> &str;

    fn match_style(&self, style: Style) -> bool;
}

pub trait Identifier {
    fn id(&self) -> u64;
}

pub trait Name {
    fn name(&self) -> &str;

    fn match_name(&self, s: &str) -> bool;
}

pub trait Prefix {
    fn prefix(&self) -> &str;

    fn match_prefix(&self, s: &str) -> bool;
}

pub trait Optional {
    fn optional(&self) -> bool;

    fn match_optional(&self, b: bool) -> bool;
}

pub trait Opt: Type + Identifier + Name + Prefix + Optional + Debug {}

#[derive(Debug)]
pub struct CommonInfo {
    id: u64,
}

impl CommonInfo {
    pub fn new(id: u64) -> Self {
        Self {
            id,
        }
    }
}

impl Info for CommonInfo {
    fn id(&self) -> u64 {
        self.id
    }
}

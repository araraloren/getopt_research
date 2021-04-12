
use std::fmt::Debug;

#[derive(Debug)]
pub enum Style { 
    Setter(bool), // option -a

    Argument, // option has argument -a <param>

    Multiple, // multiple option -abc

    NonOption, // not an option
}

pub trait Type {
    fn type_name(&self) -> &str;
}

pub trait Identifier {
    fn opt_id(&self) -> u64;
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

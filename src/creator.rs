
use std::fmt::Debug;
use crate::opt::Opt;

pub trait Creator: Debug {
    fn name(&self) -> &str;

    fn create(&self, s: &str) -> Box<dyn Opt>;
}

///
/// <name> = <type> 
/// [!]? <the option is optional or not>
/// [/]? <the option is deactivate style or not>
/// 
pub struct CreatorInfo {
    deactivate: bool,

    optional: bool,

    opt_type: String,

    opt_name: String,
}

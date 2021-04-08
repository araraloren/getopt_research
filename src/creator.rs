
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

impl CreatorInfo {
    pub fn new(s: &String) -> Result<Self, Error?> {

    }

    pub fn is_deactivate(&self) -> bool {
        self.deactivate
    }

    pub fn is_optional(&self) -> bool {
        self.optional
    }

    pub fn get_type(&self) -> &String {
        &self.opt_type
    }

    pub fn get_name(&self) -> &String {
        &self.opt_name
    }
}

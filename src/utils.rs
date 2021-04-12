
use crate::proc::Info;
use crate::proc::Proc;
use crate::opt::Opt;
use crate::err::Error;

use std::fmt::Debug;

pub trait Utils: Debug {
    fn type_name(&self) -> &str;

    fn create(&self, id: u64, ci: &CreatorInfo) -> Box<dyn Opt>;

    fn get_info(&self, opt: &dyn Opt) -> Box<dyn Info<Proc>>;
}

///
/// <name> = <type> 
/// [!]? <the option is optional or not>
/// [/]? <the option is deactivate style or not>
/// 
#[derive(Debug)]
pub struct CreatorInfo {
    deactivate: bool,

    optional: bool,

    opt_type: String,

    opt_name: String,
}

impl CreatorInfo {
    pub fn new(s: &str) -> Result<Self, Error> {
        const SPLIT: &str = "=";
        const DEACTIVATE: &str = "/";
        const NO_OPTIONAL: &str = "!";

        let splited: Vec<_> = s.split(SPLIT).collect();
        let mut type_last_index = 0;
        let mut deactivate = false;
        let mut optional = true;
        
        if splited.len() == 2 {
            if let Some(index) = splited[1].rfind(DEACTIVATE) {
                deactivate = true;
                if index != 0 {
                    type_last_index = index;
                }
            }
            if let Some(index) = splited[1].rfind(NO_OPTIONAL) {
                optional = false;
                if index != 0 && (
                    index < type_last_index || type_last_index == 0
                ) {
                    type_last_index = index;
                }
            }
            let (opt_type, _) = if type_last_index == 0 {
                (splited[1], splited[0]/* fine, not using*/)
            } else {
                splited[1].split_at(type_last_index)
            };

            return Ok(Self {
                deactivate,
                optional,
                opt_type: String::from(opt_type),
                opt_name: String::from(splited[0]),
            });
        }
        Err(Error::InvalidOptionStr(String::from(s)))
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

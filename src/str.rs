
use crate::opt::Opt;
use crate::opt::Type;
use crate::opt::Identifier;
use crate::opt::Name;
use crate::opt::Prefix;
use crate::opt::Optional;
use crate::proc::Info;
use crate::proc::Proc;
use crate::proc::Message;
use crate::utils::CreatorInfo;
use crate::utils::Utils;

const OPT_TYPE_STR: &'static str = "str";

pub trait Str: Opt { }

#[derive(Debug)]
pub struct StrOpt {
    opt_id: u64,

    name: String,

    prefix: String,

    optional: bool,
}

impl StrOpt {
    pub fn new(opt_id: u64, name: String, prefix: String, optional: bool) -> Self {
        Self {
            opt_id,
            name,
            prefix,
            optional,
        }
    }
}

impl Str for StrOpt { }

impl Opt for StrOpt { }

impl Type for StrOpt {
    fn type_name(&self) ->&str {
        OPT_TYPE_STR
    }
}

impl Identifier for StrOpt {
    fn opt_id(&self) -> u64 {
        self.opt_id
    }
}

impl Name for StrOpt {
    fn name(&self) -> &str {
        &self.name
    }

    fn match_name(&self, s: &str) -> bool {
        self.name() == s
    }
}

impl Prefix for StrOpt {
    fn prefix(&self) -> &str {
        &self.prefix
    }

    fn match_prefix(&self, s: &str) -> bool {
        self.prefix() == s
    }
}

impl Optional for StrOpt {
    fn optional(&self) -> bool {
        self.optional
    }

    fn match_optional(&self, b: bool) -> bool {
        self.optional() == b
    }
}

#[derive(Debug)]
pub struct StrCreator;

impl Utils for StrCreator {
    fn type_name(&self) -> &str {
        OPT_TYPE_STR
    }

    fn create(&self, id: u64, ci: &CreatorInfo) -> Box<dyn Opt> {
        Box::new(StrOpt::new(
            id,
            ci.get_name().clone(),
            String::from(""),
            ci.is_optional(),
        ))
    }

    fn get_info(&self, opt: &dyn Opt) -> Box<dyn Info<Proc>> {
        Box::new(StrInfo::new(opt.opt_id()))
    }
}

#[derive(Debug)]
pub struct StrInfo {
    id: u64,
}

impl StrInfo {
    pub fn new(id: u64) -> Self {
        Self {
            id,
        }
    }
}

impl Info<Proc> for StrInfo {
    fn info_id(&self) -> u64 {
        self.id
    }

    fn check(&self, msg: &Proc) -> bool {
        true
    }

    fn process(&mut self, data: &mut <Proc as Message>::Data, opt: &mut dyn Opt) {
        
    }
}

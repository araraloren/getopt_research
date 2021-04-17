
use crate::opt::Opt;
use crate::opt::Type;
use crate::opt::Style;
use crate::opt::Identifier;
use crate::opt::Name;
use crate::opt::Prefix;
use crate::opt::Optional;
use crate::opt::CommonInfo;
use crate::proc::Info;
use crate::utils::CreatorInfo;
use crate::utils::Utils;

const OPT_TYPE_STR: &'static str = "str";

pub trait Str: Opt { }

#[derive(Debug, Clone)]
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

    fn match_style(&self, style: Style) -> bool {
        match style {
            Style::Argument => {
                true
            }
            Style::Multiple => {
                self.name().len() == 1
            }
            _ => { false }
        }
    }
}

impl Identifier for StrOpt {
    fn id(&self) -> u64 {
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
pub struct StrUtils;

impl StrUtils {
    pub fn new() -> Self {
        Self { }
    }
}

impl Utils for StrUtils {
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

    fn get_info(&self, opt: &dyn Opt) -> Box<dyn Info> {
        Box::new(CommonInfo::new(opt.id()))
    }
}

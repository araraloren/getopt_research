
use std::any::Any;

use crate::opt::Opt;
use crate::opt::Type;
use crate::opt::Style;
use crate::opt::Identifier;
use crate::opt::Name;
use crate::opt::Value;
use crate::opt::Prefix;
use crate::opt::OptValue;
use crate::opt::Optional;
use crate::opt::CommonInfo;
use crate::proc::Info;
use crate::utils::CreatorInfo;
use crate::utils::Utils;

pub fn current_type() -> &'static str {
    "str"
}

pub trait Str: Opt { }

#[derive(Debug)]
pub struct StrOpt {
    opt_id: u64,

    name: String,

    prefix: String,

    optional: bool,

    value: OptValue,
}

impl StrOpt {
    pub fn new(opt_id: u64, name: String, prefix: String, optional: bool) -> Self {
        Self {
            opt_id,
            name,
            prefix,
            optional,
            value: OptValue::Null,
        }
    }
}

impl Str for StrOpt { }

impl Opt for StrOpt { }

impl Type for StrOpt {
    fn type_name(&self) ->&str {
        current_type()
    }

    fn match_style(&self, style: Style) -> bool {
        match style {
            Style::Argument => {
                true
            }
            _ => { false }
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
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

    fn set_name(&mut self, s: &str) {
        self.name = String::from(s)
    }

    fn match_name(&self, s: &str) -> bool {
        self.name() == s
    }
}

impl Prefix for StrOpt {
    fn prefix(&self) -> &str {
        &self.prefix
    }

    fn set_prefix(&mut self, s: &str) {
        self.prefix = String::from(s)
    }

    fn match_prefix(&self, s: &str) -> bool {
        self.prefix() == s
    }
}

impl Optional for StrOpt {
    fn optional(&self) -> bool {
        self.optional
    }

    fn set_optional(&mut self, b: bool) {
        self.optional = b;
    }

    fn match_optional(&self, b: bool) -> bool {
        self.optional() == b
    }
}

impl Value for StrOpt {
    fn value(&self) -> &OptValue {
        &self.value
    }

    fn set_value(&mut self, v: OptValue) {
        self.value = v;
    }

    fn parse_value(&self, v: &String) -> Option<OptValue> {
        Some(OptValue::Str(v.clone()))
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
        current_type()
    }

    fn create(&self, id: u64, ci: &CreatorInfo) -> Box<dyn Opt> {
        Box::new(StrOpt::new(
            id,
            ci.get_name().clone(),
            ci.get_prefix().clone(),
            ci.is_optional(),
        ))
    }

    fn get_info(&self, opt: &dyn Opt) -> Box<dyn Info> {
        Box::new(CommonInfo::new(opt.id()))
    }
}

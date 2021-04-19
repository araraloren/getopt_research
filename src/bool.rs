use std::any::Any;

use crate::opt::CommonInfo;
use crate::opt::Identifier;
use crate::opt::Name;
use crate::opt::Opt;
use crate::opt::OptValue;
use crate::opt::Optional;
use crate::opt::Prefix;
use crate::opt::Style;
use crate::opt::Type;
use crate::opt::Value;
use crate::proc::Info;
use crate::utils::CreatorInfo;
use crate::utils::Utils;

pub fn current_type() -> &'static str {
    "bool"
}

pub trait Bool: Opt {}

#[derive(Debug)]
pub struct BoolOpt {
    opt_id: u64,

    name: String,

    prefix: String,

    optional: bool,

    deactivate: bool,

    value: OptValue,
}

impl BoolOpt {
    pub fn new(
        opt_id: u64,
        name: String,
        prefix: String,
        optional: bool,
        deactivate: bool,
    ) -> Self {
        Self {
            opt_id,
            name,
            prefix,
            optional,
            deactivate,
            value: OptValue::Null,
        }
    }
}

impl Bool for BoolOpt {}

impl Opt for BoolOpt {}

impl Type for BoolOpt {
    fn type_name(&self) -> &str {
        current_type()
    }

    fn match_style(&self, style: Style) -> bool {
        match style {
            Style::Boolean | Style::Multiple => true,
            _ => false,
        }
    }

    fn is_deactivate(&self) -> bool {
        self.deactivate
    }

    fn is_need_argument(&self) -> bool {
        false
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Identifier for BoolOpt {
    fn id(&self) -> u64 {
        self.opt_id
    }
}

impl Name for BoolOpt {
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

impl Prefix for BoolOpt {
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

impl Optional for BoolOpt {
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

impl Value for BoolOpt {
    fn value(&self) -> &OptValue {
        &self.value
    }

    fn set_value(&mut self, v: OptValue) {
        self.value = v;
    }

    fn parse_value(&self, _v: Option<&String>) -> Option<OptValue> {
        if self.is_deactivate() {
            Some(OptValue::Bool(false))
        } else {
            Some(OptValue::Bool(true))
        }
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

    fn create(&self, id: u64, ci: &CreatorInfo) -> Box<dyn Opt> {
        Box::new(BoolOpt::new(
            id,
            ci.get_name().clone(),
            ci.get_prefix().clone(),
            ci.is_optional(),
            ci.is_deactivate(),
        ))
    }

    fn get_info(&self, opt: &dyn Opt) -> Box<dyn Info> {
        Box::new(CommonInfo::new(opt.id()))
    }
}

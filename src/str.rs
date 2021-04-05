
use crate::opt::Opt;
use crate::opt::Identifier;
use crate::opt::Name;
use crate::opt::Prefix;
use crate::opt::Optional;

use crate::creator::Creator;

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

#[derive(Debug)]
pub struct StrCreator;

impl StrCreator {
    pub fn new() -> Self {
        Self {}
    }
}

impl Str for StrOpt { }

impl Opt for StrOpt { }

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

impl Creator for StrCreator {
    fn name(&self) -> &str {
        "str"
    }

    fn create(s: &str) -> Box<dyn Opt> {
        Box::new(StrOpt::new(0, name: String, prefix: String, optional: bool))
    }
}

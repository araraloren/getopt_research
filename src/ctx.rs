use crate::opt::Opt;
use crate::opt::Style;
use crate::proc::Message;
use crate::proc::Proc;

use std::fmt::Debug;

pub trait Context: Debug {
    fn match_opt(&self, opt: &dyn Opt) -> bool;

    fn process(&self, opt: &mut dyn Opt);

    fn set_matched(&mut self);

    fn is_matched(&self) -> bool;
}

#[derive(Debug)]
pub struct OptContext {
    // prefix of option
    prefix: String,

    // name of option
    name: String,

    // a function that can get argument of option
    next_arg: Option<String>,

    // option style
    style: Style,

    // can we skip next argument when matched
    can_skip: bool,

    // are we matched
    matched: bool,
}

impl<'a, 'b, 'c> OptContext {
    pub fn new(
        prefix: String,
        name: String,
        next_arg: Option<String>,
        style: Style,
        can_skip: bool,
    ) -> Self {
        Self {
            prefix,
            name,
            next_arg,
            style,
            can_skip,
            matched: false,
        }
    }
}

impl Context for OptContext {
    fn match_opt(&self, opt: &dyn Opt) -> bool {
        debug!("MATCHING {:?} <-> {:?}", self, opt);

        let mut ret = opt.match_style(self.style.clone());

        if ret {
            ret = ret && opt.match_name(self.name.as_str());
        }
        if ret {
            ret = ret && opt.match_prefix(self.prefix.as_str());
        }

        debug!("==> {}", ret);
        ret
    }

    fn process(&self, opt: &mut dyn Opt) {
        if let Some(v) = opt.parse_value(self.next_arg.as_ref()) {
            opt.set_value(v);
        }
    }

    fn set_matched(&mut self) {
        self.matched = true;
    }

    fn is_matched(&self) -> bool {
        self.matched
    }
}

use crate::opt::Opt;
use crate::opt::Style;
use crate::proc::Proc;
use crate::proc::Message;

use std::fmt::Debug;

pub trait Context: Debug {
    fn match_opt(&self, opt: &dyn Opt) -> bool;

    fn process(&self, opt: &mut dyn Opt);

    fn set_matched(&mut self);

    fn matched(&self) -> bool;
}

pub struct ArgGetter(pub Option<Box<dyn Fn() -> String>>);

impl Debug for ArgGetter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ArgGetter").field("Fn", &"()").finish()
    }
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
        match &self.next_arg {
            Some(next_arg) => {
                match opt.parse_value(next_arg) {
                    Some(value) => {
                        opt.set_value(value);
                    }
                    None => {
                        // !!!
                    }
                }
            }
            None => {
                // !!!
            }
        }
    }

    fn set_matched(&mut self) {
        self.matched = true;
    }

    fn matched(&self) -> bool {
        self.matched
    }
}

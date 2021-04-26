
use std::fmt::Debug;

use crate::opt::Opt;
use crate::opt::Style;

pub trait Context: Debug {
    /// Check if the opt is matched with current context
    fn match_opt(&self, opt: &dyn Opt) -> bool;

    /// Process the option if matched successful
    fn process(&mut self, opt: &mut dyn Opt);

    /// Return true if the context already matched successful
    fn is_matched(&self) -> bool;

    /// Return true if the matched option need an argument
    fn is_need_argument(&self) -> bool;
}

/// Contex implementation for option. 
/// It will check the option name, prefix, style and alias.
/// It will set option value if matched.
#[derive(Debug)]
pub struct OptContext<'a, 'b, 'c> {
    opt_prefix: &'a str,

    opt_name: &'b str,

    next_argument: Option<&'c str>,

    style: Style,

    skip_next_arg: bool,

    matched: bool,
}

impl <'a, 'b, 'c> OptContext<'a, 'b, 'c> {
    pub fn new(
        prefix: &'a str,
        name: &'b str,
        arg: Option<&'c str>,
        style: Style,
        skip_next_arg: bool
    ) -> Self {
        Self {
            opt_prefix: prefix,
            opt_name: name,
            next_argument: arg,
            style,
            skip_next_arg,
            matched: false,
        }
    }

    pub fn set_prefix(&mut self, prefix: &'a str) -> &mut Self {
        self.opt_prefix = prefix;
        self
    }

    pub fn set_name(&mut self, name: &'b str) -> &mut Self {
        self.opt_name = name;
        self
    }

    pub fn set_next_argument(&mut self, arg: Option<&'c str>) -> &mut Self {
        self.next_argument = arg;
        self
    }

    pub fn set_style(&mut self, style: Style) -> &mut Self {
        self.style = style;
        self
    }

    pub fn set_skip_next_arg(&mut self, b: bool) -> &mut Self {
        self.skip_next_arg = b;
        self
    }
}

impl<'a, 'b, 'c> Context for OptContext<'a, 'b, 'c> {
    fn match_opt(&self, opt: &dyn Opt) -> bool {
        let matched = 
            opt.match_style(self.style.clone()) &&
            opt.match_prefix(self.opt_prefix) &&
            (opt.match_name(self.opt_name) || opt.match_alias(self.opt_prefix, self.opt_name));
        matched
    }

    fn process(&mut self, opt: &mut dyn Opt) {
        self.matched = true;
        if let Some(v) = opt.parse_value(self.next_argument) {
            opt.set_value(v);
        }
    }

    fn is_matched(&self) -> bool {
        self.matched
    }

    fn is_need_argument(&self) -> bool {
        self.skip_next_arg
    }
}
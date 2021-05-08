
use std::fmt::Debug;

use crate::opt::Opt;
use crate::opt::Style;
use crate::error::Result;
use crate::error::Error;

pub trait Context: Debug {
    /// Check if the opt is matched with current context
    fn match_opt(&self, opt: &dyn Opt) -> bool;

    /// Process the option if matched successful
    fn process(&mut self, opt: &mut dyn Opt) -> Result<bool>;

    /// Return true if the context already matched successful
    fn is_matched(&self) -> bool;

    /// Return true if the matched option need an argument
    fn is_need_argument(&self) -> bool;
}

/// Contex implementation for option. 
/// It will check the option name, prefix, style and alias.
/// It will set option value if matched.
#[derive(Debug)]
pub struct OptContext {
    opt_prefix: String,

    opt_name: String,

    next_argument: Option<String>,

    style: Style,

    skip_next_arg: bool,

    matched: bool,
}

impl OptContext {
    pub fn new(
        prefix: String,
        name: String,
        arg: Option<String>,
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

    pub fn set_prefix(&mut self, prefix: String) -> &mut Self {
        self.opt_prefix = prefix;
        self
    }

    pub fn set_name(&mut self, name: String) -> &mut Self {
        self.opt_name = name;
        self
    }

    pub fn set_next_argument(&mut self, arg: Option<String>) -> &mut Self {
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

impl Context for OptContext {
    fn match_opt(&self, opt: &dyn Opt) -> bool {
        let matched = 
            opt.match_style(self.style.clone()) &&
            ((opt.match_name(self.opt_name.as_str()) && opt.match_prefix(self.opt_prefix.as_str()))
                || opt.match_alias(&self.opt_prefix, &self.opt_name));
        debug!("Match Opt<{:?}> {:?}: {}", opt.id(), opt, matched);
        matched
    }

    fn process(&mut self, opt: &mut dyn Opt) -> Result<bool> {
        self.matched = true;
        debug!("Match successed => {:?} : Opt<{:?}>", self, opt.id());
        if opt.is_need_argument() && self.next_argument.is_none() {
            return Err(Error::ArgumentRequired(format!("{}{}", opt.prefix(), opt.name())));
        }
        let mut value = &String::default();

        if let Some(v) = &self.next_argument {
            value = v;
        }

        opt.set_value(opt.parse_value(value.as_str())?);
        Ok(true)
    }

    fn is_matched(&self) -> bool {
        self.matched
    }

    fn is_need_argument(&self) -> bool {
        self.skip_next_arg
    }
}

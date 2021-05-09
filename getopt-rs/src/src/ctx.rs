
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
            opt.is_style(self.style.clone()) &&
            ((opt.match_name(self.opt_name.as_str()) && opt.match_prefix(self.opt_prefix.as_str()))
                || opt.match_alias(&self.opt_prefix, &self.opt_name));
        debug!("Match Opt<{:?}> {:?} => ", opt.id(), opt);
        debug!(">>** {}", if matched { "TRUE" } else { "FALSE" });
        matched
    }

    fn process(&mut self, opt: &mut dyn Opt) -> Result<bool> {
        let mut value = &String::default();
        
        debug!("Match successed => {:?} : Opt<{:?}>", self, opt.id());
        self.matched = true;
        if opt.is_style(Style::Argument) && self.next_argument.is_none() {
            return Err(Error::ArgumentRequired(format!("{}{}", opt.prefix(), opt.name())));
        }
        if let Some(v) = &self.next_argument {
            value = v;
        }
        // we always call `set_value` since all option type need update value through this function
        opt.set_value(opt.parse_value(value.as_str())?);
        opt.set_need_invoke(true);
        Ok(true)
    }

    fn is_matched(&self) -> bool {
        self.matched
    }

    fn is_need_argument(&self) -> bool {
        self.skip_next_arg
    }
}

#[derive(Debug)]
pub struct NonOptContext {
    opt_name: String,

    style: Style,

    total: i64,

    current: i64,

    matched: bool,
}

impl NonOptContext {
    pub fn new(opt_name: String, style: Style, total: i64, current: i64) -> Self {
        Self {
            opt_name,
            style,
            total,
            current,
            matched: false,
        }
    }

    pub fn set_name(&mut self, name: String) -> &mut Self {
        self.opt_name = name;
        self
    }

    pub fn set_style(&mut self, style: Style) -> &mut Self {
        self.style = style;
        self
    }

    pub fn set_total(&mut self, total: i64) -> &mut Self {
        self.total = total;
        self
    }

    pub fn set_current(&mut self, current: i64) -> &mut Self {
        self.current = current;
        self
    }
}

impl Context for NonOptContext {
    fn match_opt(&self, opt: &dyn Opt) -> bool {
        let matched = 
            opt.is_style(self.style.clone()) && 
            opt.match_index(self.total, self.current) &&
            opt.match_name(&self.opt_name);

        debug!("Match NonOpt<{:?}> {:?} => ", opt.id(), opt);
        debug!(">>^^ {}", if matched { "TRUE" } else { "FALSE" });
        matched
    }

    fn process(&mut self, opt: &mut dyn Opt) -> Result<bool> { 
        debug!("Match successed => {:?} : Opt<{:?}>", self, opt.id());
        self.matched = true;
        // opt_name will be ignored, 
        // try to set value even if the value will be set in another side
        opt.set_value(opt.parse_value(&self.opt_name)?);
        opt.set_need_invoke(true);
        Ok(true)
    }

    fn is_matched(&self) -> bool {
        self.matched
    }

    fn is_need_argument(&self) -> bool {
        false
    }
}
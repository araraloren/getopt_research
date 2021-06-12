
use std::fmt::Debug;
use std::borrow::Cow;

use crate::opt::{Opt, Style};
use crate::error::{Result, Error};
use crate::id::Identifier;

pub trait Context<'a>: Debug {
    /// Get context identifier inside [`Proc`](crate::proc::Proc)
    fn id(&self) -> &Identifier;

    /// Check if the opt is matched with current context
    fn match_opt(&self, opt: &dyn Opt) -> bool;

    /// Process the option if matched successful
    fn process(&mut self, opt: &mut dyn Opt) -> Result<bool>;

    /// Return matched index if the context already matched successful
    fn get_matched_index(&self) -> Option<u64>;

    /// Return true if the matched option need an argument
    fn is_need_argument(&self) -> bool;

    /// Return true if context matched any option
    fn is_matched(&self) -> bool;

    /// Return the style of current context
    fn get_style(&self) -> Style;

    /// Return next argument
    fn get_next_argument(&self) -> Option<&Cow<'a, str>>;
}

/// Context implementation for option. 
/// 
/// It will check the option name, prefix, style and alias.
/// It will set option value if matched.
#[derive(Debug)]
pub struct OptContext<'a> {
    id: Identifier,

    opt_prefix: Cow<'a, str>,

    opt_name: Cow<'a, str>,

    next_argument: Option<Cow<'a, str>>,

    style: Style,

    skip_next_arg: bool,

    matched_index: Option<u64>,
}

impl<'a> OptContext<'a> {
    pub fn new(
        prefix: Cow<'a, str>,
        name: Cow<'a, str>,
        arg: Option<Cow<'a, str>>,
        style: Style,
        skip_next_arg: bool
    ) -> Self {
        Self {
            id: Identifier::new(0),
            opt_prefix: prefix,
            opt_name: name,
            next_argument: arg,
            style,
            skip_next_arg,
            matched_index: None,
        }
    }

    pub fn set_prefix(&mut self, prefix: Cow<'a, str>) -> &mut Self {
        self.opt_prefix = prefix;
        self
    }

    pub fn set_name(&mut self, name: Cow<'a, str>) -> &mut Self {
        self.opt_name = name;
        self
    }

    pub fn set_next_argument(&mut self, arg: Option<Cow<'a, str>>) -> &mut Self {
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

impl<'a> Context<'a> for OptContext<'a> {
    fn id(&self) -> &Identifier {
        &self.id
    }

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
        
        self.matched_index = Some(0);
        debug!("Match successed => {:?} : Opt<{:?}>", self, opt.id());
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

    fn get_matched_index(&self) -> Option<u64> {
        self.matched_index
    }

    fn is_need_argument(&self) -> bool {
        self.skip_next_arg
    }

    fn is_matched(&self) -> bool {
        self.matched_index.is_some()
    }

    fn get_style(&self) -> Style {
        self.style.clone()
    }

    fn get_next_argument(&self) -> Option<&Cow<'a, str>> {
        self.next_argument.as_ref()
    }
}

/// Context implementation for non-option. 
/// 
/// It will check the non-option name, index, style.
/// It will set option value if matched, and will make the callback invokeable.
#[derive(Debug)]
pub struct NonOptContext<'a> {
    id: Identifier,

    opt_name: Cow<'a, str>,

    style: Style,

    total: u64,

    current: u64,

    matched_index: Option<u64>,
}

impl<'a> NonOptContext<'a> {
    pub fn new(opt_name: Cow<'a, str>, style: Style, total: u64, current: u64) -> Self {
        Self {
            id: Identifier::new(0),
            opt_name,
            style,
            total,
            current,
            matched_index: None,
        }
    }

    pub fn id(&self) -> &Identifier {
        &self.id
    }

    pub fn set_name(&mut self, name: String) -> &mut Self {
        self.opt_name = name;
        self
    }

    pub fn set_style(&mut self, style: Style) -> &mut Self {
        self.style = style;
        self
    }

    pub fn set_total(&mut self, total: u64) -> &mut Self {
        self.total = total;
        self
    }

    pub fn set_current(&mut self, current: u64) -> &mut Self {
        self.current = current;
        self
    }
}

impl<'a> Context<'a> for NonOptContext<'a> {
    fn id(&self) -> &Identifier {
        &self.id
    }

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
        self.matched_index = Some(self.current);
        debug!("Match successed => {:?} : Opt<{:?}>", self, opt.id());
        // opt_name will be ignored, 
        // try to set value even if the value will be set in another side
        opt.set_value(opt.parse_value(&self.opt_name)?);
        opt.set_need_invoke(true);
        Ok(true)
    }

    fn get_matched_index(&self) -> Option<u64> {
        self.matched_index
    }

    fn is_need_argument(&self) -> bool {
        false
    }

    fn is_matched(&self) -> bool {
        self.matched_index.is_some()
    }

    fn get_style(&self) -> Style {
        self.style.clone()
    }

    fn get_next_argument(&self) -> Option<&Cow<'a, str>> {
        None
    }
}

/// Context implementation for delay option. 
/// 
/// It will check the option name, prefix, style and alias.
/// It will not set option value if matched.
#[derive(Debug)]
pub struct DelayContext<'a> {
    id: Identifier,

    opt_prefix: Cow<'a, str>,

    opt_name: Cow<'a, str>,

    next_argument: Option<Cow<'a, str>>,

    style: Style,

    skip_next_arg: bool,

    matched_index: Option<u64>,
}

impl<'a> DelayContext<'a> {
    pub fn new(
        prefix: Cow<'a, str>,
        name: Cow<'a, str>,
        arg: Option<Cow<'a, str>>,
        style: Style,
        skip_next_arg: bool
    ) -> Self {
        Self {
            id: Identifier::new(0),
            opt_prefix: prefix,
            opt_name: name,
            next_argument: arg,
            style,
            skip_next_arg,
            matched_index: None,
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

    pub fn set_next_argument(&mut self, arg: Option<Cow<'a, str>>) -> &mut Self {
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

impl<'a> Context<'a> for DelayContext<'a> {
    fn id(&self) -> &Identifier {
        &self.id
    }

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
        self.matched_index = Some(0); 
        debug!("Match successed => {:?} : Opt<{:?}>", self, opt.id());
        if opt.is_style(Style::Argument) && self.next_argument.is_none() {
            return Err(Error::ArgumentRequired(format!("{}{}", opt.prefix(), opt.name())));
        }
        // we always call `set_value` since all option type need update value through this function
        // opt.set_value(opt.parse_value(value.as_str())?);
        opt.set_need_invoke(true);
        Ok(true)
    }

    fn get_matched_index(&self) -> Option<u64> {
        self.matched_index
    }

    fn is_need_argument(&self) -> bool {
        self.skip_next_arg
    }

    fn is_matched(&self) -> bool {
        self.matched_index.is_some()
    }

    fn get_style(&self) -> Style {
        self.style.clone()
    }

    fn get_next_argument(&self) -> Option<&Cow<'a, str>> {
        self.next_argument.as_ref()
    }
}
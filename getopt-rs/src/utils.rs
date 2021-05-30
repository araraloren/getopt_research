
use std::fmt::Debug;

use crate::callback::CallbackType;
use crate::opt::{Opt, OptValue, NonOptIndex};
use crate::error::{Error, Result};
use crate::proc::Info;
use crate::id::Identifier;

pub trait Utils: Debug {
    fn type_name(&self) -> &str;

    fn is_support_deactivate_style(&self) -> bool;

    fn create(&self, id: Identifier, ci: &CreateInfo) -> Result<Box<dyn Opt>>;

    fn gen_info(&self, opt: &dyn Opt) -> Box<dyn Info>;
}

#[derive(Debug, Default)]
pub struct CreateInfo {
    deactivate: bool,

    optional: bool,

    type_name: String,

    opt_name: String,

    opt_prefix: String,

    opt_index: NonOptIndex,

    opt_alias: Vec<(String, String)>,

    opt_value: OptValue,

    opt_callback_type: CallbackType,
}

impl CreateInfo {
    pub fn new(
        type_name: &str,
        name: &str,
        prefix: &str,
        index: NonOptIndex,
        deactivate_style: bool,
        optional: bool,
        deafult_value: OptValue,
        opt_callback_type: CallbackType,
    ) -> Self {
        Self {
            type_name: type_name.to_owned(),
            opt_name: name.to_owned(),
            opt_prefix: prefix.to_owned(),
            opt_index: index,
            deactivate: deactivate_style,
            optional,
            opt_alias: vec![],
            opt_value: deafult_value,
            opt_callback_type,
        }
    }

    pub fn parse(s: &str, prefixs: &Vec<String>) -> Result<Self> {
        let pr = parse_opt_string(s, prefixs)?;
        let type_name = pr.type_name.ok_or(Error::NullOptionType)?;
        let opt_name = pr.opt_name.ok_or(Error::NullOptionName)?;
        Ok(Self {
            type_name,
            opt_name,
            opt_prefix: pr.opt_prefix.unwrap_or(String::default()),
            opt_index: pr.opt_index,
            deactivate: pr.deactivate.unwrap_or(false),
            optional: pr.optional.unwrap_or(true),
            opt_alias: vec![],
            opt_value: OptValue::default(),
            opt_callback_type: CallbackType::default(),
        })
    }

    /// Check if the create information is correct
    pub fn check(&self) -> Result<bool> {
        if self.get_type_name() != "" {
            Err(Error::NullOptionType)
        }
        else if self.get_name() != "" {
            Err(Error::NullOptionName)
        }
        else {
            Ok(true)
        }
    }

    /// Return true if the option support `-/a` style disable the option
    pub fn is_deactivate_style(&self) -> bool {
        self.deactivate
    }

    /// Return true if the option is force required
    pub fn is_optional(&self) -> bool {
        self.optional
    }

    /// Return the option type name
    pub fn get_type_name(&self) -> &str {
        &self.type_name
    }

    /// Return the option name
    pub fn get_name(&self) -> &str {
        &self.opt_name
    }

    /// Return the option prefix
    pub fn get_prefix(&self) -> &str {
        &self.opt_prefix
    }

    /// Return the option index
    pub fn get_index(&self) -> &NonOptIndex {
        &self.opt_index
    }

    /// Return the option alias
    pub fn get_alias(&self) -> &Vec<(String, String)> {
        &self.opt_alias
    }

    pub fn get_default_value(&self) -> &OptValue {
        &self.opt_value
    }

    pub fn get_callback_type(&self) -> &CallbackType {
        &self.opt_callback_type
    }

    pub fn set_deactivate_style(&mut self, deactivate: bool) {
        self.deactivate = deactivate;
    }

    pub fn set_optional(&mut self, optional: bool) {
        self.optional = optional;
    }

    pub fn set_type_name(&mut self, opt_type: &str) {
        self.type_name = opt_type.to_owned();
    }

    pub fn set_name(&mut self, opt_name: &str) {
        self.opt_name = opt_name.to_owned();
    }

    pub fn set_prefix(&mut self, prefix: &str) {
        self.opt_prefix = prefix.to_owned();
    }

    pub fn set_index(&mut self, index: NonOptIndex) {
        self.opt_index = index;
    }

    pub fn set_deafult_value(&mut self, value: OptValue) {
        self.opt_value = value;
    }

    pub fn set_callback_type(&mut self, callback_type: CallbackType) {
        self.opt_callback_type = callback_type;
    }

    pub fn add_alias(&mut self, prefix: &str, name: &str) {
        self.opt_alias.push((prefix.to_owned(), name.to_owned()));
    }

    pub fn rem_alias(&mut self, prefix: &str, name: &str) {
        for index in 0 .. self.opt_alias.len() {
            let alias = &self.opt_alias[index];

            if alias.0 == prefix && alias.1 == name {
                self.opt_alias.remove(index);
                break;
            }
        }
    }

    pub fn clr_alias(&mut self) {
        self.opt_alias.clear();
    }
}

#[derive(Debug, Default)]
pub struct FilterInfo {
    deactivate: Option<bool>,

    optional: Option<bool>,

    type_name: Option<String>,

    opt_name: Option<String>,

    opt_prefix: Option<String>,

    opt_index: NonOptIndex,
}

impl FilterInfo {
    pub fn new() -> Self {
        Self {
            deactivate: None,
            optional: None,
            type_name: None,
            opt_name: None,
            opt_prefix: None,
            opt_index: NonOptIndex::Null,
        }
    }

    pub fn parse(opt: &str, prefixs: &Vec<String>) -> Result<Self> {
        let pr = parse_opt_string(opt, prefixs)?;
        Ok(Self {
            deactivate: pr.deactivate,
            optional: pr.optional,
            type_name: pr.type_name,
            opt_name: pr.opt_name,
            opt_prefix: pr.opt_prefix,
            opt_index: pr.opt_index,
        })
    }

        /// Return true if filter has deactivate style
    pub fn has_deactivate_style(&self) -> bool {
        self.deactivate.is_some()
    }

    /// Return true if filter has force required
    pub fn has_optional(&self) -> bool {
        self.optional.is_some()
    }

    /// Return true if filter has option type name
    pub fn has_type_name(&self) -> bool {
        self.type_name.is_some()
    }

    /// Return true if filter has option name
    pub fn has_name(&self) -> bool {
        self.opt_name.is_some()
    }

    /// Return true if filter has prefix
    pub fn has_prefix(&self) ->  bool {
        self.opt_prefix.is_some()
    }

    /// Return true if filter has index
    pub fn has_index(&self) -> bool {
        !self.opt_index.is_null()
    }
    
    /// Return true if the option support `-/a` style disable the option
    pub fn is_deactivate_style(&self) -> bool {
        self.deactivate.unwrap()
    }

    /// Return true if the option is force required
    pub fn is_optional(&self) -> bool {
        self.optional.unwrap()
    }

    /// Return the option type name
    pub fn get_type_name(&self) -> &str {
        self.type_name.as_ref().unwrap().as_str()
    }

    /// Return the option name
    pub fn get_name(&self) -> &str {
        self.opt_name.as_ref().unwrap().as_str()
    }

    /// Return the option prefix
    pub fn get_prefix(&self) -> &str {
        self.opt_prefix.as_ref().unwrap().as_str()
    }

    /// Return the option index
    pub fn get_index(&self) -> &NonOptIndex {
        &self.opt_index
    }

    pub fn set_deactivate_style(&mut self, deactivate: bool) {
        self.deactivate = Some(deactivate);
    }

    pub fn set_optional(&mut self, optional: bool) {
        self.optional = Some(optional);
    }

    pub fn set_type_name(&mut self, opt_type: &str) {
        self.type_name = Some(opt_type.to_owned());
    }

    pub fn set_name(&mut self, opt_name: &str) {
        self.opt_name = Some(opt_name.to_owned());
    }

    pub fn set_prefix(&mut self, prefix: &str) {
        self.opt_prefix = Some(prefix.to_owned());
    }

    pub fn set_index(&mut self, index: NonOptIndex) {
        self.opt_index = index;
    }

    pub fn match_opt(&self, opt: &dyn Opt) -> bool {
        let mut ret = true;

        if ret && self.has_type_name() {
            ret = ret && (self.get_type_name() == opt.type_name());
        }
        if ret {
            if self.has_prefix() {
                let mut prefix_matched;

                prefix_matched = self.get_prefix() == opt.prefix();
                if ! prefix_matched {
                    if let Some(alias_v) = opt.alias() {
                        let mut alias_matched = false;

                        for alias in alias_v.iter() {
                            alias_matched = alias_matched || (alias.0 == self.get_prefix());
                            if alias_matched {
                                break;
                            }
                        }
                        prefix_matched = prefix_matched || alias_matched;
                    }
                }
                ret = ret && prefix_matched;
            }
            if self.has_name() {
                let mut name_matched;

                name_matched = self.get_name() == opt.name();
                if !name_matched {
                    if let Some(alias_v) = opt.alias() {
                        let mut alias_matched = false;

                        for alias in alias_v.iter() {
                            if self.has_name() {
                                alias_matched = alias_matched || (alias.1 == self.get_name());
                            }
                            if alias_matched {
                                break;
                            }
                        }
                        name_matched = name_matched || alias_matched;
                    }
                }
                ret = ret && name_matched;
            }
        }
        if ret && !self.get_index().is_null() {
            ret = ret && (self.get_index() == opt.index());
        }
        ret
    }
}

#[derive(Debug)]
struct ParseResult {
    type_name: Option<String>,

    opt_name: Option<String>,

    opt_prefix: Option<String>,

    deactivate: Option<bool>,

    optional: Option<bool>,

    opt_index: NonOptIndex,
}

/// Parse input string `<prefix>|<name>=<type>[!][/]@<index>`,
/// such as `-|o=a!`, means the force required option `o`, with a prefix "-", 
/// and option type is `a`.
/// `!` means the option is optional or not.
/// `/` means the option is deactivate style or not.
fn parse_opt_string(s: &str, prefixs: &Vec<String>) -> Result<ParseResult> {
    const SPLIT: &str = "=";
    const DEACTIVATE: &str = "/";
    const NO_OPTIONAL: &str = "!";
    const INDEX: &str = "@";

    if s.is_empty(){
        return Err(Error::InvalidOptionStr(s.to_owned()));
    }

    let mut splited_index = 0;
    let mut deactivate = None;
    let mut optional = None;
    let mut opt_index = NonOptIndex::Null;
    let opt_name;
    let mut type_name = None;
    let right_info;
    let left_info;
    let mut opt_prefix  = None;
    let mut prefix_index = 0;
    let without_prefix;

    for prefix in prefixs.iter() {
        if s.starts_with(prefix) {
            opt_prefix = Some(prefix.clone());
            prefix_index = prefix.len();
            break;
        }
    }

    if prefix_index == s.len() {
        return Err(Error::InvalidOptionStr(s.to_owned()));
    }

    // do we have a prefix matched
    if opt_prefix.is_none() {
        without_prefix = s;
    }
    else {
        without_prefix = s.split_at(prefix_index).1;
    }

    let splited: Vec<_> = without_prefix.split(SPLIT).collect();

    // for example, s is `-o=a!/@1`, prefix is `-`
    if splited.len() == 2 {
        // `o`
        left_info = splited[0];
        // `a!/@1`
        right_info = splited[1];
    }
    else {
        // without type, right_info is `-|o!/@1`
        right_info = without_prefix;
        left_info = without_prefix;
    }

    // if we have a `/`
    if let Some(index) = right_info.rfind(DEACTIVATE) {
        deactivate = Some(true);
        if index != 0 {
            splited_index = index;
        }
    }
    // if we have a `!`
    if let Some(index) = right_info.rfind(NO_OPTIONAL) {
        optional = Some(false);
        if index != 0 && (index < splited_index || splited_index == 0) {
            splited_index = index;
        }
    }
    // if we have a `@`
    if let Some(index) = right_info.rfind(INDEX) {
        match right_info.split_at(index + 1).1.parse::<i64>() {
            Ok(v) => {
                if v > 0 {
                    opt_index = NonOptIndex::forward(v as u64);
                }
                else if v < 0 {
                    opt_index = NonOptIndex::backward((-v) as  u64);
                }
                else {
                    opt_index = NonOptIndex::anywhere();
                }
            }
            Err(_) => {
                return Err(Error::InvalidOptionStr(s.to_owned()))
            }
        }
        if index != 0 && (index < splited_index || splited_index == 0) {
            splited_index = index;
        }
    }

    if splited.len() == 2 {
        let inner_opt_type;

        if splited_index == 0 {
            // we not have `/`, `!` or `@`, so right_info is option type
            inner_opt_type = right_info;
        } else {
            // left part is option type
            inner_opt_type = right_info.split_at(splited_index).0;
        };
        
        type_name = Some(inner_opt_type.to_owned());
        opt_name = Some(left_info.to_owned());
    }
    else {
        if splited_index == 0 {
            // we not have `/`, `!` or `@`, so right_info is option name part
            opt_name = Some(right_info.to_owned());
        }
        else {
            // left part is option name part
            opt_name  = Some(right_info.split_at(splited_index).0.to_owned());         
        }
    }

    debug!("Parsing ==> {:?} -> {:?} {:?} {:?}", s, opt_prefix, opt_name, type_name);

    return Ok(ParseResult {
        type_name,
        opt_name,
        opt_prefix,
        deactivate,
        optional,
        opt_index,
    })
}

#[cfg(test)]
mod tests {
    use crate::utils::parse_opt_string;
    use crate::opt::NonOptIndex;

    #[test]
    fn str_can_parse_to_create_info() {
        let test_cases = vec![
            ("o=b", Some(("b", "o", "", NonOptIndex::Null, false, true))),
            ("o=b!", Some(("b", "o", "", NonOptIndex::Null, false, false))),
            ("o=b/", Some(("b", "o", "", NonOptIndex::Null, true, true))),
            ("o=b!/", Some(("b", "o", "", NonOptIndex::Null, true, false))),
            ("o=b/!", Some(("b", "o", "", NonOptIndex::Null, true, false))),
            ("option=b", Some(("b", "option", "", NonOptIndex::Null, false, true))),
            ("option=b!", Some(("b", "option", "", NonOptIndex::Null, false, false))),
            ("option=b/", Some(("b", "option", "", NonOptIndex::Null, true, true))),
            ("option=b!/", Some(("b", "option", "", NonOptIndex::Null, true, false))),
            ("option=b/!", Some(("b", "option", "", NonOptIndex::Null, true, false))),

            ("-o=b", Some(("b", "o", "-", NonOptIndex::Null, false, true))),
            ("-o=b!", Some(("b", "o", "-", NonOptIndex::Null, false, false))),
            ("-o=b/", Some(("b", "o", "-", NonOptIndex::Null, true, true))),
            ("-o=b!/", Some(("b", "o", "-", NonOptIndex::Null, true, false))),
            ("-o=b/!", Some(("b", "o", "-", NonOptIndex::Null, true, false))),
            ("-option=b", Some(("b", "option", "-", NonOptIndex::Null, false, true))),
            ("-option=b!", Some(("b", "option", "-", NonOptIndex::Null, false, false))),
            ("-option=b/", Some(("b", "option", "-", NonOptIndex::Null, true, true))),
            ("-option=b!/", Some(("b", "option", "-", NonOptIndex::Null, true, false))),
            ("-option=b/!", Some(("b", "option", "-", NonOptIndex::Null, true, false))),

            ("--o=b", Some(("b", "o", "--", NonOptIndex::Null, false, true))),
            ("--o=b!", Some(("b", "o", "--", NonOptIndex::Null, false, false))),
            ("--o=b/", Some(("b", "o", "--", NonOptIndex::Null, true, true))),
            ("--o=b!/", Some(("b", "o", "--", NonOptIndex::Null, true, false))),
            ("--o=b/!", Some(("b", "o", "--", NonOptIndex::Null, true, false))),
            ("--option=b", Some(("b", "option", "--", NonOptIndex::Null, false, true))),
            ("--option=b!", Some(("b", "option", "--", NonOptIndex::Null, false, false))),
            ("--option=b/", Some(("b", "option", "--", NonOptIndex::Null, true, true))),
            ("--option=b!/", Some(("b", "option", "--", NonOptIndex::Null, true, false))),
            ("--option=b/!", Some(("b", "option", "--", NonOptIndex::Null, true, false))),

            ("/o=b", Some(("b", "o", "/", NonOptIndex::Null, false, true))),
            ("/o=b!", Some(("b", "o", "/", NonOptIndex::Null, false, false))),
            ("/o=b/", Some(("b", "o", "/", NonOptIndex::Null, true, true))),
            ("/o=b!/", Some(("b", "o", "/", NonOptIndex::Null, true, false))),
            ("/o=b/!", Some(("b", "o", "/", NonOptIndex::Null, true, false))),
            ("/option=b", Some(("b", "option", "/", NonOptIndex::Null, false, true))),
            ("/option=b!", Some(("b", "option", "/", NonOptIndex::Null, false, false))),
            ("/option=b/", Some(("b", "option", "/", NonOptIndex::Null, true, true))),
            ("/option=b!/", Some(("b", "option", "/", NonOptIndex::Null, true, false))),
            ("/option=b/!", Some(("b", "option", "/", NonOptIndex::Null, true, false))),

            ("o", Some(("", "o", "", NonOptIndex::Null, false, true))),
            ("o!", Some(("", "o", "", NonOptIndex::Null, false, false))),
            ("o/", Some(("", "o", "", NonOptIndex::Null, true, true))),
            ("o!/", Some(("", "o", "", NonOptIndex::Null, true, false))),
            ("o/!", Some(("", "o", "", NonOptIndex::Null, true, false))),
            ("option", Some(("", "option", "", NonOptIndex::Null, false, true))),
            ("option!", Some(("", "option", "", NonOptIndex::Null, false, false))),
            ("option/", Some(("", "option", "", NonOptIndex::Null, true, true))),
            ("option!/", Some(("", "option", "", NonOptIndex::Null, true, false))),
            ("option/!", Some(("", "option", "", NonOptIndex::Null, true, false))),

            ("-o", Some(("", "o", "-", NonOptIndex::Null, false, true))),
            ("-o!", Some(("", "o", "-", NonOptIndex::Null, false, false))),
            ("-o/", Some(("", "o", "-", NonOptIndex::Null, true, true))),
            ("-o!/", Some(("", "o", "-", NonOptIndex::Null, true, false))),
            ("-o/!", Some(("", "o", "-", NonOptIndex::Null, true, false))),
            ("-option", Some(("", "option", "-", NonOptIndex::Null, false, true))),
            ("-option!", Some(("", "option", "-", NonOptIndex::Null, false, false))),
            ("-option/", Some(("", "option", "-", NonOptIndex::Null, true, true))),
            ("-option!/", Some(("", "option", "-", NonOptIndex::Null, true, false))),
            ("-option/!", Some(("", "option", "-", NonOptIndex::Null, true, false))),

            ("--o", Some(("", "o", "--", NonOptIndex::Null, false, true))),
            ("--o!", Some(("", "o", "--", NonOptIndex::Null, false, false))),
            ("--o/", Some(("", "o", "--", NonOptIndex::Null, true, true))),
            ("--o!/", Some(("", "o", "--", NonOptIndex::Null, true, false))),
            ("--o/!", Some(("", "o", "--", NonOptIndex::Null, true, false))),
            ("--option", Some(("", "option", "--", NonOptIndex::Null, false, true))),
            ("--option!", Some(("", "option", "--", NonOptIndex::Null, false, false))),
            ("--option/", Some(("", "option", "--", NonOptIndex::Null, true, true))),
            ("--option!/", Some(("", "option", "--", NonOptIndex::Null, true, false))),
            ("--option/!", Some(("", "option", "--", NonOptIndex::Null, true, false))),

            ("o=a@1", Some(("a", "o", "", NonOptIndex::Forward(1), false, true))),
            ("o=a!@1", Some(("a", "o", "", NonOptIndex::Forward(1), false, false))),
            ("o=a/@1", Some(("a", "o", "", NonOptIndex::Forward(1), true, true))),
            ("o=a!/@1", Some(("a", "o", "", NonOptIndex::Forward(1), true, false))),
            ("o=a/!@1", Some(("a", "o", "", NonOptIndex::Forward(1), true, false))),
            ("option=a@1", Some(("a", "option", "", NonOptIndex::Forward(1), false, true))),
            ("option=a!@1", Some(("a", "option", "", NonOptIndex::Forward(1), false, false))),
            ("option=a/@1", Some(("a", "option", "", NonOptIndex::Forward(1), true, true))),
            ("option=a!/@1", Some(("a", "option", "", NonOptIndex::Forward(1), true, false))),
            ("option=a/!@1", Some(("a", "option", "", NonOptIndex::Forward(1), true, false))),

            ("-o=a@1", Some(("a", "o", "-", NonOptIndex::Forward(1), false, true))),
            ("-o=a!@1", Some(("a", "o", "-", NonOptIndex::Forward(1), false, false))),
            ("-o=a/@1", Some(("a", "o", "-", NonOptIndex::Forward(1), true, true))),
            ("-o=a!/@1", Some(("a", "o", "-", NonOptIndex::Forward(1), true, false))),
            ("-o=a/!@1", Some(("a", "o", "-", NonOptIndex::Forward(1), true, false))),
            ("-option=a@1", Some(("a", "option", "-", NonOptIndex::Forward(1), false, true))),
            ("-option=a!@1", Some(("a", "option", "-", NonOptIndex::Forward(1), false, false))),
            ("-option=a/@1", Some(("a", "option", "-", NonOptIndex::Forward(1), true, true))),
            ("-option=a!/@1", Some(("a", "option", "-", NonOptIndex::Forward(1), true, false))),
            ("-option=a/!@1", Some(("a", "option", "-", NonOptIndex::Forward(1), true, false))),
            
            ("o@1", Some(("", "o", "", NonOptIndex::Forward(1), false, true))),
            ("o!@1", Some(("", "o", "", NonOptIndex::Forward(1), false, false))),
            ("o/@1", Some(("", "o", "", NonOptIndex::Forward(1), true, true))),
            ("o!/@1", Some(("", "o", "", NonOptIndex::Forward(1), true, false))),
            ("o/!@1", Some(("", "o", "", NonOptIndex::Forward(1), true, false))),
            ("option@1", Some(("", "option", "", NonOptIndex::Forward(1), false, true))),
            ("option!@1", Some(("", "option", "", NonOptIndex::Forward(1), false, false))),
            ("option/@1", Some(("", "option", "", NonOptIndex::Forward(1), true, true))),
            ("option!/@1", Some(("", "option", "", NonOptIndex::Forward(1), true, false))),
            ("option/!@1", Some(("", "option", "", NonOptIndex::Forward(1), true, false))),

            ("-o@1", Some(("", "o", "-", NonOptIndex::Forward(1), false, true))),
            ("-o!@1", Some(("", "o", "-", NonOptIndex::Forward(1), false, false))),
            ("-o/@1", Some(("", "o", "-", NonOptIndex::Forward(1), true, true))),
            ("-o!/@1", Some(("", "o", "-", NonOptIndex::Forward(1), true, false))),
            ("-o/!@1", Some(("", "o", "-", NonOptIndex::Forward(1), true, false))),
            ("-option@1", Some(("", "option", "-", NonOptIndex::Forward(1), false, true))),
            ("-option!@1", Some(("", "option", "-", NonOptIndex::Forward(1), false, false))),
            ("-option/@1", Some(("", "option", "-", NonOptIndex::Forward(1), true, true))),
            ("-option!/@1", Some(("", "option", "-", NonOptIndex::Forward(1), true, false))),
            ("-option/!@1", Some(("", "option", "-", NonOptIndex::Forward(1), true, false))),

            ("o=a@-3", Some(("a", "o", "", NonOptIndex::Backward(3), false, true))),
            ("o=a!@-3", Some(("a", "o", "", NonOptIndex::Backward(3), false, false))),
            ("o=a/@-3", Some(("a", "o", "", NonOptIndex::Backward(3), true, true))),
            ("o=a!/@-3", Some(("a", "o", "", NonOptIndex::Backward(3), true, false))),
            ("o=a/!@-3", Some(("a", "o", "", NonOptIndex::Backward(3), true, false))),
            ("option=a@-3", Some(("a", "option", "", NonOptIndex::Backward(3), false, true))),
            ("option=a!@-3", Some(("a", "option", "", NonOptIndex::Backward(3), false, false))),
            ("option=a/@-3", Some(("a", "option", "", NonOptIndex::Backward(3), true, true))),
            ("option=a!/@-3", Some(("a", "option", "", NonOptIndex::Backward(3), true, false))),
            ("option=a/!@-3", Some(("a", "option", "", NonOptIndex::Backward(3), true, false))),

            ("o@-3", Some(("", "o", "", NonOptIndex::Backward(3), false, true))),
            ("o!@-3", Some(("", "o", "", NonOptIndex::Backward(3), false, false))),
            ("o/@-3", Some(("", "o", "", NonOptIndex::Backward(3), true, true))),
            ("o!/@-3", Some(("", "o", "", NonOptIndex::Backward(3), true, false))),
            ("o/!@-3", Some(("", "o", "", NonOptIndex::Backward(3), true, false))),
            ("option@-3", Some(("", "option", "", NonOptIndex::Backward(3), false, true))),
            ("option!@-3", Some(("", "option", "", NonOptIndex::Backward(3), false, false))),
            ("option/@-3", Some(("", "option", "", NonOptIndex::Backward(3), true, true))),
            ("option!/@-3", Some(("", "option", "", NonOptIndex::Backward(3), true, false))),
            ("option/!@-3", Some(("", "option", "", NonOptIndex::Backward(3), true, false))),

            ("o=a@1!", None),
            ("o=a@1/", None),
            ("o=a@1!/", None),
            ("o=a@1/!", None),
            ("option=a@1!", None),
            ("option=a@1/", None),
            ("option=a@1!/", None),
            ("option=a@1/!", None),

            ("o@1!", None),
            ("o@1/", None),
            ("o@1!/", None),
            ("o@1/!", None),
            ("option@1!", None),
            ("option@1/", None),
            ("option@1!/", None),
            ("option@1/!", None),

            ("o=a@-3!", None),
            ("o=a@-3/", None),
            ("o=a@-4!/", None),
            ("o=a@-5/!", None),
            ("option=a@-1!", None),
            ("option=a@-2/", None),
            ("option=a@-1!/", None),
            ("option=a@-2/!", None),
        ];

        let prefixs = vec!["--".to_owned(), "/".to_owned(), "-".to_owned()];

        for case in test_cases.iter() {
            match parse_opt_string(case.0, &prefixs) {
                Ok(ci) => {
                    let test_ci = case.1.as_ref().unwrap();

                    assert_eq!(ci.opt_prefix.as_ref().unwrap_or(&String::from("")).as_str(), test_ci.2);
                    assert_eq!(ci.opt_name.as_ref().unwrap_or(&String::from("")).as_str(), test_ci.1);
                    assert_eq!(ci.type_name.as_ref().unwrap_or(&String::from("")).as_str(), test_ci.0);
                    assert_eq!(ci.opt_index, test_ci.3);
                    assert_eq!(ci.deactivate.as_ref().unwrap_or(&false), &test_ci.4);
                    assert_eq!(ci.optional.as_ref().unwrap_or(&true), &test_ci.5);
                }
                Err(_) => {
                    assert_eq!(true, case.1.is_none());
                }
            }
        }
    }
}

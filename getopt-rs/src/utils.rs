
use std::fmt::Debug;

use crate::opt::Opt;
use crate::opt::OptIndex;
use crate::opt::OptValue;
use crate::error::Error;
use crate::error::Result;
use crate::proc::Info;
use crate::id::Identifier;

pub trait Utils: Debug {
    fn type_name(&self) -> &str;

    fn is_support_deactivate_style(&self) -> bool;

    fn create(&self, id: Identifier, ci: &CreateInfo) -> Box<dyn Opt>;

    fn gen_info(&self, opt: &dyn Opt) -> Box<dyn Info>;
}

#[derive(Debug)]
pub struct CreateInfo {
    deactivate: bool,

    optional: bool,

    type_name: String,

    opt_name: String,

    opt_prefix: String,

    opt_index: OptIndex,

    opt_alias: Vec<String>,

    opt_value: OptValue,
}

impl CreateInfo {
    pub fn new(
        type_name: &str,
        name: &str,
        prefix: &str,
        index: OptIndex,
        deactivate_style: bool,
        optional: bool,
        deafult_value: OptValue,
    ) -> Self {
        Self {
            type_name: String::from(type_name),
            opt_name: String::from(name),
            opt_prefix: String::from(prefix),
            opt_index: index,
            deactivate: deactivate_style,
            optional: optional,
            opt_alias: vec![],
            opt_value: deafult_value,
        }
    }

    pub fn parse(s: &str) -> Result<Self> {
        let pr = parse_opt_string(s)?;
        Ok(Self {
            type_name: pr.type_name,
            opt_name: pr.opt_name,
            opt_prefix: pr.opt_prefix,
            opt_index: pr.opt_index,
            deactivate: pr.deactivate,
            optional: pr.optional,
            opt_alias: vec![],
            opt_value: OptValue::Null,
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
    pub fn get_index(&self) -> &OptIndex {
        &self.opt_index
    }

    /// Return the option alias
    pub fn get_alias(&self) -> Vec<&str> {
        self.opt_alias
            .iter()
            .map(|s|s.as_str())
            .collect()
    }

    pub fn get_default_value(&self) -> &OptValue {
        &self.opt_value
    }

    pub fn set_deactivate_style(&mut self, deactivate: bool) {
        self.deactivate = deactivate;
    }

    pub fn set_optional(&mut self, optional: bool) {
        self.optional = optional;
    }

    pub fn set_type_name(&mut self, opt_type: &str) {
        self.type_name = String::from(opt_type);
    }

    pub fn set_name(&mut self, opt_name: &str) {
        self.opt_name = String::from(opt_name);
    }

    pub fn set_prefix(&mut self, prefix: &str) {
        self.opt_prefix = String::from(prefix);
    }

    pub fn set_index(&mut self, index: OptIndex) {
        self.opt_index = index;
    }

    pub fn set_deafult_value(&mut self, value: OptValue) {
        self.opt_value = value;
    }

    pub fn add_alias(&mut self, alias: &str) {
        self.opt_alias.push(String::from(alias));
    }

    pub fn rem_alias(&mut self, s: &str) {
        for index in 0 .. self.opt_alias.len() {
            if self.opt_alias[index] == s {
                self.opt_alias.remove(index);
                break;
            }
        }
    }

    pub fn clr_alias(&mut self) {
        self.opt_alias.clear();
    }
}

#[derive(Debug)]
pub struct FilterInfo {
    deactivate: bool,

    optional: bool,

    type_name: String,

    opt_name: String,

    opt_prefix: String,

    opt_index: OptIndex,
}

impl FilterInfo {
    pub fn new() -> Self {
        Self {
            deactivate: false,
            optional: true,
            type_name: String::default(),
            opt_name: String::default(),
            opt_prefix: String::default(),
            opt_index: OptIndex::Null,
        }
    }

    pub fn parse(opt: &str) -> Result<Self> {
        let pr = parse_opt_string(opt)?;
        Ok(Self {
            deactivate: pr.deactivate,
            optional: pr.optional,
            type_name: pr.type_name,
            opt_name: pr.opt_name,
            opt_prefix: pr.opt_prefix,
            opt_index: pr.opt_index,
        })
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
    pub fn get_index(&self) -> &OptIndex {
        &self.opt_index
    }

    pub fn set_deactivate_style(&mut self, deactivate: bool) {
        self.deactivate = deactivate;
    }

    pub fn set_optional(&mut self, optional: bool) {
        self.optional = optional;
    }

    pub fn set_type_name(&mut self, opt_type: &str) {
        self.type_name = String::from(opt_type);
    }

    pub fn set_name(&mut self, opt_name: &str) {
        self.opt_name = String::from(opt_name);
    }

    pub fn set_prefix(&mut self, prefix: &str) {
        self.opt_prefix = String::from(prefix);
    }

    pub fn set_index(&mut self, index: OptIndex) {
        self.opt_index = index;
    }
}

#[derive(Debug)]
struct ParseResult {
    type_name: String,

    opt_name: String,

    opt_prefix: String,

    deactivate: bool,

    optional: bool,

    opt_index: OptIndex,
}

/// Parse input string `<name>=<type>[!][/]@<index>`,
/// such as `o=a!`, "o=p@1". The option name is `o`, and type is `a`.
/// `!` means the option is optional or not.
/// `/` means the option is deactivate style or not.
fn parse_opt_string(s: &str) -> Result<ParseResult> {
    const SPLIT: &str = "=";
    const DEACTIVATE: &str = "/";
    const NO_OPTIONAL: &str = "!";
    const INDEX: &str = "@";

    let splited: Vec<_> = s.split(SPLIT).collect();
    let mut type_last_index = 0;
    let mut deactivate = false;
    let mut optional = true;
    let mut opt_index = OptIndex::Null;

    if splited.len() == 2 {
        if let Some(index) = splited[1].rfind(DEACTIVATE) {
            deactivate = true;
            if index != 0 {
                type_last_index = index;
            }
        }
        if let Some(index) = splited[1].rfind(NO_OPTIONAL) {
            optional = false;
            if index != 0 && (index < type_last_index || type_last_index == 0) {
                type_last_index = index;
            }
        }
        if let Some(index) = splited[1].rfind(INDEX) {
            match splited[1].split_at(index + 1).1.parse::<i32>() {
                Ok(v) => {
                    opt_index = OptIndex::new(v);
                }
                Err(_) => {
                    return Err(Error::InvalidOptionStr(String::from(s)))
                }
            }
            if index != 0 && (index < type_last_index || type_last_index == 0) {
                type_last_index = index;
            }
        }
        let (opt_type, _) = if type_last_index == 0 {
            (splited[1], splited[0] /* fine, not using*/)
        } else {
            splited[1].split_at(type_last_index)
        };

        return Ok(ParseResult {
            type_name: String::from(opt_type),
            opt_name: String::from(splited[0]),
            opt_prefix: String::default(),
            deactivate,
            optional,
            opt_index,
        })
    }
    else {
        if s.is_empty() {
            Ok(ParseResult {
                type_name: String::default(),
                opt_name: String::default(),
                opt_prefix: String::default(),
                deactivate,
                optional,
                opt_index,
            })
        }
        else {
            Err(Error::InvalidOptionStr(String::from(s)))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::CreateInfo;
    use crate::opt::OptIndex;

    #[test]
    fn str_can_parse_to_create_info() {
        let test_cases = vec![
            ("o=a", Some(CreateInfo::new("a", "o", "", OptIndex::Null, false, true))),
            ("o=a!", Some(CreateInfo::new("a", "o", "", OptIndex::Null, false, false))),
            ("o=a/", Some(CreateInfo::new("a", "o", "", OptIndex::Null, true, true))),
            ("o=a!/", Some(CreateInfo::new("a", "o", "", OptIndex::Null, true, false))),
            ("o=a/!", Some(CreateInfo::new("a", "o", "", OptIndex::Null, true, false))),
            ("option=a", Some(CreateInfo::new("a", "option", "", OptIndex::Null, false, true))),
            ("option=a!", Some(CreateInfo::new("a", "option", "", OptIndex::Null, false, false))),
            ("option=a/", Some(CreateInfo::new("a", "option", "", OptIndex::Null, true, true))),
            ("option=a!/", Some(CreateInfo::new("a", "option", "", OptIndex::Null, true, false))),
            ("option=a/!", Some(CreateInfo::new("a", "option", "", OptIndex::Null, true, false))),

            ("o=a@1", Some(CreateInfo::new("a", "o", "", OptIndex::Forward(1), false, true))),
            ("o=a!@1", Some(CreateInfo::new("a", "o", "", OptIndex::Forward(1), false, false))),
            ("o=a/@1", Some(CreateInfo::new("a", "o", "", OptIndex::Forward(1), true, true))),
            ("o=a!/@1", Some(CreateInfo::new("a", "o", "", OptIndex::Forward(1), true, false))),
            ("o=a/!@1", Some(CreateInfo::new("a", "o", "", OptIndex::Forward(1), true, false))),
            ("option=a@1", Some(CreateInfo::new("a", "option", "", OptIndex::Forward(1), false, true))),
            ("option=a!@1", Some(CreateInfo::new("a", "option", "", OptIndex::Forward(1), false, false))),
            ("option=a/@1", Some(CreateInfo::new("a", "option", "", OptIndex::Forward(1), true, true))),
            ("option=a!/@1", Some(CreateInfo::new("a", "option", "", OptIndex::Forward(1), true, false))),
            ("option=a/!@1", Some(CreateInfo::new("a", "option", "", OptIndex::Forward(1), true, false))),

            ("o=a@-3", Some(CreateInfo::new("a", "o", "", OptIndex::Backward(3), false, true))),
            ("o=a!@-3", Some(CreateInfo::new("a", "o", "", OptIndex::Backward(3), false, false))),
            ("o=a/@-3", Some(CreateInfo::new("a", "o", "", OptIndex::Backward(3), true, true))),
            ("o=a!/@-3", Some(CreateInfo::new("a", "o", "", OptIndex::Backward(3), true, false))),
            ("o=a/!@-3", Some(CreateInfo::new("a", "o", "", OptIndex::Backward(3), true, false))),
            ("option=a@-3", Some(CreateInfo::new("a", "option", "", OptIndex::Backward(3), false, true))),
            ("option=a!@-3", Some(CreateInfo::new("a", "option", "", OptIndex::Backward(3), false, false))),
            ("option=a/@-3", Some(CreateInfo::new("a", "option", "", OptIndex::Backward(3), true, true))),
            ("option=a!/@-3", Some(CreateInfo::new("a", "option", "", OptIndex::Backward(3), true, false))),
            ("option=a/!@-3", Some(CreateInfo::new("a", "option", "", OptIndex::Backward(3), true, false))),

            ("o=a@1!", None),
            ("o=a@1/", None),
            ("o=a@1!/", None),
            ("o=a@1/!", None),
            ("option=a@1!", None),
            ("option=a@1/", None),
            ("option=a@1!/", None),
            ("option=a@1/!", None),

            ("o=a@-3!", None),
            ("o=a@-3/", None),
            ("o=a@-4!/", None),
            ("o=a@-5/!", None),
            ("option=a@-1!", None),
            ("option=a@-2/", None),
            ("option=a@-1!/", None),
            ("option=a@-2/!", None),
        ];

        for case in test_cases.iter() {
            match CreateInfo::parse(case.0) {
                Ok(ci) => {
                    let test_ci = case.1.as_ref().unwrap();

                    assert_eq!(ci.get_type_name(), test_ci.get_type_name());
                    assert_eq!(ci.get_name(), test_ci.get_name());
                    assert_eq!(ci.get_prefix(), test_ci.get_prefix());
                    assert_eq!(ci.get_index(), test_ci.get_index());
                    assert_eq!(ci.get_alias(), test_ci.get_alias());
                }
                Err(_) => {
                    assert_eq!(true, case.1.is_none());
                }
            }
        }
    }
}

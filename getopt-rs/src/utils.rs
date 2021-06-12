
use std::fmt::Debug;
use std::borrow::Cow;

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
pub struct CreateInfo<'a, 'b, 'c, 'd> {
    deactivate: bool,

    optional: bool,

    type_name: Cow<'a, str>,

    opt_name: Cow<'b, str>,

    opt_prefix: Cow<'c, str>,

    opt_index: NonOptIndex,

    opt_alias: Vec<(Cow<'d, str>, Cow<'d, str>)>,

    opt_value: OptValue,

    opt_callback_type: CallbackType,
}

impl<'a, 'b, 'c, 'd> CreateInfo<'a, 'b, 'c, 'd> {
    pub fn new(
        type_name: impl Into<Cow<'a, str>>,
        name: impl Into<Cow<'b, str>>,
        prefix: impl Into<Cow<'c, str>>,
        index: NonOptIndex,
        deactivate_style: bool,
        optional: bool,
        deafult_value: OptValue,
        opt_callback_type: CallbackType,
    ) -> Self {
        Self {
            type_name: type_name.into(),
            opt_name: name.into(),
            opt_prefix: prefix.into(),
            opt_index: index,
            deactivate: deactivate_style,
            optional,
            opt_alias: vec![],
            opt_value: deafult_value,
            opt_callback_type,
        }
    }

    pub fn parse(s: &str, prefixs: &'c Vec<String>) -> Result<Self> {
        let pr = parse_opt_string(s, prefixs)?;
        let type_name = pr.type_name.ok_or(Error::NullOptionType)?;
        let opt_name = pr.opt_name.ok_or(Error::NullOptionName)?;
        Ok(Self {
            type_name,
            opt_name,
            opt_prefix: pr.opt_prefix.unwrap_or(Cow::Borrowed("")),
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
    pub fn get_type_name(&self) -> &Cow<'a, str> {
        &self.type_name
    }

    /// Return the option name
    pub fn get_name(&self) -> &Cow<'b, str> {
        &self.opt_name
    }

    /// Return the option prefix
    pub fn get_prefix(&self) -> &Cow<'c, str> {
        &self.opt_prefix
    }

    /// Return the option index
    pub fn get_index(&self) -> &NonOptIndex {
        &self.opt_index
    }

    /// Return the option alias
    pub fn get_alias(&self) -> &Vec<(Cow<'d, str>, Cow<'d, str>)> {
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

    pub fn set_type_name(&mut self, opt_type: Cow<'a, str>) {
        self.type_name = opt_type;
    }

    pub fn set_name(&mut self, opt_name: Cow<'b, str>) {
        self.opt_name = opt_name;
    }

    pub fn set_prefix(&mut self, prefix: Cow<'c, str>) {
        self.opt_prefix = prefix;
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

    pub fn add_alias(&mut self, prefix: Cow<'d, str>, name: Cow<'d, str>) {
        self.opt_alias.push((prefix, name));
    }

    pub fn rem_alias(&mut self, prefix: Cow<'d, str>, name: Cow<'d, str>) {
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
pub struct FilterInfo<'a, 'b, 'c> {
    deactivate: Option<bool>,

    optional: Option<bool>,

    type_name: Option<Cow<'a, str>>,

    opt_name: Option<Cow<'b, str>>,

    opt_prefix: Option<Cow<'c, str>>,

    opt_index: NonOptIndex,
}

impl<'a, 'b, 'c> FilterInfo<'a, 'b, 'c> {
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

    pub fn parse(opt: &str, prefixs: &'c Vec<String>) -> Result<Self> {
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
        self.type_name.as_ref().unwrap().as_ref()
    }

    /// Return the option name
    pub fn get_name(&self) -> &str {
        self.opt_name.as_ref().unwrap().as_ref()
    }

    /// Return the option prefix
    pub fn get_prefix(&self) -> &str {
        self.opt_prefix.as_ref().unwrap().as_ref()
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

    pub fn set_type_name(&mut self, opt_type: Cow<'a, str>) {
        self.type_name = Some(opt_type);
    }

    pub fn set_name(&mut self, opt_name: Cow<'b, str>) {
        self.opt_name = Some(opt_name);
    }

    pub fn set_prefix(&mut self, prefix: Cow<'c, str>) {
        self.opt_prefix = Some(prefix);
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
struct ParseResult<'a, 'b> {
    type_name: Option<Cow<'a, str>>,

    opt_name: Option<Cow<'a, str>>,

    opt_prefix: Option<Cow<'b, str>>,

    deactivate: Option<bool>,

    optional: Option<bool>,

    opt_index: NonOptIndex,
}

/// Parse input string `<prefix>|<name>=<type>[!][/]@<index>`,
/// such as `-|o=a!`, means the force required option `o`, with a prefix "-", 
/// and option type is `a`.
/// `!` means the option is optional or not.
/// `/` means the option is deactivate style or not.
fn parse_opt_string<'a, 'b, 'c>(pattern: &'a str, prefix: &'b Vec<String>) -> Result<ParseResult<'c, 'b>> {
    let mut pattern = opt_parser::ParserPattern::new(pattern, prefix);
    let mut data_keeper = opt_parser::DataKeeper::default();

    let res = opt_parser::State::new().parse(&mut pattern, &mut data_keeper)?;

    if res {
        let pr = ParseResult {
            type_name: data_keeper.type_name,
            opt_name: data_keeper.name,
            opt_prefix: data_keeper.prefix,
            deactivate: Some(data_keeper.deactivate),
            optional: Some(! data_keeper.optional),
            opt_index: {
                if data_keeper.forward_index.is_some() {
                    NonOptIndex::forward(data_keeper.forward_index.unwrap())
                }
                else if data_keeper.backward_index.is_some() {
                    NonOptIndex::backward(data_keeper.backward_index.unwrap())
                }
                else if data_keeper.anywhere.unwrap_or(false) {
                    NonOptIndex::anywhere()
                }
                else if data_keeper.list.len() > 0 {
                    NonOptIndex::list(data_keeper.list.clone())
                }
                else if data_keeper.except.len() > 0 {
                    NonOptIndex::except(data_keeper.except.clone())
                }
                else {
                    NonOptIndex::default()
                }
            }
        };
        Ok(pr)
    }
    else {
        Err(Error::InvalidOptionStr(format!("{:?}", pattern)))
    }
}

pub mod opt_parser {
    use std::borrow::Cow;
    use std::str::Chars;
    use std::iter::Skip;
    use crate::error::Result;
    use crate::error::Error;
        
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub enum State {
        PreCheck,
        Prefix,
        Name,
        Equal,
        Type,
        Deactivate,
        Optional,
        Index,
        FowradIndex,
        BackwardIndex,
        List,
        Except,
        End,
    }

    #[derive(Debug)]
    pub struct ParserPattern<'a, 'b> {
        pub pattern: &'a str,

        pub support_prefix: &'b Vec<String>,

        pub cur_index: usize,

        pub end_index: usize,
    }

    impl<'a, 'b> ParserPattern<'a, 'b> {
        pub fn new(pattern: &'a str, prefix: &'b Vec<String>) -> Self {
            Self {
                support_prefix: prefix,
                end_index: pattern.len(),
                cur_index: 0,
                pattern,
            }
        }

        pub fn is_end(&self) -> bool {
            self.cur_index == self.end_index
        }

        pub fn inc_index(&mut self, inc: usize) {
            self.cur_index += inc;
        }

        pub fn set_index(&mut self, index: usize) {
            self.cur_index = index;
        }

        pub fn chars(&self) -> Skip<Chars> {
            self.pattern.chars().skip(self.cur_index)
        }
    }

    #[derive(Debug, Default)]
    pub struct DataKeeper<'a, 'b> {
        pub prefix: Option<Cow<'b, str>>,

        pub name: Option<Cow<'a, str>>,

        pub type_name: Option<Cow<'a, str>>,

        pub deactivate: bool,

        pub optional: bool,

        pub forward_index: Option<u64>,

        pub backward_index: Option<u64>,

        pub anywhere: Option<bool>,

        pub list: Vec<u64>,

        pub except: Vec<u64>,
    }

    impl<'a, 'b> ToString for DataKeeper<'a, 'b> {
        fn to_string(&self) -> String {
            let mut out = String::default();

            if self.prefix.is_some() {
                out += &format!("{}", self.prefix.as_ref().unwrap());
            }
            if self.name.is_some() {
                out += &format!("{}", self.name.as_ref().unwrap());
            }
            if self.type_name.is_some() {
                out += &format!("={}", self.type_name.as_ref().unwrap());
            }
            if self.deactivate {
                out += "/";
            }
            if self.optional {
                out += "!";
            }
            if self.forward_index.is_some() {
                out += &format!("@{}", self.forward_index.unwrap());
            } else if self.backward_index.is_some() {
                out += &format!("@-{}", self.backward_index.unwrap());
            } else if self.anywhere.is_some() {
                out += "@0";
            } else if self.list.len() > 0 {
                out += "@[";
                for (i, ch) in self.list.iter().enumerate() {
                    out += &format!("{}", ch);
                    if i < self.list.len() - 1 {
                        out += ",";
                    }
                }
                out += "]";
            } else if self.except.len() > 0 {
                out += "@-[";
                for (i, ch) in self.except.iter().enumerate() {
                    out += &format!("{}", ch);
                    if i < self.except.len() - 1 {
                        out += ",";
                    }
                }
                out += "]";
            }
            out
        }
    }

    impl<'a, 'b, 'c> State {
        pub fn new() -> Self {
            Self::PreCheck
        }

        pub fn transition(self, pattern: &mut ParserPattern<'a, 'b>) -> Self {
            match self {
                Self::PreCheck => {
                    if pattern.pattern.is_empty() {
                        Self::End
                    } else {
                        Self::Prefix
                    }
                }
                State::Prefix => {
                    if pattern.is_end() {
                        Self::End
                    } else {
                        Self::Name
                    }
                }
                State::Name => {
                    if pattern.is_end() {
                        Self::End
                    } else {
                        if let Some(ch) = pattern.chars().nth(0) {
                            match ch {
                                // equal state will increment the index
                                '=' => Self::Equal,
                                _ => Self::Type,
                            }
                        } else {
                            Self::End
                        }
                    }
                }
                State::Equal => {
                    if pattern.is_end() {
                        Self::End
                    } else {
                        Self::Type
                    }
                }
                State::Type | State::Deactivate | State::Optional => {
                    if pattern.is_end() {
                        Self::End
                    } else {
                        if let Some(ch) = pattern.chars().nth(0) {
                            match ch {
                                '!' => Self::Optional,
                                '/' => Self::Deactivate,
                                '@' => Self::Index,
                                _ => Self::End,
                            }
                        } else {
                            Self::End
                        }
                    }
                }
                State::Index => {
                    if pattern.is_end() {
                        Self::End
                    } else {
                        let (_, index_part) = pattern.pattern.split_at(pattern.cur_index);

                        if index_part.starts_with("+[") || index_part.starts_with("[") {
                            State::List
                        } else if index_part.starts_with("-[") {
                            State::Except
                        } else if index_part.starts_with("-") {
                            State::BackwardIndex
                        } else {
                            State::FowradIndex
                        }
                    }
                }
                State::FowradIndex | State::BackwardIndex | State::List | State::Except => State::End,
                State::End => {
                    State::End
                },
            }
        }

        pub fn parse(
            self,
            pattern: &mut ParserPattern<'a, 'b>,
            data_keeper: &mut DataKeeper<'c, 'b>,
        ) -> Result<bool> {
            if !pattern.is_end() && self != State::End {
                let state = self.transition(pattern);

                match state {
                    State::Prefix => {
                        for prefix in pattern.support_prefix {
                            if pattern.pattern.starts_with(prefix) {
                                data_keeper.prefix = Some(Cow::Borrowed(prefix.as_str()));
                                pattern.inc_index(prefix.len());
                                break;
                            }
                        }
                    }
                    State::Name => {
                        let mut cur_index = pattern.cur_index;

                        for ch in pattern.chars() {
                            cur_index += 1;
                            if ch == '=' || ch == '!' || ch == '/' || ch == '@' {
                                if cur_index - pattern.cur_index > 1 {
                                    data_keeper.name = Some(Cow::Owned(String::from(
                                        pattern
                                            .pattern
                                            .get(pattern.cur_index..cur_index - 1)
                                            .unwrap(),
                                    )));
                                    pattern.set_index(cur_index - 1);
                                }
                                break;
                            } else if cur_index == pattern.end_index {
                                if cur_index - pattern.cur_index > 1 {
                                    data_keeper.name = Some(Cow::Owned(String::from(
                                        pattern.pattern.get(pattern.cur_index..cur_index).unwrap(),
                                    )));
                                    pattern.set_index(cur_index);
                                }
                                break;
                            }
                        }
                    }
                    State::Equal => {
                        pattern.inc_index(1);
                    }
                    State::Type => {
                        let mut cur_index = pattern.cur_index;

                        for ch in pattern.chars() {
                            cur_index += 1;
                            if ch == '!' || ch == '/' || ch == '@' {
                                if cur_index - pattern.cur_index > 1 {
                                    data_keeper.type_name = Some(Cow::Owned(String::from(
                                        pattern
                                            .pattern
                                            .get(pattern.cur_index..cur_index - 1)
                                            .unwrap(),
                                    )));
                                    pattern.set_index(cur_index - 1);
                                }
                                break;
                            } else if cur_index == pattern.end_index {
                                if cur_index - pattern.cur_index > 1 {
                                    data_keeper.type_name = Some(Cow::Owned(String::from(
                                        pattern.pattern.get(pattern.cur_index..cur_index).unwrap(),
                                    )));
                                    pattern.set_index(cur_index);
                                }
                                break;
                            }
                        }
                    }
                    State::Deactivate => {
                        data_keeper.deactivate = true;
                        pattern.inc_index(1);
                    }
                    State::Optional => {
                        data_keeper.optional = true;
                        pattern.inc_index(1);
                    }
                    State::Index => {
                        pattern.inc_index(1);
                    }
                    State::FowradIndex => {
                        let (_, index_part) = pattern.pattern.split_at(pattern.cur_index);

                        let index = index_part.parse::<u64>()
                                                        .map_err(|e| Error::InavlidNonOptionIndex(format!("{:?}", e)))?;
                        if index > 0 {
                            data_keeper.forward_index = Some(index);
                        }
                        else {
                            data_keeper.anywhere = Some(true);
                        }
                        pattern.set_index(pattern.end_index);
                    }
                    State::BackwardIndex => {
                        let (_, index_part) = pattern.pattern.split_at(pattern.cur_index + 1);

                        let index = index_part.parse::<u64>()
                                                        .map_err(|e| Error::InavlidNonOptionIndex(format!("{:?}", e)))?;
                        if index > 0 {
                            data_keeper.backward_index = Some(index);
                        }
                        else {
                            data_keeper.anywhere = Some(true);
                        }
                        pattern.set_index(pattern.end_index);
                    }
                    State::List => {
                        let (_, index_part) = pattern.pattern.split_at(pattern.cur_index);

                        if index_part.starts_with("+[") {
                            let index_part = pattern
                                .pattern
                                .get(pattern.cur_index + 2..pattern.end_index - 1)
                                .unwrap();

                            data_keeper.list = index_part
                                .split(',')
                                .map(|v| v.parse::<u64>().map_err(|e| Error::InavlidNonOptionIndex(format!("{:?}", e))))
                                .collect::<Result<Vec<u64>>>()?;
                        } else {
                            let index_part = pattern
                                .pattern
                                .get(pattern.cur_index + 1..pattern.end_index - 1)
                                .unwrap();

                            data_keeper.list = index_part
                                .split(',')
                                .map(|v| v.parse::<u64>().map_err(|e| Error::InavlidNonOptionIndex(format!("{:?}", e))))
                                .collect::<Result<Vec<u64>>>()?;
                        }
                        pattern.set_index(pattern.end_index);
                    }
                    State::Except => {
                        let index_part = pattern
                            .pattern
                            .get(pattern.cur_index + 2..pattern.end_index - 1)
                            .unwrap();

                        data_keeper.except = index_part
                            .split(',')
                            .map(|v| v.parse::<u64>().map_err(|e| Error::InavlidNonOptionIndex(format!("{:?}", e))))
                            .collect::<Result<Vec<u64>>>()?;
                        pattern.set_index(pattern.end_index);
                    }
                    State::End => {
                        if !pattern.is_end() {
                            return Err(Error::InvalidOptionStr(format!("{}", pattern.pattern)));
                        }
                    }
                    _ => {}
                }

                state.parse(pattern, data_keeper)
            }
            else {
                Ok(true)
            }
        }
    }
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

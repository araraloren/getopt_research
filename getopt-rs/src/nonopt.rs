
use crate::opt_def;
use crate::opt_name_def;
use crate::opt_type_def;
use crate::opt_identifier_def;
use crate::opt_index_def;
use crate::opt_alias_def;
use crate::opt_callback_def;
use crate::opt_optional_def;
use crate::opt::Opt;
use crate::callback::CallbackType;
use crate::error::Result;
use crate::error::Error;
use crate::utils::Utils;
use crate::utils::CreateInfo;
use crate::proc::Info;

pub trait NonOpt: Opt { }

pub mod pos {
    use super::*;
    use crate::opt::*;
    use crate::id::Identifier as IIdentifier;

    pub fn current_type() -> &'static str {
        "pos"
    }

    pub trait Pos: NonOpt { }

    /// PosNonOpt target the value to [`bool`](prim@bool), 
    /// 
    /// * The non-option type name is `pos`.
    /// * The option is not support deactivate style.
    /// * The option accept the style [`Style::Pos`].
    /// * In default, the option is `optional`, it can be change through the [`set_optional`](crate::opt::Optional::set_optional).
    /// * The option need an [`OptValue::Bool`] argument, the default value is [`OptValue::default()`].
    /// * The option not support alias.
    /// * The option support callback type [`CallbackType::Index`].
    ///
    /// User can set it at specify index of command line non-option argument.
    #[derive(Debug)]
    pub struct PosNonOpt {
        id: IIdentifier,

        name: String,

        optional: bool,

        value: OptValue,

        index: NonOptIndex,

        callback: CallbackType,

        default_value: OptValue,
    }

    impl PosNonOpt {
        pub fn new(id: IIdentifier, name: String, optional: bool, index: NonOptIndex) -> Self {
            Self {
                id,
                name,
                optional,
                value: OptValue::default(),
                index,
                default_value: OptValue::default(),
                callback: CallbackType::default(),
            }
        }
    }

    opt_def!(PosNonOpt, Pos, NonOpt);

    opt_type_def!(
        PosNonOpt,
        current_type(),
        { style, Style::Pos }
    );

    opt_callback_def!(
        PosNonOpt,
        callback,
        callback_type,
        CallbackType::Index,
        CallbackType::Null,
    );

    opt_identifier_def!(
        PosNonOpt,
        id,
        para,
    );

    impl Name for PosNonOpt {
        fn name(&self) -> &str {
            &self.name
        }

        fn prefix(&self) -> &str {
            ""
        }

        fn set_name(&mut self, name_para: &str) {
            self.name = name_para.to_owned()
        }

        fn set_prefix(&mut self, _: &str) {
            
        }

        fn match_name(&self, _: &str) -> bool {
            // for `Pos`, it only pay attention on position
            true
        }

        fn match_prefix(&self, prefix_para: &str) -> bool {
            self.prefix() == prefix_para
        }
    }

    opt_optional_def!(
        PosNonOpt,
        optional,
        optional,
    );

    opt_alias_def!( PosNonOpt );

    opt_index_def!(
        PosNonOpt,
        index,
        index,
    );

    /// Pos using value hold the return value of callback
    impl Value for PosNonOpt {
        fn value(&self) -> &OptValue {
            &self.value
        }

        fn default_value(&self) -> &OptValue {
            &self.default_value
        }

        fn set_value(&mut self, value_para: OptValue) {
            self.value = value_para;
        }

        // ignore set default value operate
        fn set_default_value(&mut self, _: OptValue) {

        }

        fn parse_value(&self, _: &str) -> Result<OptValue> {
            return Ok(OptValue::from_bool(true));
        }

        fn has_value(&self) -> bool {
            self.value().is_bool()
        }

        fn reset_value(&mut self) {
            self.set_value(self.default_value().clone());
        }
    }

    /// Default [`Utils`] implementation for [`PosNonOpt`]
    #[derive(Debug)]
    pub struct PosUtils;

    impl PosUtils {
        pub fn new() -> Self {
            Self {}
        }
    }

    impl Utils for PosUtils {
        fn type_name(&self) -> &str {
            current_type()
        }

        fn is_support_deactivate_style(&self) -> bool {
            false
        }

        /// Create an [`PosNonOpt`] using option information [`CreateInfo`].
        /// 
        /// ```no_run
        /// use getopt_rs::utils::{Utils, CreateInfo};
        /// use getopt_rs::nonopt::pos::*;
        /// use getopt_rs::id::*;
        /// 
        /// let utils = PosUtils::new();
        /// let ci = CreateInfo::parse("name=pos@2", &vec![]).unwrap();
        /// let _non_opt = utils.create(Identifier::new(1), &ci);
        /// ```
        fn create(&self, id: IIdentifier, ci: &CreateInfo) -> Result<Box<dyn Opt>> {
            if ci.is_deactivate_style() {
                if ! self.is_support_deactivate_style() {
                    return Err(Error::UtilsNotSupportDeactivateStyle(ci.get_name().to_owned()));
                }
            }
            if ci.get_type_name() != self.type_name() {
                return Err(Error::UtilsNotSupportTypeName(self.type_name().to_owned(), ci.get_type_name().to_owned()))
            }
            
            assert_eq!(ci.get_type_name(), self.type_name());
            
            let opt = Box::new(PosNonOpt::new(
                id,
                ci.get_name().to_owned(),
                ci.is_optional(),
                ci.get_index().clone(),
            ));

            Ok(opt)
        }

        fn gen_info(&self, opt: &dyn Opt) -> Box<dyn Info> {
            Box::new(OptionInfo::new(opt.id()))
        }
    }
}

pub mod cmd {
    use super::*;
    use crate::opt::*;
    use crate::id::Identifier as IIdentifier;

    pub fn current_type() -> &'static str {
        "cmd"
    }

    pub trait Cmd: NonOpt { }

    /// CmdNonOpt target the value to [`bool`](prim@bool), 
    /// 
    /// * The non-option type name is `cmd`.
    /// * The option is not support deactivate style.
    /// * The option accept the style [`Style::Cmd`].
    /// * In default, the option is always not `optional`.
    /// * The option need an [`OptValue::Bool`] argument, the default value is [`OptValue::default()`].
    /// * The option not support alias.
    /// * The option support callback type [`CallbackType::Main`].
    ///
    /// User can set it at first command line non-option argument.
    #[derive(Debug)]
    pub struct CmdNonOpt {
        id: IIdentifier,

        name: String,

        optional: bool,

        value: OptValue,

        index: NonOptIndex,

        callback: CallbackType,

        default_value: OptValue,
    }

    impl CmdNonOpt {
        pub fn new(id: IIdentifier, name: String) -> Self {
            Self {
                id,
                name,
                optional: false,
                value: OptValue::default(),
                index: NonOptIndex::new(1), // Cmd is the first noa
                callback: CallbackType::default(),
                default_value: OptValue::default(),
            }
        }
    }

    opt_def!(CmdNonOpt, Cmd, NonOpt);

    opt_type_def!(
        CmdNonOpt, 
        current_type(),
        { style, Style::Cmd }
    );

    opt_callback_def!(
        CmdNonOpt,
        callback,
        callback,
        CallbackType::Main,
        CallbackType::Null,
    );

    opt_identifier_def!(
        CmdNonOpt,
        id,
        para,
    );

    opt_name_def!(
        CmdNonOpt,
        name,
        name,
    );

    impl Optional for CmdNonOpt {
        fn optional(&self) -> bool {
            self.optional
        }

        fn set_optional(&mut self, _: bool) {
            
        }

        fn match_optional(&self, optional_para: bool) -> bool {
            self.optional() == optional_para
        }
    }

    opt_alias_def!( CmdNonOpt );

    impl Index for CmdNonOpt {
        fn index(&self) -> &NonOptIndex {
            &self.index
        }

        /// Can not change the index of [`Cmd`]
        fn set_index(&mut self, _: NonOptIndex) {
            
        }

        fn match_index(&self, total: i64, current: i64) -> bool {
            if let Some(realindex) = self.index().calc_index(total) {
                return realindex == current;
            }
            false
        }
    }

    /// Pos using value hold the return value of callback
    impl Value for CmdNonOpt {
        fn value(&self) -> &OptValue {
            &self.value
        }

        fn default_value(&self) -> &OptValue {
            &self.default_value
        }

        fn set_value(&mut self, value_para: OptValue) {
            self.value = value_para;
        }

        // ignore set default value operate
        fn set_default_value(&mut self, _: OptValue) {

        }

        fn parse_value(&self, _: &str) -> Result<OptValue> {
            return Ok(OptValue::from_bool(true));
        }

        fn has_value(&self) -> bool {
            self.value().is_bool()
        }

        fn reset_value(&mut self) {
            self.set_value(self.default_value().clone());
        }
    }

    /// Default [`Utils`] implementation for [`CmdNonOpt`]
    #[derive(Debug)]
    pub struct CmdUtils;

    impl CmdUtils {
        pub fn new() -> Self {
            Self {}
        }
    }

    impl Utils for CmdUtils {
        fn type_name(&self) -> &str {
            current_type()
        }

        fn is_support_deactivate_style(&self) -> bool {
            false
        }

        /// Create an [`CmdNonOpt`] using option information [`CreateInfo`].
        /// 
        /// ```no_run
        /// use getopt_rs::utils::{Utils, CreateInfo};
        /// use getopt_rs::nonopt::cmd::*;
        /// use getopt_rs::id::*;
        /// 
        /// let utils = CmdUtils::new();
        /// let ci = CreateInfo::parse("name=cmd", &vec![]).unwrap();
        /// let _non_opt = utils.create(Identifier::new(1), &ci);
        /// ```
        fn create(&self, id: IIdentifier, ci: &CreateInfo) -> Result<Box<dyn Opt>> {
            if ci.is_deactivate_style() {
                if ! self.is_support_deactivate_style() {
                    return Err(Error::UtilsNotSupportDeactivateStyle(ci.get_name().to_owned()));
                }
            }
            if ci.get_type_name() != self.type_name() {
                return Err(Error::UtilsNotSupportTypeName(self.type_name().to_owned(), ci.get_type_name().to_owned()))
            }
            
            assert_eq!(ci.get_type_name(), self.type_name());

            let opt = Box::new(CmdNonOpt::new(
                id,
                ci.get_name().to_owned(),
            ));

            Ok(opt)
        }

        fn gen_info(&self, opt: &dyn Opt) -> Box<dyn Info> {
            Box::new(OptionInfo::new(opt.id()))
        }
    }
}

pub mod main {
    use super::*;
    use crate::opt::*;
    use crate::id::Identifier as IIdentifier;

    pub fn current_type() -> &'static str {
        "main"
    }

    pub trait Main: NonOpt { }

    /// MainNonOpt target the value to [`bool`](prim@bool), 
    /// 
    /// * The non-option type name is `main`.
    /// * The option is not support deactivate style.
    /// * The option accept the style [`Style::Main`].
    /// * In default, the option will ignore `optional`.
    /// * The option need an [`OptValue::Bool`] argument, the default value is [`OptValue::default()`].
    /// * The option not support alias.
    /// * The option support callback type [`CallbackType::Main`].
    ///
    /// The [`Parser`](crate::parser::Parser) will always call the callback of `MainNonOpt`.
    #[derive(Debug)]
    pub struct MainNonOpt {
        id: IIdentifier,

        name: String,

        optional: bool,

        value: OptValue,

        index: NonOptIndex,

        callback: CallbackType,

        default_value: OptValue,
    }

    impl MainNonOpt {
        pub fn new(id: IIdentifier, name: String) -> Self {
            Self {
                id,
                name,
                optional: true,
                value: OptValue::default(),
                index: NonOptIndex::default(), // Cmd is the first noa
                callback: CallbackType::default(),
                default_value: OptValue::default(),
            }
        }
    }

    opt_def!(MainNonOpt, Main, NonOpt);

    opt_type_def!(
        MainNonOpt, 
        current_type(),
        false,
        { style, Style::Main }
    );

    opt_callback_def!(
        MainNonOpt,
        callback,
        callback,
        CallbackType::Main,
        CallbackType::Null,
    );

    opt_identifier_def!(
        MainNonOpt,
        id,
        para,
    );

    impl Name for MainNonOpt {
        fn name(&self) -> &str {
            &self.name
        }

        fn prefix(&self) -> &str {
            ""
        }

        fn set_name(&mut self, name_para: &str) {
            self.name = name_para.to_owned()
        }

        fn set_prefix(&mut self, _: &str) {
            
        }

        fn match_name(&self, _: &str) -> bool {
            true
        }

        fn match_prefix(&self, prefix_para: &str) -> bool {
            self.prefix() == prefix_para
        }
    }

    impl Optional for MainNonOpt {
        fn optional(&self) -> bool {
            self.optional
        }

        fn set_optional(&mut self, _: bool) {
            
        }

        fn match_optional(&self, optional_para: bool) -> bool {
            self.optional() == optional_para
        }
    }

    opt_alias_def!( MainNonOpt );

    impl Index for MainNonOpt {
        fn index(&self) -> &NonOptIndex {
            &self.index
        }

        /// Can not change the index of [`Main`]
        fn set_index(&mut self, _: NonOptIndex) {
            
        }

        fn match_index(&self, _: i64, _: i64) -> bool {
            true
        }
    }

    /// Pos using value hold the return value of callback
    impl Value for MainNonOpt {
        fn value(&self) -> &OptValue {
            &self.value
        }

        fn default_value(&self) -> &OptValue {
            &self.default_value
        }

        fn set_value(&mut self, value_para: OptValue) {
            self.value = value_para;
        }

        // ignore set default value operate
        fn set_default_value(&mut self, _: OptValue) {

        }

        fn parse_value(&self, _: &str) -> Result<OptValue> {
            return Ok(OptValue::from_bool(true));
        }

        fn has_value(&self) -> bool {
            self.value().is_bool()
        }

        fn reset_value(&mut self) {
            self.set_value(self.default_value().clone());
        }
    }

    /// Default [`Utils`] implementation for [`MainNonOpt`]
    #[derive(Debug)]
    pub struct MainUtils;

    impl MainUtils {
        pub fn new() -> Self {
            Self {}
        }
    }

    impl Utils for MainUtils {
        fn type_name(&self) -> &str {
            current_type()
        }

        fn is_support_deactivate_style(&self) -> bool {
            false
        }

        /// Create an [`MainNonOpt`] using option information [`CreateInfo`].
        /// 
        /// ```no_run
        /// use getopt_rs::utils::{Utils, CreateInfo};
        /// use getopt_rs::nonopt::main::*;
        /// use getopt_rs::id::*;
        /// 
        /// let utils = MainUtils::new();
        /// let ci = CreateInfo::parse("name=main", &vec![]).unwrap();
        /// let _non_opt = utils.create(Identifier::new(1), &ci);
        /// ```
        fn create(&self, id: IIdentifier, ci: &CreateInfo) -> Result<Box<dyn Opt>> {
            if ci.is_deactivate_style() {
                if ! self.is_support_deactivate_style() {
                    return Err(Error::UtilsNotSupportDeactivateStyle(ci.get_name().to_owned()));
                }
            }
            if ci.get_type_name() != self.type_name() {
                return Err(Error::UtilsNotSupportTypeName(self.type_name().to_owned(), ci.get_type_name().to_owned()))
            }
            
            assert_eq!(ci.get_type_name(), self.type_name());

            let opt = Box::new(MainNonOpt::new(
                id,
                ci.get_name().to_owned()
            ));

            Ok(opt)
        }

        fn gen_info(&self, opt: &dyn Opt) -> Box<dyn Info> {
            Box::new(OptionInfo::new(opt.id()))
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::opt::*;
    use crate::id::Identifier as IIdentifier;

    #[test]
    fn make_opt_type_pos_work() {
        let pos_utils = pos::PosUtils::new();
        
        assert_eq!(pos_utils.type_name(), pos::current_type());
        assert_eq!(pos_utils.is_support_deactivate_style(), false);
        
        let ci = CreateInfo::parse("nonopt=pos@2", &vec![]).unwrap();
        let mut nonopt = pos_utils.create(IIdentifier::new(1), &ci).unwrap();

        assert_eq!(nonopt.type_name(), "pos");
        assert_eq!(nonopt.is_deactivate_style(), false);
        assert_eq!(nonopt.is_style(Style::Pos), true);
        assert_eq!(nonopt.check().is_err(), false);

        assert_eq!(nonopt.id().get(), 1);
        nonopt.set_id(IIdentifier::new(42));
        assert_eq!(nonopt.id().get(), 42);

        assert_eq!(nonopt.callback_type(), CallbackType::Null);
        assert_eq!(nonopt.is_need_invoke(), false);
        nonopt.set_need_invoke(true);
        assert_eq!(nonopt.callback_type(), CallbackType::Index);
        assert_eq!(nonopt.is_need_invoke(), true);

        nonopt.add_alias("-", "c");
        assert_eq!(nonopt.alias(), None);
        assert_eq!(nonopt.match_alias("-", "c"), false);
        assert_eq!(nonopt.rem_alias("-", "c"), false);
        assert_eq!(nonopt.alias(), None);

        assert_eq!(nonopt.index(), &NonOptIndex::Forward(2));
        assert_eq!(nonopt.match_index(6, 2), true);
        nonopt.set_index(NonOptIndex::Forward(3));
        assert_eq!(nonopt.match_index(6, 3), true);

        assert_eq!(nonopt.name(), "nonopt");
        assert_eq!(nonopt.prefix(), "");
        assert_eq!(nonopt.match_name("opt"), true);
        assert_eq!(nonopt.match_name("opv"), true);
        assert_eq!(nonopt.match_prefix("--"), false);
        assert_eq!(nonopt.match_prefix("-"), false);
        nonopt.set_name("count");
        nonopt.set_prefix("+");
        assert_eq!(nonopt.match_name("count"), true);
        assert_eq!(nonopt.match_name("opt"), true);
        assert_eq!(nonopt.match_prefix("+"), false);
        assert_eq!(nonopt.match_prefix("--"), false);

        assert_eq!(nonopt.optional(), true);
        assert_eq!(nonopt.match_optional(true), true);
        nonopt.set_optional(false);
        assert_eq!(nonopt.optional(), false);
        assert_eq!(nonopt.match_optional(true), false);

        assert_eq!(nonopt.value().is_null(), true);
        assert_eq!(nonopt.default_value().is_null(), true);
        assert_eq!(nonopt.has_value(), false);
        nonopt.set_value(nonopt.parse_value("").unwrap());
        assert_eq!(nonopt.value().as_bool(), Some(&true));
        nonopt.set_default_value(OptValue::from_bool(false));
        assert_eq!(nonopt.default_value().as_bool(), None);
        nonopt.reset_value();
        assert_eq!(nonopt.value().as_bool(), None);

        assert_eq!(nonopt.as_ref().as_any().is::<pos::PosNonOpt>(), true);
    }

    #[test]
    fn make_opt_type_cmd_work() {
        let cmd_utils = cmd::CmdUtils::new();
        
        assert_eq!(cmd_utils.type_name(), cmd::current_type());
        assert_eq!(cmd_utils.is_support_deactivate_style(), false);
        
        let ci = CreateInfo::parse("nonopt=cmd", &vec![]).unwrap();
        let mut nonopt = cmd_utils.create(IIdentifier::new(1), &ci).unwrap();

        assert_eq!(nonopt.type_name(), "cmd");
        assert_eq!(nonopt.is_deactivate_style(), false);
        assert_eq!(nonopt.is_style(Style::Cmd), true);
        assert_eq!(nonopt.check().is_err(), true);

        assert_eq!(nonopt.id().get(), 1);
        nonopt.set_id(IIdentifier::new(42));
        assert_eq!(nonopt.id().get(), 42);

        assert_eq!(nonopt.callback_type(), CallbackType::Null);
        assert_eq!(nonopt.is_need_invoke(), false);
        nonopt.set_need_invoke(true);
        assert_eq!(nonopt.callback_type(), CallbackType::Main);
        assert_eq!(nonopt.is_need_invoke(), true);

        nonopt.add_alias("-", "c");
        assert_eq!(nonopt.alias(), None);
        assert_eq!(nonopt.match_alias("-", "c"), false);
        assert_eq!(nonopt.rem_alias("-", "c"), false);
        assert_eq!(nonopt.alias(), None);

        assert_eq!(nonopt.index(), &NonOptIndex::Forward(1));
        assert_eq!(nonopt.match_index(6, 1), true);
        nonopt.set_index(NonOptIndex::Forward(3));
        assert_eq!(nonopt.match_index(6, 3), false);

        assert_eq!(nonopt.name(), "nonopt");
        assert_eq!(nonopt.prefix(), "");
        assert_eq!(nonopt.match_name("nonopt"), true);
        assert_eq!(nonopt.match_name("opv"), false);
        assert_eq!(nonopt.match_prefix("--"), false);
        assert_eq!(nonopt.match_prefix("-"), false);
        nonopt.set_name("count");
        nonopt.set_prefix("+");
        assert_eq!(nonopt.match_name("count"), true);
        assert_eq!(nonopt.match_name("opt"), false);
        assert_eq!(nonopt.match_prefix("+"), false);
        assert_eq!(nonopt.match_prefix("--"), false);

        assert_eq!(nonopt.optional(), false);
        assert_eq!(nonopt.match_optional(true), false);
        nonopt.set_optional(false);
        assert_eq!(nonopt.optional(), false);
        assert_eq!(nonopt.match_optional(true), false);

        assert_eq!(nonopt.value().is_null(), true);
        assert_eq!(nonopt.default_value().is_null(), true);
        assert_eq!(nonopt.has_value(), false);
        nonopt.set_value(nonopt.parse_value("").unwrap());
        assert_eq!(nonopt.value().as_bool(), Some(&true));
        nonopt.set_default_value(OptValue::from_bool(false));
        assert_eq!(nonopt.default_value().as_bool(), None);
        nonopt.reset_value();
        assert_eq!(nonopt.value().as_bool(), None);

        assert_eq!(nonopt.as_ref().as_any().is::<cmd::CmdNonOpt>(), true);
    }

    #[test]
    fn make_opt_type_main_work() {
        let cmd_utils = main::MainUtils::new();
        
        assert_eq!(cmd_utils.type_name(), main::current_type());
        assert_eq!(cmd_utils.is_support_deactivate_style(), false);
        
        let ci = CreateInfo::parse("nonopt=main", &vec![]).unwrap();
        let mut nonopt = cmd_utils.create(IIdentifier::new(1), &ci).unwrap();

        assert_eq!(nonopt.type_name(), "main");
        assert_eq!(nonopt.is_deactivate_style(), false);
        assert_eq!(nonopt.is_style(Style::Main), true);
        assert_eq!(nonopt.check().is_err(), false);

        assert_eq!(nonopt.id().get(), 1);
        nonopt.set_id(IIdentifier::new(42));
        assert_eq!(nonopt.id().get(), 42);

        assert_eq!(nonopt.callback_type(), CallbackType::Null);
        assert_eq!(nonopt.is_need_invoke(), false);
        nonopt.set_need_invoke(true);
        assert_eq!(nonopt.callback_type(), CallbackType::Main);
        assert_eq!(nonopt.is_need_invoke(), true);

        nonopt.add_alias("-", "c");
        assert_eq!(nonopt.alias(), None);
        assert_eq!(nonopt.match_alias("-", "c"), false);
        assert_eq!(nonopt.rem_alias("-", "c"), false);
        assert_eq!(nonopt.alias(), None);

        assert_eq!(nonopt.index(), &NonOptIndex::Null);
        assert_eq!(nonopt.match_index(6, 1), true);
        nonopt.set_index(NonOptIndex::Forward(3));
        assert_eq!(nonopt.match_index(6, 4), true);

        assert_eq!(nonopt.name(), "nonopt");
        assert_eq!(nonopt.prefix(), "");
        assert_eq!(nonopt.match_name("nonopt"), true);
        assert_eq!(nonopt.match_name("opv"), true);
        assert_eq!(nonopt.match_prefix("--"), false);
        assert_eq!(nonopt.match_prefix("-"), false);
        nonopt.set_name("count");
        nonopt.set_prefix("+");
        assert_eq!(nonopt.match_name("count"), true);
        assert_eq!(nonopt.match_name("opt"), true);
        assert_eq!(nonopt.match_prefix("+"), false);
        assert_eq!(nonopt.match_prefix("--"), false);

        assert_eq!(nonopt.optional(), true);
        assert_eq!(nonopt.match_optional(true), true);
        nonopt.set_optional(false);
        assert_eq!(nonopt.optional(), true);
        assert_eq!(nonopt.match_optional(true), true);

        assert_eq!(nonopt.value().is_null(), true);
        assert_eq!(nonopt.default_value().is_null(), true);
        assert_eq!(nonopt.has_value(), false);
        nonopt.set_value(nonopt.parse_value("").unwrap());
        assert_eq!(nonopt.value().as_bool(), Some(&true));
        nonopt.set_default_value(OptValue::from_bool(false));
        assert_eq!(nonopt.default_value().as_bool(), None);
        nonopt.reset_value();
        assert_eq!(nonopt.value().as_bool(), None);

        assert_eq!(nonopt.as_ref().as_any().is::<main::MainNonOpt>(), true);
    }
}
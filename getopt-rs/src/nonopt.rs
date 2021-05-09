
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
                value: OptValue::null(),
                index,
                callback: CallbackType::Null,
                default_value: OptValue::null(),
            }
        }
    }

    opt_def!(PosNonOpt, Pos, NonOpt);

    opt_type_def!(
        PosNonOpt, 
        current_type(),
        false,
        false,
        style,
        Style::Pos
    );

    opt_callback_def!(
        PosNonOpt,
        callback,
        callback,
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

        fn create(&self, id: IIdentifier, ci: &CreateInfo) -> Result<Box<dyn Opt>> {
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
        pub fn new(id: IIdentifier, name: String, optional: bool, index: NonOptIndex) -> Self {
            Self {
                id,
                name,
                optional,
                value: OptValue::null(),
                index,
                callback: CallbackType::Null,
                default_value: OptValue::null(),
            }
        }
    }

    opt_def!(CmdNonOpt, Cmd, NonOpt);

    opt_type_def!(
        CmdNonOpt, 
        current_type(),
        false,
        false,
        style,
        Style::Cmd
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

    opt_optional_def!(
        CmdNonOpt,
        optional,
        optional,
    );

    opt_alias_def!( CmdNonOpt );

    opt_index_def!(
        CmdNonOpt,
        index,
        index,
    );

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

        fn create(&self, id: IIdentifier, ci: &CreateInfo) -> Result<Box<dyn Opt>> {
            let opt = Box::new(CmdNonOpt::new(
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

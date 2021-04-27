
use std::fmt::Debug;

use crate::opt::Opt;
use crate::opt::OptValue;
use crate::opt::OptIndex;
use crate::error::Error;
use crate::error::Result;
use crate::proc::Publisher;
use crate::proc::Proc;
use crate::id::Identifier;
use crate::utils::Utils;
use crate::utils::CreateInfo;
use crate::utils::FilterInfo;

pub trait Set: Debug {
    fn add_utils(&mut self, utils: Box<dyn Utils>) -> Result<bool>;

    fn rem_utils(&mut self, type_name: &str) -> Result<bool>;

    fn get_utlis(&self, type_name: &str);

    fn add_opt(&mut self, opt: &str) -> Commit;

    fn add_opt_ci(&mut self, ci: &CreateInfo) -> Result<Identifier>;

    fn add_opt_raw(&mut self, opt: Box<dyn Opt>);

    fn get_opt_i(&self, id: Identifier) -> Option<& dyn Opt>;

    fn get_opt_mut_i(&self, id: Identifier) -> Option<&mut dyn Opt>;

    fn get_opt(&self, name: &str) -> Filter;

    fn get_opt_mut(&self, name: &str) -> FilterMut;

    fn get_commit(&mut self) -> Commit;

    fn get_filter(&self) -> Filter;

    fn get_filter_mut(&mut self) -> FilterMut;

    fn subscribe_from(&self, publisher: &mut dyn Publisher<Box<dyn Proc>>);
}

#[derive(Debug)]
pub struct Commit<'a> {
    ref_set: &'a mut dyn Set,

    create_info: CreateInfo,
}

impl<'a> Commit<'a> {
    pub fn new(set: &'a mut dyn Set, ci: CreateInfo) -> Self {
        Self {
            ref_set: set,
            create_info: ci,
        }
    }

    pub fn set_deactivate_style(&mut self, deactivate: bool) {
        self.create_info.set_deactivate_style(deactivate);
    }

    pub fn set_optional(&mut self, optional: bool) {
        self.create_info.set_optional(optional);
    }

    pub fn set_type_name(&mut self, opt_type: &str) {
        self.create_info.set_type_name(opt_type);
    }

    pub fn set_name(&mut self, opt_name: &str) {
        self.create_info.set_name(opt_name);
    }

    pub fn set_prefix(&mut self, prefix: &str) {
        self.create_info.set_prefix(prefix);
    }

    pub fn set_index(&mut self, index: OptIndex) {
        self.create_info.set_index(index);
    }

    pub fn add_alias(&mut self, alias: &str) {
        self.create_info.add_alias(alias);
    }

    pub fn rem_alias(&mut self, s: &str) {
        self.create_info.rem_alias(s);
    }

    pub fn clr_alias(&mut self) {
        self.create_info.clr_alias();
    }

    pub fn set_deafult_value(&mut self, value: OptValue) {
        self.create_info.set_deafult_value(value);
    }

    pub fn commit(&mut self) -> Result<Identifier> {
        self.create_info.check()?;
        self.ref_set.add_opt_ci(&self.create_info)
    }
}

#[derive(Debug)]
pub struct Filter<'a> {
    ref_set: &'a dyn Set,

    filter_info: FilterInfo,
}

impl<'a> Filter<'a> {
    pub fn new(set: &'a mut dyn Set, fi: FilterInfo) -> Self {
        Self {
            ref_set: set,
            filter_info: fi,
        }
    }

    pub fn set_deactivate_style(&mut self, deactivate: bool) {
        self.filter_info.set_deactivate_style(deactivate);
    }

    pub fn set_optional(&mut self, optional: bool) {
        self.filter_info.set_optional(optional);
    }

    pub fn set_type_name(&mut self, opt_type: &str) {
        self.filter_info.set_type_name(opt_type);
    }

    pub fn set_name(&mut self, opt_name: &str) {
        self.filter_info.set_name(opt_name);
    }

    pub fn set_prefix(&mut self, prefix: &str) {
        self.filter_info.set_prefix(prefix);
    }

    pub fn set_index(&mut self, index: OptIndex) {
        self.filter_info.set_index(index);
    }
}

#[derive(Debug)]
pub struct FilterMut<'a> {
    ref_set: &'a mut dyn Set,
}

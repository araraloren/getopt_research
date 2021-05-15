
use std::fmt::Debug;
use std::slice::Iter;
use std::slice::IterMut;
use std::ops::Index;
use std::ops::IndexMut;
use std::collections::HashMap;

use crate::opt::Opt;
use crate::opt::OptValue;
use crate::opt::NonOptIndex;
use crate::error::Error;
use crate::error::Result;
use crate::proc::Publisher;
use crate::proc::Subscriber;
use crate::proc::Proc;
use crate::id::Identifier;
use crate::utils::Utils;
use crate::utils::CreateInfo;
use crate::utils::FilterInfo;

pub trait Set: Debug + Subscriber + Index<Identifier, Output=dyn Opt> + IndexMut<Identifier> {
    fn add_utils(&mut self, utils: Box<dyn Utils>) -> Result<bool>;

    fn rem_utils(&mut self, type_name: &str) -> Result<bool>;

    fn get_utils(&self, type_name: &str) -> Option<& dyn Utils>;

    
    fn add_opt(&mut self, opt: &str) -> Result<Commit>;

    fn add_opt_ci(&mut self, ci: &CreateInfo) -> Result<Identifier>;

    fn add_opt_raw(&mut self, opt: Box<dyn Opt>) -> Result<Identifier>;

    
    fn get_opt(&self, id: Identifier) -> Option<& dyn Opt>;

    fn get_opt_mut(&mut self, id: Identifier) -> Option<&mut dyn Opt>;

    fn get_opt_i(&self, index: usize) -> Option<& dyn Opt>;

    fn get_opt_mut_i(&mut self, index: usize) -> Option<&mut dyn Opt>;

    fn len(&self) -> usize;


    fn filter(&self, opt: &str) -> Result<Filter>;

    fn filter_mut(&mut self, opt: &str) -> Result<FilterMut>;
    
    fn find(&self, fi: &FilterInfo) -> Option<&dyn Opt>;

    fn find_all(&self, fi: &FilterInfo) -> Vec<Option<&dyn Opt>>;

    fn find_mut(&mut self, fi: &FilterInfo) -> Option<&mut dyn Opt>;

    fn find_all_mut(&mut self, fi: &FilterInfo) -> Vec<Option<&mut dyn Opt>>;

    
    fn iter(&self) -> Iter<Box<dyn Opt>>;

    fn iter_mut(&mut self) -> IterMut<Box<dyn Opt>>;

    fn get_all_prefixs(&self) -> Vec<String>;

    fn check(&self) -> Result<bool>;

    fn reset(&mut self);
}

#[derive(Debug)]
pub struct DefaultSet {
    opts: Vec<Box<dyn Opt>>,

    utils: HashMap<String, Box<dyn Utils>>,
}

impl DefaultSet {
    pub fn new() -> Self {
        Self {
            opts: vec![],
            utils: HashMap::new(),
        }
    }
}

impl Subscriber for DefaultSet {
    fn subscribe_from(&self, publisher: &mut dyn Publisher<Box<dyn Proc>>) {
        for opt in &self.opts {
            publisher.reg_subscriber(
                self.get_utils(opt.type_name())
                    .unwrap()
                    .gen_info(opt.as_ref()),
            );
        }
    }
}

impl Set for DefaultSet {
    fn add_utils(&mut self, utils: Box<dyn Utils>) -> Result<bool> {
        if ! self.utils.contains_key(utils.type_name()) {
            self.utils.insert(utils.type_name().to_owned(), utils);
            Ok(true)
        }
        else {
            Err(Error::DuplicateOptionType(utils.type_name().to_owned()))
        }
    }

    fn rem_utils(&mut self, type_name: &str) -> Result<bool> {
        if self.utils.contains_key(type_name) {
            self.utils.remove(type_name);
            Ok(true)
        }
        else {
            Err(Error::InvalidOptionType(type_name.to_owned()))
        }
    }

    fn get_utils(&self, type_name: &str) -> Option<& dyn Utils> {
        match self.utils.get(type_name) {
            Some(util) => {
                Some(util.as_ref())
            }
            None => { 
                None 
            }
        }
    }

    
    fn add_opt(&mut self, opt: &str) -> Result<Commit> {
        let ci = CreateInfo::parse(opt)?;
        Ok(Commit::new(self, ci))
    }

    fn add_opt_ci(&mut self, ci: &CreateInfo) -> Result<Identifier> {
        let id = Identifier::new(self.opts.len() as u64);

        match self.get_utils(ci.get_type_name()) {
            Some(util) => {
                let opt = util.create(id, &ci)?;

                self.opts.push(opt);
                Ok(id)
            }
            None => Err(Error::InvalidOptionType(ci.get_type_name().to_owned()))
        }
    }

    fn add_opt_raw(&mut self, opt: Box<dyn Opt>) -> Result<Identifier> {
        let mut opt = opt;
        let id = Identifier::new(self.opts.len() as u64);

        opt.set_id(id); // reset the id
        self.opts.push(opt);
        Ok(id)
    }

    
    fn get_opt(&self, id: Identifier) -> Option<& dyn Opt> {
        match self.opts.get(id.get() as usize) {
            Some(opt) => Some(opt.as_ref()),
            None => None,
        }
    }

    fn get_opt_mut(&mut self, id: Identifier) -> Option<&mut dyn Opt> {
        match self.opts.get_mut(id.get() as usize) {
            Some(opt) => Some(opt.as_mut()),
            None => None,
        }
    }

    fn get_opt_i(&self, index: usize) -> Option<& dyn Opt> {
        match self.opts.get(index) {
            Some(opt) => Some(opt.as_ref()),
            None => None,
        }
    }

    fn get_opt_mut_i(&mut self, index: usize) -> Option<&mut dyn Opt> {
        match self.opts.get_mut(index) {
            Some(opt) => Some(opt.as_mut()),
            None => None,
        }
    }

    fn len(&self) -> usize {
        self.opts.len()
    }

    fn filter(&self, opt: &str) -> Result<Filter> {
        let fi = FilterInfo::parse(opt)?;
        Ok(Filter::new(self, fi))
    }

    fn filter_mut(&mut self, opt: &str) -> Result<FilterMut> {
        let fi = FilterInfo::parse(opt)?;
        Ok(FilterMut::new(self, fi))
    }
    
    fn find(&self, fi: &FilterInfo) -> Option<&dyn Opt> {
        for opt in self.opts.iter() {
            if fi.match_opt(opt.as_ref()) {
                return Some(opt.as_ref());
            }
        }
        None
    }

    fn find_all(&self, fi: &FilterInfo) -> Vec<Option<&dyn Opt>> {
        let mut opts = vec![];

        for opt in self.opts.iter() {
            if fi.match_opt(opt.as_ref()) {
                opts.push(Some(opt.as_ref()))
            }
        }
        opts
    }

    fn find_mut(&mut self, fi: &FilterInfo) -> Option<&mut dyn Opt> {
        for opt in self.opts.iter_mut() {
            if fi.match_opt(opt.as_ref()) {
                return Some(opt.as_mut());
            }
        }
        None
    }

    fn find_all_mut(&mut self, fi: &FilterInfo) -> Vec<Option<&mut dyn Opt>> {
        let mut opts: Vec<Option<&mut dyn Opt>> = vec![];

        for opt in self.opts.iter_mut() {
            if fi.match_opt(opt.as_ref()) {
                opts.push(Some(opt.as_mut()))
            }
        }
        opts
    }
    
    fn iter(&self) -> Iter<Box<dyn Opt>> {
        self.opts.iter()
    }

    fn iter_mut(&mut self) -> IterMut<Box<dyn Opt>> {
        self.opts.iter_mut()
    }

    fn get_all_prefixs(&self) -> Vec<String> {
        let mut ret: Vec<String> = vec![];

        for opt in &self.opts {
            if !ret.iter().find(|s|s.as_str() == opt.prefix()).is_some() {
                ret.push(opt.prefix().to_owned());
            }

            if let Some(alias) = opt.alias() {
                for alias in alias.iter() {
                    if !ret.iter().find(|s|s.as_str() == alias.0).is_some() {
                        ret.push(alias.0.to_owned());
                    }
                }
            }
        }

        ret
    }

    fn check(&self) -> Result<bool> {
        for opt in &self.opts {
            opt.check()?;
        }
        Ok(true)
    }

    fn reset(&mut self) {
        for opt in self.opts.iter_mut() {
            opt.reset_value();
        }
    }
}

impl Index<Identifier> for DefaultSet {
    type Output = dyn Opt;

    fn index(&self, index: Identifier) -> &Self::Output {
        self.opts[index.get() as usize].as_ref()
    }
}

impl IndexMut<Identifier> for DefaultSet {
    fn index_mut(&mut self, index: Identifier) -> &mut Self::Output {
        self.opts[index.get() as usize].as_mut()
    }
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

    pub fn set_index(&mut self, index: NonOptIndex) {
        self.create_info.set_index(index);
    }

    pub fn add_alias(&mut self, prefix: &str, name: &str) {
        self.create_info.add_alias(prefix, name);
    }

    pub fn rem_alias(&mut self, prefix: &str, name: &str) {
        self.create_info.rem_alias(prefix, name);
    }

    pub fn clr_alias(&mut self) {
        self.create_info.clr_alias();
    }

    pub fn set_deafult_value(&mut self, value: OptValue) {
        self.create_info.set_deafult_value(value);
    }

    pub fn commit(&mut self) -> Result<Identifier> {
        self.ref_set.add_opt_ci(&self.create_info)
    }
}

#[derive(Debug)]
pub struct Filter<'a> {
    ref_set: &'a dyn Set,

    filter_info: FilterInfo,
}

impl<'a> Filter<'a> {
    pub fn new(set: &'a dyn Set, fi: FilterInfo) -> Self {
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

    pub fn set_index(&mut self, index: NonOptIndex) {
        self.filter_info.set_index(index);
    }

    pub fn find(&self) -> Option<&dyn Opt> {
        self.ref_set.find(&self.filter_info)
    }

    pub fn find_all(&self) -> Vec<Option<&dyn Opt>> {
        self.ref_set.find_all(&self.filter_info)
    }
}

#[derive(Debug)]
pub struct FilterMut<'a> {
    ref_set: &'a mut dyn Set,

    filter_info: FilterInfo,
}

impl<'a> FilterMut<'a> {
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

    pub fn set_index(&mut self, index: NonOptIndex) {
        self.filter_info.set_index(index);
    }

    pub fn find(&mut self) -> Option<&mut dyn Opt> {
        self.ref_set.find_mut(&self.filter_info)
    }

    pub fn find_all(&mut self) -> Vec<Option<&mut dyn Opt>> {
        self.ref_set.find_all_mut(&self.filter_info)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::opt::*;
    use crate::nonopt::*;
    use crate::id::Identifier as IIdentifier;

    #[test]
    fn make_sure_set_work() {
        let mut set = DefaultSet::new();

        assert_eq!(set.add_utils(Box::new(str::StrUtils::new())).is_err(), false);
        assert_eq!(set.add_utils(Box::new(int::IntUtils::new())).is_err(), false);
        assert_eq!(set.add_utils(Box::new(uint::UintUtils::new())).is_err(), false);
        assert_eq!(set.add_utils(Box::new(flt::FltUtils::new())).is_err(), false);
        assert_eq!(set.add_utils(Box::new(bool::BoolUtils::new())).is_err(), false);
        assert_eq!(set.add_utils(Box::new(pos::PosUtils::new())).is_err(), false);
        assert_eq!(set.add_utils(Box::new(cmd::CmdUtils::new())).is_err(), false);
        assert_eq!(set.add_utils(Box::new(main::MainUtils::new())).is_err(), false);
        assert_eq!(set.add_utils(Box::new(main::MainUtils::new())).is_err(), true);

        assert_eq!(set.get_utils("str").unwrap().type_name(), "str");
        assert_eq!(set.get_utils("int").unwrap().type_name(), "int");
        assert_eq!(set.get_utils("bool").unwrap().type_name(), "bool");
        assert_eq!(set.get_utils("cmd").unwrap().type_name(), "cmd");
        assert_eq!(set.get_utils("main").unwrap().type_name(), "main");
        assert_eq!(set.get_utils("pos").unwrap().type_name(), "pos");
        
        assert_eq!(set.rem_utils("uint").is_err(), false);
        assert_eq!(set.get_utils("uint").is_none(), true);
        assert_eq!(set.get_utils("flt").is_none(), false);
        assert_eq!(set.get_utils("array").is_none(), true);
        assert_eq!(set.rem_utils("uint").is_err(), true);

        if let Ok(mut commit) = set.add_opt("-|P=bool") {
            assert!(commit.commit().is_ok());
        }
        if let Err(_) = set.add_opt("-/L=bool") {
            assert!(true);
        }
        if let Ok(ci) = CreateInfo::parse("-|H=bool") {
            assert!(set.add_opt_ci(&ci).is_ok());
        }
        let int_utils = int::IntUtils::new(); {
            let fake_id = IIdentifier::new(11);

            if let Ok(ci) = CreateInfo::parse("-|O=int") {
                if let Ok(optimz) = int_utils.create(fake_id, &ci) {
                    assert!(set.add_opt_raw(optimz).is_ok());
                }
            }
            if let Ok(ci) = CreateInfo::parse("-|maxdepth=int") {
                if let Ok(optimz) = int_utils.create(fake_id, &ci) {
                    assert!(set.add_opt_raw(optimz).is_ok());
                }
            }
            if let Ok(ci) = CreateInfo::parse("-|mindepth=int") {
                if let Ok(optimz) = int_utils.create(fake_id, &ci) {
                    assert!(set.add_opt_raw(optimz).is_ok());
                }
            }
        }
        let mut id = IIdentifier::new(0);

        if let Ok(mut commit) = set.add_opt("-|d=bool") {
            commit.add_alias("--", "depth");
            let ret_id = commit.commit();
            assert!(ret_id.is_ok());
            id = ret_id.unwrap();
        }
        if let Some(depth) = set.get_opt(id.clone()) {
            assert_eq!(depth.name(), "d");
        }
        if let Some(depth) = set.get_opt_mut(id.clone()) {
            assert_eq!(depth.name(), "d");
        }
        if let Some(p) = set.get_opt_i(0) {
            assert_eq!(p.name(), "P");
        }
        if let Some(p) = set.get_opt_mut_i(0) {
            assert_eq!(p.name(), "P");
        }
        assert_eq!(set.len(), 6);

        if let Ok(mut commit) = set.add_opt("-|help=bool") {
            commit.add_alias("--", "help");
            assert!(commit.commit().is_ok());
        }
        if let Ok(mut commit) = set.add_opt("-|mount=bool") {
            assert!(commit.commit().is_ok());
        }
        if let Ok(mut commit) = set.add_opt("-|version=bool") {
            commit.add_alias("--", "version");
            assert!(commit.commit().is_ok());
        }
        if let Ok(mut commit) = set.add_opt("-|amin=int") {
            assert!(commit.commit().is_ok());
        }
        if let Ok(mut commit) = set.add_opt("-|atime=int") {
            assert!(commit.commit().is_ok());
        }
        if let Ok(mut commit) = set.add_opt("-|cmin=int") {
            assert!(commit.commit().is_ok());
        }
        if let Ok(mut commit) = set.add_opt("-|ctime=int") {
            assert!(commit.commit().is_ok());
        }
        if let Ok(mut commit) = set.add_opt("-|fstype=str") {
            assert!(commit.commit().is_ok());
        }
        if let Ok(mut commit) = set.add_opt("-|iname=str") {
            assert!(commit.commit().is_ok());
        }
        if let Ok(mut commit) = set.add_opt("-|name=str") {
            assert!(commit.commit().is_ok());
        }
        if let Ok(mut commit) = set.add_opt("localtion=pos@1") {
            assert!(commit.commit().is_ok());
        }
        if let Ok(mut commit) = set.add_opt("find=main") {
            assert!(commit.commit().is_ok());
        }
        if let Ok(filter) = set.filter("-|help=bool") {
            assert!(filter.find().is_some());
        }
        if let Ok(filter) = set.filter("-|help=str") {
            assert!(filter.find().is_none());
        }
        
        let mut fi = FilterInfo::new();

        fi.set_type_name("str");
        let vec = set.find_all(&fi);

        assert_eq!(vec.len(), 3);
    }
}

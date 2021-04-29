
use std::fmt::Debug;
use std::slice::Iter;
use std::slice::IterMut;
use std::slice::SliceIndex;
use std::ops::Index;
use std::ops::IndexMut;
use std::collections::HashMap;

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

    
    fn subscribe_from(&self, publisher: &mut dyn Publisher<Box<dyn Proc>>);

    
    fn iter(&self) -> Iter<Box<dyn Opt>>;

    fn iter_mut(&mut self) -> IterMut<Box<dyn Opt>>;
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

impl Set for DefaultSet {
    fn add_utils(&mut self, utils: Box<dyn Utils>) -> Result<bool> {
        if ! self.utils.contains_key(utils.type_name()) {
            self.utils.insert(String::from(utils.type_name()), utils);
            Ok(true)
        }
        else {
            Err(Error::DuplicateOptionType(String::from(utils.type_name())))
        }
    }

    fn rem_utils(&mut self, type_name: &str) -> Result<bool> {
        if self.utils.contains_key(type_name) {
            self.utils.remove(type_name);
            Ok(true)
        }
        else {
            Err(Error::InvalidOptionType(String::from(type_name)))
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
                let opt = util.create(id, &ci);

                self.opts.push(opt);
                Ok(id)
            }
            None => Err(Error::InvalidOptionType(String::from(ci.get_type_name())))
        }
    }

    fn add_opt_raw(&mut self, opt: Box<dyn Opt>) -> Result<Identifier> {
        let id = Identifier::new(self.opts.len() as u64);
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
            if opt.type_name() == fi.get_type_name() 
            && opt.match_name(fi.get_name())
            && opt.match_prefix(fi.get_prefix())
            && opt.index() == *fi.get_index() {
                return Some(opt.as_ref());
            }
        }
        None
    }

    fn find_all(&self, fi: &FilterInfo) -> Vec<Option<&dyn Opt>> {
        let mut opts = vec![];

        for opt in self.opts.iter() {
            if opt.type_name() == fi.get_type_name() 
            && opt.match_name(fi.get_name())
            && opt.match_prefix(fi.get_prefix())
            && opt.index() == *fi.get_index() {
                opts.push(Some(opt.as_ref()))
            }
        }
        opts
    }

    fn find_mut(&mut self, fi: &FilterInfo) -> Option<&mut dyn Opt> {
        for opt in self.opts.iter_mut() {
            if opt.type_name() == fi.get_type_name() 
            && opt.match_name(fi.get_name())
            && opt.match_prefix(fi.get_prefix())
            && opt.index() == *fi.get_index() {
                return Some(opt.as_mut());
            }
        }
        None
    }

    fn find_all_mut(&mut self, fi: &FilterInfo) -> Vec<Option<&mut dyn Opt>> {
        let mut opts: Vec<Option<&mut dyn Opt>> = vec![];

        for opt in self.opts.iter_mut() {
            if opt.type_name() == fi.get_type_name() 
            && opt.match_name(fi.get_name())
            && opt.match_prefix(fi.get_prefix())
            && opt.index() == *fi.get_index() {
                opts.push(Some(opt.as_mut()))
            }
        }
        opts
    }

    
    fn subscribe_from(&self, publisher: &mut dyn Publisher<Box<dyn Proc>>) {
        for opt in &self.opts {
            publisher.subscribe(
                self.get_utils(opt.type_name())
                    .unwrap()
                    .gen_info(opt.as_ref()),
            );
        }
    }

    
    fn iter(&self) -> Iter<Box<dyn Opt>> {
        self.opts.iter()
    }

    fn iter_mut(&mut self) -> IterMut<Box<dyn Opt>> {
        self.opts.iter_mut()
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

    pub fn set_index(&mut self, index: OptIndex) {
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

    pub fn set_index(&mut self, index: OptIndex) {
        self.filter_info.set_index(index);
    }

    pub fn find(&mut self) -> Option<&mut dyn Opt> {
        self.ref_set.find_mut(&self.filter_info)
    }

    pub fn find_all(&mut self) -> Vec<Option<&mut dyn Opt>> {
        self.ref_set.find_all_mut(&self.filter_info)
    }
}

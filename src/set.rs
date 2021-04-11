
use crate::opt::Opt;

use crate::err::Error;

use crate::utils::Utils;
use crate::utils::CreatorInfo;

use std::collections::HashMap;

#[derive(Debug)]
pub struct Set {
    opts: Vec<Box<dyn Opt>>,

    utils: HashMap<String, Box<dyn Utils>>,
}

impl Set {
    pub fn new() -> Self {
        Self {
            opts: vec![],
            utils: HashMap::new(),
        }
    }

    pub fn add_utils(&mut self, s: &'static str, utils: Box<dyn Utils>) -> &mut Self {
        self.utils.insert(String::from(s), utils);
        self
    }

    pub fn get_utils(&self, s: &'static str) -> Option<&Box<dyn Utils>> {
        self.utils.get(s)
    }

    pub fn add_opt(&mut self, id: u64, n: &'static str, opt: &'static str) -> Result<bool, Error> {
        match self.get_utils(n) {
            Some(util) => {
                let ci  = CreatorInfo::new(opt)?;
                let opt = util.create(id, &ci);
                self.opts.push(opt);
                Ok(true)
            }
            None => {
                Err(Error::InvalidOptionType(String::from(n)))
            }
        }
    }

    pub fn get_opt_by_id(&self, id: u64) -> Option<&dyn Opt> {
        for opt in &self.opts {
            if opt.opt_id() == id {
                return Some(opt.as_ref())
            }
        }
        None
    }
}

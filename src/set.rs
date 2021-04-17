
use crate::opt::Opt;
use crate::proc::Publisher;
use crate::proc::Proc;
use crate::err::Error;
use crate::utils::Utils;
use crate::utils::CreatorInfo;
use crate::id::IdGenerator;

use std::collections::HashMap;


#[derive(Debug)]
pub struct Set {
    opts: Vec<Box<dyn Opt>>,

    utils: HashMap<String, Box<dyn Utils>>,

    idgen: Box<dyn IdGenerator>,
}

impl Set {
    pub fn new(idgen: Box<dyn IdGenerator>) -> Self {
        Self {
            opts: vec![],
            utils: HashMap::new(),
            idgen,
        }
    }

    pub fn add_utils(&mut self, s: &str, utils: Box<dyn Utils>) -> &mut Self {
        self.utils.insert(String::from(s), utils);
        self
    }

    pub fn get_utils(&self, s: &str) -> Option<&Box<dyn Utils>> {
        self.utils.get(s)
    }

    pub fn add_opt(&mut self, n: &str, opt: &'static str) -> Result<u64, Error> {
        let id  = self.idgen.next_id();
        
        match self.get_utils(n) {
            Some(util) => {
                let ci  = CreatorInfo::new(opt)?;
                let opt = util.create(id, &ci);
                self.opts.push(opt);
                Ok(id)
            }
            None => {
                Err(Error::InvalidOptionType(String::from(n)))
            }
        }
    }

    pub fn get_opt(&self, id: u64) -> Option<&dyn Opt> {
        for opt in &self.opts {
            if opt.id() == id {
                return Some(opt.as_ref())
            }
        }
        None
    }

    pub fn get_opt_mut(&mut self, id: u64) -> Option<&mut dyn Opt> {
        for opt in &mut self.opts {
            if opt.id() == id {
                return Some(opt.as_mut())
            }
        }
        None
    }

    pub fn collect_prefix(&self) -> Vec<String> {
        let mut ret: Vec<String> = vec![];

        for opt in &self.opts {
            let prefix = String::from(opt.prefix());

            if ! ret.contains(&prefix) {
                ret.push(prefix);
            }
        }

        ret
    }

    pub fn subscribe_from(&self, publisher: &mut dyn Publisher<Proc>) {
        for opt in &self.opts {
            publisher.subscribe(
                self.get_utils(opt.type_name())
                    .unwrap()
                    .get_info(opt.as_ref())
            );
        }
    }

    pub fn len(&self) -> usize {
        self.opts.len()
    }
}


use crate::opt::Opt;

use crate::creator::Creator;

use std::collections::HashMap;

#[derive(Debug)]
pub struct Set {
    opts: Vec<Box<dyn Opt>>,

    creators: HashMap<String, Box<dyn Creator>>,
}

impl Set {
    pub fn new() -> Self {
        Self {
            opts: vec![],
            creators: HashMap::new(),
        }
    }

    pub fn add_creator(&mut self, s: &'static str, creator: Box<dyn Creator>) -> &mut Self {
        self.creators.insert(String::from(s), creator);
        self
    }

    pub fn get_creator(&self, s: &'static str) -> Option<&Box<dyn Creator>> {
        self.creators.get(s)
    }
}

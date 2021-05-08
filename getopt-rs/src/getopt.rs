
use crate::set::Set;
use crate::parser::Parser;
use crate::arg::Iterator;
use crate::error::Result;

#[derive(Debug)]
pub struct Getopt {
    parsers: Vec<Box<dyn Parser>>,

    arg_iter: Box<dyn Iterator>,
}

impl Getopt {
    pub fn new(iter: Box<dyn Iterator>) -> Self {
        Self {
            parsers: vec![],
            arg_iter: iter,
        }
    }

    pub fn initialized(&mut self, iter: &mut dyn std::iter::Iterator<Item=String>) -> &mut Self {
        self.arg_iter.set_args(iter);
        self
    }

    pub fn initialized_default(&mut self) -> &mut Self {
        self.initialized(&mut std::env::args())
    }

    pub fn app_subscriber(&mut self, set: Box<dyn Set>, mut parser: Box<dyn Parser>) -> &mut Self {
        parser.publish_to(set);
        self.parsers.push(parser);
        self
    }

    pub fn parse(&mut self) -> Result<bool> {
        Ok(true)
    }

    pub fn reset(&mut self) {
        self.parsers.iter_mut().for_each(|s|s.reset());
        self.arg_iter.reset();
    }
}

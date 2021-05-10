
use crate::set::Set;
use crate::parser::Parser;
use crate::parser::ReturnValue;
use crate::arg::IndexIterator;
use crate::error::Result;

use std::iter::Iterator;

#[derive(Debug)]
pub struct Getopt<'a, T> where T: IndexIterator {
    parser: Vec<Box<dyn Parser>>,

    preparser: Option<Box<dyn Parser>>,

    arg_iter: T,

    return_value: Vec<ReturnValue<'a>>,
}

impl<'a, T> Getopt<'a, T> where T: IndexIterator {
    pub fn new(iter: T) -> Self {
        Self {
            parser: vec![],
            preparser: None,
            arg_iter: iter,
            return_value: vec![],
        }
    }

    pub fn initialized(&mut self, iter: &mut dyn Iterator<Item=String>) -> &mut Self {
        self.arg_iter.set_args(iter);
        self
    }

    pub fn initialized_default(&mut self) -> &mut Self {
        self.initialized(&mut std::env::args())
    }

    pub fn set_preparser(&mut self, set: Box<dyn Set>, mut parser: Box<dyn Parser>) -> &mut Self {
        parser.publish_to(set);
        self.preparser = Some(parser);
        self
    }

    pub fn app_subscriber(&mut self, set: Box<dyn Set>, mut parser: Box<dyn Parser>) -> &mut Self {
        parser.publish_to(set);
        self.parser.push(parser);
        self
    }

    pub fn parse(&mut self) -> Result<bool> {
        Ok(true)
    }

    pub fn reset(&mut self) {
        self.parser.iter_mut().for_each(|s|s.reset());
        self.arg_iter.reset();
    }
}
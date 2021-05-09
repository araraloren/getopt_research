
use crate::error::Result;
use crate::error::Error;

use std::fmt::Debug;

pub trait Iterator : Debug {
    fn set_prefix(&mut self, prefix: Vec<String>);

    fn set_args(&mut self, args: &mut dyn std::iter::Iterator<Item = String>);

    fn reach_end(&self) -> bool;

    fn fill_current_and_next(&mut self);

    fn current(&self) -> &Option<String>;

    fn current_index(&self) -> usize;

    fn count(&self) -> usize;

    fn next(&self) -> &Option<String>;

    fn skip(&mut self);

    fn parse(&self) -> Result<Argument>;

    fn reset(&mut self);
}

#[derive(Debug)]
pub struct Argument {
    prefix: Option<String>,

    name: Option<String>,

    value: Option<String>,
}

impl Argument {
    pub fn new(prefix: Option<String>, name: Option<String>, value: Option<String>) -> Self {
        Self {
            prefix,
            name,
            value,
        }
    }

    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    pub fn get_prefix(&self) -> Option<&String> {
        self.prefix.as_ref()
    }

    pub fn get_value(&self) -> Option<&String> {
        self.value.as_ref()
    }
}

#[derive(Debug)]
pub struct ArgIterator {
    cache_prefixs: Vec<String>,

    index: usize,

    total: usize,

    args: Vec<String>,

    arg: Option<String>,

    next_arg: Option<String>,
}

impl ArgIterator {
    pub fn new() -> Self {
        Self {
            cache_prefixs: vec![],
            index: 0,
            total: 0,
            args: vec![],
            arg: None,
            next_arg: None,
        }
    }
}

impl Iterator for ArgIterator {
    fn set_prefix(&mut self, prefixs: Vec<String>) {
        let mut prefixs = prefixs;
        prefixs.sort_by(|a: &String, b: &String| b.len().cmp(&a.len()));
        debug!("Set all prefix to => {:?}", prefixs);
        self.cache_prefixs = prefixs;
    }

    fn set_args(&mut self, args: &mut dyn std::iter::Iterator<Item = String>) {
        self.args = args.map(|s|s).collect();
        self.total = self.args.len();
        debug!("Set command line to => {:?}", self.args);
    }

    fn reach_end(&self) -> bool {
        self.index >= self.total
    }

    fn fill_current_and_next(&mut self) {
        self.arg = Some(self.args[self.current_index()].clone());
        self.next_arg = if self.current_index() + 1 < self.count() {
            Some(self.args[self.current_index() + 1].clone())
        } else {
            None
        };
    }

    fn current(&self) -> &Option<String> {
        &self.arg
    }

    fn current_index(&self) -> usize {
        self.index
    }

    fn count(&self) -> usize {
        self.total
    }

    fn next(&self) -> &Option<String> {
        &self.next_arg
    }

    fn skip(&mut self) {
        self.index += 1;
    }

    fn parse(&self) -> Result<Argument> {
        match self.current() {
            Some(s) => {
                const SPLIT: &'static str = "=";

                let mut p_prefix: Option<String>;
                let p_name: Option<String>;
                let mut p_value: Option<String> = None;

                for prefix in &self.cache_prefixs {
                    if s.starts_with(prefix) {
                        p_prefix = Some(prefix.to_owned());
                        let (_, left_str) = s.split_at(prefix.len());
                        let name_or_value: Vec<_> = left_str.split(SPLIT).collect();

                        match name_or_value.len() {
                            1 => {
                                p_name = Some(left_str.to_owned());
                            }
                            2 => {
                                p_name = Some(name_or_value[0].to_owned());
                                p_value = Some(name_or_value[1].to_owned());
                            }
                            _ => {
                                continue;
                            }
                        }
                        return Ok(Argument::new(p_prefix, p_name, p_value));
                    }
                }
                Err(Error::InvalidOptionStr(s.clone()))
            }
            None => {
                Err(Error::InvalidNextArgument)
            }
        }
    }

    fn reset(&mut self) {
        self.index = 0;
        self.arg = None;
        self.next_arg = None;
    }
}
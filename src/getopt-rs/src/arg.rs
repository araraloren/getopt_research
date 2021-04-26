
use crate::error::Result;
use crate::error::Error;

pub trait Iterator {
    fn set_prefix(&mut self, prefix: Vec<String>);

    fn set_args<T: std::iter::Iterator<Item = String>>(&mut self, args: T);

    fn reach_end(&self) -> bool;

    fn fill_current_and_next(&mut self);

    fn current(&self) -> Option<&String>;

    fn current_index(&self) -> usize;

    fn count(&self) -> usize;

    fn next(&self) -> Option<&String>;

    fn increment(&mut self);

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
        self.cache_prefixs = prefixs;
    }

    fn set_args<T: std::iter::Iterator<Item = String>>(&mut self, args: T) {
        self.args = args.map(|s|s).collect();
        self.total = self.args.len();
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

    fn current(&self) -> Option<&String> {
        self.arg.as_ref()
    }

    fn current_index(&self) -> usize {
        self.index
    }

    fn count(&self) -> usize {
        self.total
    }

    fn next(&self) -> Option<&String> {
        self.next_arg.as_ref()
    }

    fn increment(&mut self) {
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
                        p_prefix = Some(String::from(prefix));
                        let (_, left_str) = s.split_at(prefix.len());
                        let name_or_value: Vec<_> = left_str.split(SPLIT).collect();

                        match name_or_value.len() {
                            1 => {
                                p_name = Some(String::from(left_str));
                            }
                            2 => {
                                p_name = Some(String::from(name_or_value[0]));
                                p_value = Some(String::from(name_or_value[1]));
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
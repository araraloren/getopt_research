
use crate::error::Result;
use crate::error::Error;

use std::fmt::Debug;
use std::iter::Iterator;

/// `IndexIterator` iterate the arguments by index.
/// It can access [`current`](IndexIterator::current) and [`next`](IndexIterator::next) argument at same time.
pub trait IndexIterator : Debug {
    /// Set available prefixs for all options.
    /// This is will help [`IndexIterator`] parse the argument.
    fn set_prefix(&mut self, prefix: Vec<String>);

    /// Set [`std::iter::Iterator`] of arguments.
    fn set_args(&mut self, args: &mut dyn Iterator<Item = String>);

    /// Return true if the [`IndexIterator`] is reach end
    fn reach_end(&self) -> bool;

    /// Read current and next argument if they are available
    fn fill_current_and_next(&mut self);

    /// Get current argument
    fn current(&self) -> &Option<String>;

    /// Get current argument's index
    fn current_index(&self) -> usize;

    /// Get total number of arguments
    fn count(&self) -> usize;

    /// Get next argument
    fn next(&self) -> &Option<String>;

    /// Increment the index to next argument 
    fn skip(&mut self);

    /// Parsing current argument to [`Argument`]
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

impl IndexIterator for ArgIterator {
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
        parse_argument(self.current(), &self.cache_prefixs)
    }

    fn reset(&mut self) {
        self.index = 0;
        self.arg = None;
        self.next_arg = None;
    }
}


pub fn parse_argument(s: &Option<String>, prefixs: &Vec<String>) -> Result<Argument> {
    match s {
        Some(s) => {
            const SPLIT: &'static str = "=";

            let mut p_prefix: Option<String>;
            let p_name: Option<String>;
            let mut p_value: Option<String> = None;

            for prefix in prefixs.iter() {
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

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn make_sure_arg_iterator_working() {
        {// test1
            let mut data = ["cpp", "-i=iostream", "-L", "libabc", "-o", "example.c++"]
                                 .iter()
                                 .map(|&a|String::from(a));
            let backup = data.clone().collect();
            let check = vec![
                vec![],
                ["-", "i", "iostream"].iter().map(|&a|String::from(a)).collect(),
                ["-", "L"].iter().map(|&a|String::from(a)).collect(),
                vec![],
                ["-", "o"].iter().map(|&a|String::from(a)).collect(),
            ];

            let mut iter = ArgIterator::new();

            iter.set_args(&mut data);
            iter.set_prefix(vec!["-".to_owned()]);
            testing_one_iterator(&mut iter, &backup, &check);
        }

        {// test1
            let mut data = ["c", "+std=c11", "-i=stdlib.h", "-L", "libabc", "--include=stdio.h", "example.c++"]
                                 .iter()
                                 .map(|&a|String::from(a));
            let backup = data.clone().collect();
            let check = vec![
                vec![],
                ["+", "std", "c11"].iter().map(|&a|String::from(a)).collect(),
                ["-", "i", "stdlib.h"].iter().map(|&a|String::from(a)).collect(),
                ["-", "L"].iter().map(|&a|String::from(a)).collect(),
                vec![],
                ["--", "include", "stdio.h"].iter().map(|&a|String::from(a)).collect(),
            ];

            let mut iter = ArgIterator::new();

            iter.set_args(&mut data);
            iter.set_prefix(vec!["-".to_owned(), "--".to_owned(), "+".to_owned()]);
            testing_one_iterator(&mut iter, &backup, &check);
        }
    }

    fn testing_one_iterator(iter: &mut dyn IndexIterator, original_data: &Vec<String>, check: &Vec<Vec<String>>) {
        
        assert!(iter.current().is_none());
        assert!(iter.next().is_none());
        assert_eq!(iter.count(), original_data.len());

        while ! iter.reach_end() {
            iter.fill_current_and_next();

            assert_eq!( iter.current().as_ref(), original_data.get(iter.current_index()) );
            assert_eq!( iter.next().as_ref(), original_data.get(iter.current_index() + 1) );

            iter.skip();
        }
        assert!(iter.reach_end());

        iter.reset();

        assert!(iter.current().is_none());
        assert!(iter.next().is_none());

        while ! iter.reach_end() {
            iter.fill_current_and_next();

            if let Ok(data) = iter.parse() {
                assert_eq!(data.get_prefix(), check[iter.current_index()].get(0));
                assert_eq!(data.get_name(), check[iter.current_index()].get(1));
                assert_eq!(data.get_value(), check[iter.current_index()].get(2));
                iter.skip();
            }

            iter.skip();
        }
        assert!(iter.reach_end());
    }
}
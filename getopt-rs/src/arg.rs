
use crate::error::Result;
use crate::error::Error;

use std::fmt::Debug;
use std::iter::Iterator;
use async_trait::async_trait;

/// `IndexIterator` iterate the arguments by index.
/// It can access [`current`](IndexIterator::current) and [`next`](IndexIterator::next) argument at same time.
#[async_trait]
pub trait IndexIterator : Debug {
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
    #[cfg(not(feature="async"))]
    fn parse(&self, prefixs: &Vec<String>) -> Result<Argument>;

    /// Parsing current argument to [`Argument`]
    #[cfg(feature="async")]
    async fn parse(&self, prefixs: &Vec<String>) -> Result<Argument>;

    fn reset(&mut self);
}

#[derive(Debug, Clone)]
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

    pub fn from_args(args: &mut dyn std::iter::Iterator<Item = String>) -> Self {
        let mut ret = Self {
            cache_prefixs: vec![],
            index: 0,
            total: 0,
            args: vec![],
            arg: None,
            next_arg: None,
        };
        ret.set_args(args);
        ret
    }
}

#[async_trait]
impl IndexIterator for ArgIterator {
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

    #[cfg(not(feature="async"))]
    fn parse(&self, prefixs: &Vec<String>) -> Result<Argument> {
        parse_argument(self.current(), prefixs)
    }

    #[cfg(feature="async")]
    async fn parse(&self, prefixs: &Vec<String>) -> Result<Argument> {
        parse_argument(self.current(), prefixs).await
    }

    fn reset(&mut self) {
        self.index = 0;
        self.arg = None;
        self.next_arg = None;
    }
}

#[cfg(feature="async")]
pub async fn parse_argument(s: &Option<String>, prefixs: &Vec<String>) -> Result<Argument> {
    parse_argument_impl(s, prefixs)
}

#[cfg(not(feature="async"))]
pub fn parse_argument(s: &Option<String>, prefixs: &Vec<String>) -> Result<Argument> {
    parse_argument_impl(s, prefixs)
}

fn parse_argument_impl(s: &Option<String>, prefixs: &Vec<String>) -> Result<Argument> {
    match s {
        Some(s) => {
            const SPLIT: &'static str = "=";

            let p_prefix: Option<String>;
            let p_name: Option<String>;
            let mut p_value: Option<String> = None;

            for prefix in prefixs.iter() {
                if s.starts_with(prefix) {
                    let (_, left_str) = s.split_at(prefix.len());

                    if left_str.len() > 0 {
                        p_prefix = Some(prefix.to_owned());
                        let name_or_value: Vec<_> = left_str.splitn(2, SPLIT).collect();

                        if name_or_value.len() > 1 {
                            p_name = Some(name_or_value[0].to_owned());
                            p_value = Some(name_or_value[1].to_owned());
                        }
                        else {
                            p_name = Some(left_str.to_owned());
                        }

                        return Ok(Argument::new(p_prefix, p_name, p_value));
                    }
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
            testing_one_iterator(&mut iter, &vec!["-".to_owned()], &backup, &check);
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
            testing_one_iterator(&mut iter, &vec!["--".to_owned(), "-".to_owned(), "+".to_owned()], &backup, &check);
        }
    }

    fn testing_one_iterator(iter: &mut dyn IndexIterator, prefixs: &Vec<String>, original_data: &Vec<String>, check: &Vec<Vec<String>>) {
        
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

            if let Ok(data) = iter.parse(prefixs) {
                assert_eq!(data.get_prefix(), check[iter.current_index()].get(0));
                assert_eq!(data.get_name(), check[iter.current_index()].get(1));
                assert_eq!(data.get_value(), check[iter.current_index()].get(2));
                iter.skip();
            }

            iter.skip();
        }
        assert!(iter.reach_end());
    }

    #[test]
    fn test_parse_argument() {
        let prefixs: Vec<String> = ["--", "-", "+", "/"].iter().map(|&v|String::from(v)).collect();

        let test_cases = [
            ("--option", Some(("--", "option", None))),
            ("--option=1", Some(("--", "option", Some("1".to_owned())))),
            ("-o", Some(("-", "o", None))),
            ("+o", Some(("+", "o", None))),
            ("/o", Some(("/", "o", None))),
            ("-", None),
            ("-w=1=2", Some(("-", "w", Some("1=2".to_owned())))),
        ];

        for acase in &test_cases {
            if let Ok(a) = parse_argument(&Some(acase.0.to_owned()), &prefixs) {
                if let Some(atest) = &acase.1 {
                    assert_eq!(atest.0, a.get_prefix().unwrap().as_str());
                    assert_eq!(atest.1, a.get_name().unwrap().as_str());
                    assert_eq!(atest.2.as_ref(), a.get_value());
                }
            }
            else {
                assert_eq!(acase.1, None);
            }
        }
    }
}
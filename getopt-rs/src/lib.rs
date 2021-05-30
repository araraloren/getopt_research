pub mod id;
pub mod error;
pub mod callback;
pub mod opt;
pub mod ctx;
pub mod set;
pub mod arg;
pub mod proc;
pub mod utils;
pub mod parser;
pub mod nonopt;

#[macro_use]
extern crate log;

extern crate async_trait;

pub mod prelude {
    pub use crate::error::{Result, Error};
    pub use crate::parser::{Parser, ForwardParser, DelayParser, PreParser};
    pub use crate::set::{Set, DefaultSet};
    pub use crate::arg::{IndexIterator, ArgIterator};
    pub use crate::id::{IdGenerator, DefaultIdGen, Identifier};
    pub use crate::proc::{Proc, Subscriber, SequenceProc};
    pub use crate::opt::Opt;
    pub use crate::callback::{CallbackType, OptCallback};
    pub use crate::getopt_impl;
    
    /// getopt will set do the previous work for you,
    /// and call the getopt_impl.
    /// 
    /// For example,
    /// the `ai` is an instance of [`ArgIterator`].
    /// The `parser` is an instance of [`Parser`].
    /// And `set` is an instance of [`Set`].
    /// 
    /// `getopt(ai, parser, set)` will may expand to 
    /// ```ignore
    /// {
    ///     let mut parsers: Vec<Box<dyn Parser>> = ::alloc::vec::Vec::new();
    ///     set.subscribe_from(&mut parser);
    ///     parser.publish_to(Box::new(set));
    ///     parsers.push(Box::new(parser));
    ///     getopt_impl(&mut ai, parsers)
    /// }
    /// ```
    /// 
    /// `getopt(ai, parser1, set1, parser2, set2)` will may expand to 
    /// ```ignore
    /// {
    ///     let mut parsers: Vec<Box<dyn Parser>> = ::alloc::vec::Vec::new();
    ///     set1.subscribe_from(&mut parser1);
    ///     parser1.publish_to(Box::new(set1));
    ///     parsers.push(Box::new(parser1));
    ///     set2.subscribe_from(&mut parser2);
    ///     parser2.publish_to(Box::new(set2));
    ///     parsers.push(Box::new(parser2));
    ///     getopt_impl(&mut ai, parsers)
    /// }
    /// ```
    pub use getopt_rs_macro::getopt;
}

use prelude::*;

#[cfg(not(feature="async"))]
pub fn getopt_impl<G>(iter: &mut dyn IndexIterator, parsers: Vec<Box<dyn Parser<G>>>) -> Result<Option<Box<dyn Parser<G>>>>
    where G: IdGenerator {
    for mut parser in parsers {
        let ret = parser.parse(iter)?;

        if let Some(ret) = ret {
            if ret {
                return Ok(Some(parser));
            }
            else {
                iter.reset();
            }
        }
    }
    Ok(None)
}

#[cfg(feature="async")]
pub async fn getopt_impl<G>(iter: &mut dyn IndexIterator, parsers: Vec<Box<dyn Parser<G>>>) -> Result<Option<Box<dyn Parser<G>>>>
    where G: IdGenerator {
    for mut parser in parsers {
        let ret = parser.parse(iter).await?;

        if let Some(ret) = ret {
            if ret {
                return Ok(Some(parser));
            }
            else {
                iter.reset();
            }
        }
    }
    Ok(None)
}

pub mod tools {
    use crate::prelude::*;
    use simplelog::{Config, CombinedLogger, SimpleLogger};
    use log::LevelFilter;


    pub fn idgenerator(id: u64) -> DefaultIdGen  {
        DefaultIdGen::new(crate::id::Identifier::new(id))
    }

    pub fn open_log() -> std::result::Result<(), log::SetLoggerError> {
        CombinedLogger::init(vec![
            SimpleLogger::new(LevelFilter::Warn, Config::default()),
            SimpleLogger::new(LevelFilter::Error, Config::default()),
            SimpleLogger::new(LevelFilter::Debug, Config::default()),
            SimpleLogger::new(LevelFilter::Info, Config::default()),
        ])
    }

    pub fn delay_parse(id_generator: DefaultIdGen) -> DelayParser<DefaultIdGen> {
        DelayParser::new(id_generator)
    }

    pub fn pre_parse(id_generator: DefaultIdGen) -> PreParser<DefaultIdGen> {
        PreParser::new(id_generator)
    }

    pub fn forward_parse(id_generator: DefaultIdGen) -> ForwardParser<DefaultIdGen> {
        ForwardParser::new(id_generator)
    }

    #[cfg(not(feature="async"))]
    pub fn simple_value_callback<F>(t: F) -> OptCallback where F: 'static + FnMut(&dyn Opt) -> Result<bool> {
        OptCallback::from_value(Box::new(crate::callback::SimpleValueCallback::new(t)))
    }

    #[cfg(not(feature="async"))]
    pub fn simple_index_callback<F>(t: F) -> OptCallback where F: 'static + FnMut( &dyn Set, &String ) -> Result<bool> {
        OptCallback::from_index(Box::new(crate::callback::SimpleIndexCallback::new(t))) 
    }

    #[cfg(not(feature="async"))]
    pub fn simple_main_callback<F>(t: F) -> OptCallback where F: 'static + FnMut( &dyn Set, &Vec<String> ) -> Result<bool> {
        OptCallback::from_main(Box::new(crate::callback::SimpleMainCallback::new(t))) 
    }
}
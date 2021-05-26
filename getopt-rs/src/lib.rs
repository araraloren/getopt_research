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
    pub use crate::set::Set;
    pub use crate::error::Result;
    pub use crate::parser::Parser;
    pub use crate::set::DefaultSet;
    pub use crate::arg::IndexIterator;
    pub use crate::arg::ArgIterator;
    pub use crate::id::IdGenerator;
    pub use crate::id::DefaultIdGen;
    pub use crate::id::Identifier;
    pub use crate::proc::Subscriber;
    pub use crate::opt::Opt;
    pub use crate::callback::*;
    pub use crate::getopt_impl;
    
    /// getopt will set do the previous work for you,
    /// and call the getopt_impl.
    /// 
    /// For example,
    /// the `ai` is an instance of [`ArgIterator`].
    /// The `parser` is an instance of [`Parser`].
    /// And `set` is an instance of [`Set`].
    /// 
    /// `getopt(ai, parser, set)` will expand to 
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
    /// `getopt(ai, parser1, set1, parser2, set2)` will expand to 
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
pub fn getopt_impl(iter: &mut dyn IndexIterator, parsers: Vec<Box<dyn Parser>>) -> Result<Option<Box<dyn Parser>>> {
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
pub async fn getopt_impl(iter: &mut dyn IndexIterator, parsers: Vec<Box<dyn Parser>>) -> Result<Option<Box<dyn Parser>>> {
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
    use crate::callback::OptCallback;
    use crate::parser::DelayParser;
    use crate::parser::ForwardParser;
    use crate::parser::PreParser;
    use crate::callback::*;
    use crate::prelude::*;
    use simplelog::CombinedLogger;
    use simplelog::SimpleLogger;
    use log::LevelFilter;
    use simplelog::Config;


    pub fn idgenerator(id: u64) -> Box<dyn IdGenerator>  {
        Box::new(DefaultIdGen::new(crate::id::Identifier::new(id)))
    }

    pub fn open_log() -> std::result::Result<(), log::SetLoggerError> {
        CombinedLogger::init(vec![
            SimpleLogger::new(LevelFilter::Warn, Config::default()),
            SimpleLogger::new(LevelFilter::Error, Config::default()),
            SimpleLogger::new(LevelFilter::Debug, Config::default()),
            SimpleLogger::new(LevelFilter::Info, Config::default()),
        ])
    }

    pub fn delay_parser(id_generator: Box<dyn IdGenerator>) -> DelayParser {
        DelayParser::new(id_generator)
    }

    pub fn pre_parser(id_generator: Box<dyn IdGenerator>) -> PreParser {
        PreParser::new(id_generator)
    }

    pub fn forward_parser(id_generator: Box<dyn IdGenerator>) -> ForwardParser {
        ForwardParser::new(id_generator)
    }

    pub fn simple_value_callback<T>(t: T) -> OptCallback where T: 'static + FnMut(&dyn Opt) -> Result<bool> {
        OptCallback::from_value(Box::new(SimpleValueCallback::new(t))) 
    }

    pub fn simple_index_callback<T>(t: T) -> OptCallback where T: 'static + FnMut( &dyn Set, &String ) -> Result<bool> {
        OptCallback::from_index(Box::new(SimpleIndexCallback::new(t))) 
    }

    pub fn simple_main_callback<T>(t: T) -> OptCallback where T: 'static + FnMut( &dyn Set, &Vec<String> ) -> Result<bool> {
        OptCallback::from_main(Box::new(SimpleMainCallback::new(t))) 
    }
}
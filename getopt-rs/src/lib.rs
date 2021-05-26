pub mod id;
pub mod opt;
pub mod ctx;
pub mod set;
pub mod arg;
pub mod proc;
pub mod help;
pub mod error;
pub mod utils;
pub mod parser;
pub mod nonopt;
pub mod callback;

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

pub fn create_idgenerator(id: u64) -> Box<dyn IdGenerator>  {
    Box::new(DefaultIdGen::new(crate::id::Identifier::new(id)))
}
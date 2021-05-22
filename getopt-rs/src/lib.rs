
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
    pub use crate::getopt_impl;
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
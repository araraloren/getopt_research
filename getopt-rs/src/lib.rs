
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

pub mod prelude {
    pub use crate::set::Set;
    pub use crate::error::Result;
    pub use crate::parser::Parser;
    pub use crate::set::DefaultSet;
    pub use crate::arg::IndexIterator;
    pub use crate::arg::ArgIterator;
    pub use crate::id::IdGenerator;
    pub use crate::id::DefaultIdGen;
}

use prelude::*;

#[macro_export]
macro_rules! getopt {
    ( $( $parser:ident ),+ ) => {{
        use getopt_rs::getopt_impl;
        use getopt_rs::arg::IndexIterator;
        use getopt_rs::arg::ArgIterator;

        let parsers: Vec<Box<dyn Parser>>  = vec![ $( Box::new($parser) ),+ ];
        let mut iter = ArgIterator::new();

        iter.set_args(&mut std::env::args().skip(1));
        getopt_impl(&mut iter, parsers)
    }};

    ( $iter:expr, $( $parser:ident ),+ ) => {{
        use getopt_rs::getopt_impl;

        let parsers: Vec<Box<dyn Parser>>  = vec![ $( Box::new($parser) ),+ ];

        getopt_impl($iter, parsers)
    }};
}

pub fn getopt_impl(iter: &mut dyn IndexIterator, parsers: Vec<Box<dyn Parser>>) -> Option<Box<dyn Parser>> {
    for mut parser in parsers {
        match parser.parse(iter) {
            Ok(ret) => {
                if let Some(ret) = ret {
                    if ret {
                        return Some(parser);
                    }
                    else {
                        iter.reset();
                    }
                }
            }
            Err(e) => {
                error!("CATCH {:?}", e);
            }
        }
    }
    None
}

pub fn create_idgenerator(id: u64) -> Box<dyn IdGenerator>  {
    Box::new(DefaultIdGen::new(crate::id::Identifier::new(id)))
}
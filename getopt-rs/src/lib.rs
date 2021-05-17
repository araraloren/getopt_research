
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
}

use prelude::*;

#[macro_export]
macro_rules! getopt {
    ( $( $parser:ident ),+ ) => {
        use getopt_rs::getopt_impl;
        use getopt_rs::arg::IndexIterator;
        use getopt_rs::arg::ArgIterator;

        let parsers: Vec<Box<dyn Parser>>  = vec![ $( Box::new($parser) ),+ ];
        let mut iter = ArgIterator::new();

        iter.set_args(&mut std::env::args());
        getopt_impl(&mut iter, parsers)
    };

    ( $iter:expr, $( $parser:ident ),+ ) => {
        use getopt_rs::getopt_impl;

        let parsers: Vec<Box<dyn Parser>>  = vec![ $( Box::new($parser) ),+ ];

        getopt_impl($iter, parsers)
    };
}

pub fn getopt_impl(iter: &mut dyn IndexIterator, parsers: Vec<Box<dyn Parser>>) -> Option<Box<dyn Parser>> {
    for mut parser in parsers {
        if let Ok(ret) = parser.parse(iter) {
            if let Some(ret) = ret {
                if ret {
                    return Some(parser);
                }
                else {
                    iter.reset();
                }
            }
        }
    }
    None
}
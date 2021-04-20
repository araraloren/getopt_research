mod bool;
mod ctx;
mod err;
mod id;
mod opt;
mod parser;
mod proc;
mod set;
mod str;
mod utils;

use id::DefaultIdGen;
use set::Set;

#[macro_use]
extern crate log;

use simplelog::*;

fn main() -> Result<(), err::Error> {
    CombinedLogger::init(vec![
        SimpleLogger::new(LevelFilter::Warn, Config::default()),
        SimpleLogger::new(LevelFilter::Debug, Config::default()),
        SimpleLogger::new(LevelFilter::Error, Config::default()),
        SimpleLogger::new(LevelFilter::Info, Config::default()),
    ])
    .unwrap();

    let mut set = Set::new(Box::new(DefaultIdGen::new()));

    set.add_utils(
        crate::str::current_type(),
        Box::new(crate::str::StrUtils::new()),
    );
    set.add_utils(
        crate::bool::current_type(),
        Box::new(crate::bool::BoolUtils::new()),
    );
    set.add_str_opt("q=str", "-")?;
    set.add_str_opt("query=str", "--")?;
    set.add_bool_opt("other=bool", "--")?;

    let mut parser = parser::Parser::new();

    set.subscribe_from(&mut parser);
    parser.publish_to(set);
    parser.init(["let", "-q", "foo", "--query", "bar", "--other", "we", "are", "noa"]
                        .iter()
                        .map(|&s| String::from(s)));
    parser.parse();

    dbg!(parser.set());

    Ok(())
}

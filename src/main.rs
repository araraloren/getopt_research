
use getopt_rs::parser::ForwardParser;
use getopt_rs::set::DefaultSet;
use getopt_rs::set::Set;
use getopt_rs::proc::Subscriber;
use getopt_rs::parser::Parser;
use getopt_rs::id::DefaultIdGen;
use getopt_rs::id::Identifier;
use getopt_rs::arg::Iterator;
use getopt_rs::arg::ArgIterator;
use getopt_rs::opt;
use simplelog::*;

fn main() {
    CombinedLogger::init(vec![
        SimpleLogger::new(LevelFilter::Warn, Config::default()),
        SimpleLogger::new(LevelFilter::Debug, Config::default()),
        SimpleLogger::new(LevelFilter::Error, Config::default()),
        SimpleLogger::new(LevelFilter::Info, Config::default()),
    ])
    .unwrap();

    let mut set = DefaultSet::new();
    let mut parser = ForwardParser::new(Box::new(DefaultIdGen::new(Identifier::new(0))));

    set.add_utils(Box::new(opt::str::StrUtils::new())).unwrap();
    set.add_utils(Box::new(opt::bool::BoolUtils::new())).unwrap();
    set.add_utils(Box::new(opt::arr::ArrUtils::new())).unwrap();
    set.add_utils(Box::new(opt::int::IntUtils::new())).unwrap();
    set.add_utils(Box::new(opt::example::PathUtils::new())).unwrap();

    if let Ok(mut commit) = set.add_opt("-|q=str") {
        commit.add_alias("--", "query");
        commit.commit().unwrap();
    }

    if let Ok(mut commit) = set.add_opt("-|f=bool") {
        commit.add_alias("--", "force");
        commit.commit().unwrap();
    }
    
    if let Ok(mut commit) = set.add_opt("-|k=arr") {
        commit.add_alias("--", "keyword");
        commit.commit().unwrap();
    }

    if let Ok(mut commit) = set.add_opt("-|id=int") {
        commit.add_alias("--", "identifier");
        commit.commit().unwrap();
    }

    if let Ok(mut commit) = set.add_opt("-|i=path") {
        commit.add_alias("--", "import");
        commit.set_deafult_value(getopt_rs::opt::OptValue::from_any(Box::new(std::path::PathBuf::from("E:\\rust"))));
        commit.commit().unwrap();
    }

    set.subscribe_from(&mut parser);
    parser.publish_to(Box::new(set));

    let mut ai = ArgIterator::new();

    ai.set_args(
        &mut ["let", "--query", "bar", "--force", "-id", "-123", "-i", "E:\\rust\\getopt", "-k", "we", "--keyword", "are", "noa"]
        .iter()
        .map(|&s| String::from(s))
    );
    parser.parse(&mut ai).unwrap();

    dbg!( parser.set().as_ref().unwrap().filter("force").unwrap().find() );
    dbg!( parser.set().as_ref().unwrap().filter("-|id").unwrap().find() );

    use std::path::PathBuf;

    if let Ok(filter) = parser.set().as_ref().unwrap().filter("-|i") {
        let value = filter.find().unwrap().value();
        
        dbg!(value.downcast_ref::<PathBuf>());
    }
}


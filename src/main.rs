
#[macro_use]
extern crate getopt_rs;

use getopt_rs::{arg::ArgIterator};
use getopt_rs::arg::IndexIterator;
use getopt_rs::callback::OptCallback;
#[cfg(not(features="async"))]
use getopt_rs::callback::*;
use getopt_rs::id::DefaultIdGen;
use getopt_rs::id::Identifier;
use getopt_rs::opt::Opt;
use getopt_rs::parser::ForwardParser;
use getopt_rs::parser::DelayParser;
use getopt_rs::parser::Parser;
use getopt_rs::proc::Subscriber;
use getopt_rs::set::DefaultSet;
use getopt_rs::set::Set;
use getopt_rs::error::Result;
use simplelog::*;
use std::sync::Arc;
use std::sync::Mutex;
use getopt_rs_macro::getopt;
use getopt_rs::getopt_impl;

#[async_std::main]
async fn main() {
    CombinedLogger::init(vec![
        SimpleLogger::new(LevelFilter::Warn, Config::default()),
        SimpleLogger::new(LevelFilter::Error, Config::default()),
        //SimpleLogger::new(LevelFilter::Debug, Config::default()),
        SimpleLogger::new(LevelFilter::Info, Config::default()),
    ])
    .unwrap();

    example3();
}


fn example3() {
    let mut cache: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(vec![]));
    let mut set = DefaultSet::new();
    let mut parser = DelayParser::new(Box::new(DefaultIdGen::new(Identifier::new(0))));

    set.initialize_prefixs();
    set.initialize_utils().unwrap();
 
    if let Ok(mut commit) = set.add_opt("-d=bool") {
        let id = commit.commit().unwrap();
        let cache_ref = cache.clone();
        parser.set_callback(id, 
            OptCallback::Value(Box::new(SimpleValueCallback::new(
                move |_| {
                    let mut writer = cache_ref.lock().unwrap();
                    let ret = (*writer).iter()
                        .filter(|&v|{ std::path::Path::new(v.as_str()).is_dir()})
                        .map(|v| { v.clone() })
                        .collect();
                    *writer = ret;
                    Ok(true)
                }
            ))));
    }
    if let Ok(mut commit) = set.add_opt("-f=bool") {
        let id = commit.commit().unwrap();
        let cache_ref = cache.clone();
        parser.set_callback(id, 
            OptCallback::Value(Box::new(SimpleValueCallback::new(
                move |_| {
                    let mut writer = cache_ref.lock().unwrap();
                    let ret = (*writer).iter()
                        .filter(|&v|{ std::path::Path::new(v.as_str()).is_file()})
                        .map(|v| { v.clone() })
                        .collect();
                    *writer = ret;
                    Ok(true)
                }
            ))));
    }
    if let Ok(mut commit) = set.add_opt("-l=bool") {
        let id = commit.commit().unwrap();
        let cache_ref = cache.clone();
        parser.set_callback(id, 
            OptCallback::Value(Box::new(SimpleValueCallback::new(
                move |_| {
                    let mut writer = cache_ref.lock().unwrap();
                    let ret = (*writer).iter()
                        .filter(|&v|{ std::path::Path::new(v.as_str()).read_link().is_ok()})
                        .map(|v| { v.clone() })
                        .collect();
                    *writer = ret;
                    Ok(true)
                }
            ))));
    }
    if let Ok(mut commit) = set.add_opt("-s=uint") {
        let id = commit.commit().unwrap();
        let cache_ref = cache.clone();
        parser.set_callback(id, 
            OptCallback::Value(Box::new(SimpleValueCallback::new(
                move |opt| {
                    let mut writer = cache_ref.lock().unwrap();
                    let ret = (*writer).iter()
                        .filter(|&v|{ 
                            let metadata = std::fs::metadata(v).unwrap();
                            metadata.len() > *opt.value().as_uint().unwrap()
                        })
                        .map(|v| { v.clone() })
                        .collect();
                    *writer = ret;
                    Ok(true)
                }
            ))));
    }
    if let Ok(mut commit) = set.add_opt("directory=pos@1") {
        let id = commit.commit().unwrap();
        let cache_ref = cache.clone();
        parser.set_callback(id, 
            OptCallback::Index(Box::new(SimpleIndexCallback::new(
                move |_, v| {
                    let mut writer = cache_ref.lock().unwrap();
                    for entry in std::fs::read_dir(v).unwrap() {
                        let entry = entry.unwrap();
                        
                        (*writer).push(entry.path().to_str().unwrap().to_owned());
                    }
                    Ok(true)
                }
            ))));
    }
    if let Ok(mut commit) = set.add_opt("main=main") {
        let id = commit.commit().unwrap();
        let cache_ref = cache.clone();
        parser.set_callback(id, 
            OptCallback::Main(Box::new(SimpleMainCallback::new(
                move |_, noa| {
                    let mut regex: Option<regex::Regex> = None;

                    if noa.len() == 2 {
                        regex = regex::Regex::new(noa[1].as_str()).ok();
                    }
                    for file in cache_ref.lock().unwrap().iter() {
                        match &regex {
                            Some(regex) => {
                                if regex.is_match(file) {
                                    println!("{}", file);
                                }
                            }
                            None => {
                                println!("{}", file);
                            }
                        }
                    }
                    Ok(true)
                }
            ))));
    }

    // set.subscribe_from(&mut parser);
    // parser.publish_to(Box::new(set));

    let mut ai: ArgIterator = ArgIterator::new();

    ai.set_args(&mut std::env::args().skip(1));

    getopt!(ai, parser, set);
}


// async fn example3() {
//     let cache: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(vec![]));
//     let mut set = DefaultSet::new();
//     let mut parser = DelayParser::new(Box::new(DefaultIdGen::new(Identifier::new(0))));

//     set.initialize_prefixs();
//     set.initialize_utils().unwrap();

//     #[derive(Debug)]
//     struct AsyncValueCallback(Arc<Mutex<Vec<String>>>);

//     #[async_trait::async_trait(?Send)]
//     impl ValueCallback for AsyncValueCallback {
//         async fn call(&mut self, opt: &dyn Opt) -> Result<bool> {
//             let mut writer = self.0.lock().unwrap();
//             let ret = match opt.name() {
//                 "d" => {
//                     (*writer).iter()
//                     .filter(|&v|{ std::path::Path::new(v.as_str()).is_dir()})
//                     .map(|v| { v.clone() })
//                     .collect()
//                 }
//                 "f" => {
//                     (*writer).iter()
//                     .filter(|&v|{ std::path::Path::new(v.as_str()).is_file()})
//                     .map(|v| { v.clone() })
//                     .collect()
//                 }
//                 "l" => {
//                     (*writer).iter()
//                     .filter(|&v|{ std::path::Path::new(v.as_str()).read_link().is_ok()})
//                     .map(|v| { v.clone() })
//                     .collect()
//                 }
//                 "s" => {
//                     (*writer).iter()
//                     .filter(|&v|{ std::fs::metadata(v).unwrap().len() > *opt.value().as_uint().unwrap() })
//                     .map(|v| { v.clone() })
//                     .collect()
//                 }
//                 _ => {
//                     panic!("Unknow option name!")
//                 }
//             };
//             *writer = ret;
//             Ok(true)
//         }
//     }
 
//     if let Ok(mut commit) = set.add_opt("-d=bool") {
//         let id = commit.commit().unwrap();
//         parser.set_callback(id, OptCallback::Value(Box::new(AsyncValueCallback(cache.clone()))));
//     }
//     if let Ok(mut commit) = set.add_opt("-f=bool") {
//         let id = commit.commit().unwrap();
//         parser.set_callback(id, OptCallback::Value(Box::new(AsyncValueCallback(cache.clone()))));
//     }
//     if let Ok(mut commit) = set.add_opt("-l=bool") {
//         let id = commit.commit().unwrap();
//         parser.set_callback(id, OptCallback::Value(Box::new(AsyncValueCallback(cache.clone()))));
//     }
//     if let Ok(mut commit) = set.add_opt("-s=uint") {
//         let id = commit.commit().unwrap();
//         parser.set_callback(id, OptCallback::Value(Box::new(AsyncValueCallback(cache.clone()))));
//     }

//     #[derive(Debug)]
//     struct AsyncIndexCallback(Arc<Mutex<Vec<String>>>);

//     #[async_trait::async_trait(?Send)]
//     impl IndexCallback for AsyncIndexCallback {
//         async fn call(&mut self, _: &dyn Set, v: &String) -> Result<bool> {
//             let mut writer = self.0.lock().unwrap();
//             for entry in std::fs::read_dir(v).unwrap() {
//                 let entry = entry.unwrap();
                
//                 (*writer).push(entry.path().to_str().unwrap().to_owned());
//             }
//             Ok(true)
//         }
//     }
//     if let Ok(mut commit) = set.add_opt("directory=pos@1") {
//         let id = commit.commit().unwrap();
//         parser.set_callback(id, OptCallback::Index(Box::new(AsyncIndexCallback(cache.clone()))));
//     }

//     #[derive(Debug)]
//     struct AsyncMainCallback(Arc<Mutex<Vec<String>>>);

//     #[async_trait::async_trait(?Send)]
//     impl MainCallback for AsyncMainCallback {
//         async fn call(&mut self, _: &dyn Set, noa: &Vec<String>) -> Result<bool> {
//             let mut regex: Option<regex::Regex> = None;

//             if noa.len() == 2 {
//                 regex = regex::Regex::new(noa[1].as_str()).ok();
//             }
//             for file in self.0.lock().unwrap().iter() {
//                 match &regex {
//                     Some(regex) => {
//                         if regex.is_match(file) {
//                             println!("{}", file);
//                         }
//                     }
//                     None => {
//                         println!("{}", file);
//                     }
//                 }
//             }
//             Ok(true)
//         }
//     }
//     if let Ok(mut commit) = set.add_opt("main=main") {
//         let id = commit.commit().unwrap();
//         parser.set_callback(id, OptCallback::Main(Box::new(AsyncMainCallback(cache.clone()))));
//     }

//     set.subscribe_from(&mut parser);
//     parser.publish_to(Box::new(set));

//     let mut ai: ArgIterator = ArgIterator::new();

//     ai.set_args(&mut std::env::args().skip(1));

//     parser.parse(&mut ai).await.unwrap();
// }

// fn example2() {
//     let count = Arc::new(Mutex::new(32));

//     let ref_count = count.clone();

//     let mut set = DefaultSet::new();
//     let mut parser = DelayParser::new(Box::new(DefaultIdGen::new(Identifier::new(0))));

//     set.initialize_utils().unwrap();
//     set.initialize_prefixs();
//     set.add_utils(Box::new(opt::example::PathUtils::new())).unwrap();

//     if let Ok(mut commit) = set.add_opt("-q=str") {
//         commit.add_alias("--", "query");
//         let id = commit.commit().unwrap();
//         parser.set_callback(
//             id,
//             OptCallback::Value(Box::new(SimpleValueCallback::new(move |opt: &dyn opt::Opt| {
//                 dbg!("got a opt: ", opt);
//                 let mut writer = ref_count.lock().unwrap();
//                 *writer = 42;
//                 Ok(true)
//             }))),
//         );
//     }

//     if let Ok(mut commit) = set.add_opt("-f=bool") {
//         commit.add_alias("--", "force");
//         commit.commit().unwrap();
//     }

//     if let Ok(mut commit) = set.add_opt("-k=array") {
//         commit.add_alias("--", "keyword");
//         commit.commit().unwrap();
//     }

//     if let Ok(mut commit) = set.add_opt("-id=int") {
//         commit.add_alias("--", "identifier");
//         commit.commit().unwrap();
//     }

//     if let Ok(mut commit) = set.add_opt("-i=path") {
//         commit.add_alias("--", "import");
//         commit.set_deafult_value(getopt_rs::opt::OptValue::from_any(Box::new(
//             std::path::PathBuf::from("E:\\rust"),
//         )));
//         commit.commit().unwrap();
//     }

//     if let Ok(mut commit) = set.add_opt("operator=pos@-1") {
//         let id = commit.commit().unwrap();
//         parser.set_callback(
//             id,
//             OptCallback::from_index(Box::new(SimpleIndexCallback::new(
//                 |set: &dyn Set, arg: &String| {
//                     println!("In pos Meeting {:?}", arg);
//                     Ok(true)
//                 },
//             ))),
//         )
//     }

//     if let Ok(mut commit) = set.add_opt("mysql=cmd") {
//         let id = commit.commit().unwrap();
//         parser.set_callback(
//             id,
//             OptCallback::from_main(Box::new(SimpleMainCallback::new(
//                 |set: &dyn Set, args: &Vec<String>| {
//                     println!("In cmd Meeting {:?}", args);
//                     Ok(true)
//                 },
//             ))),
//         )
//     }

//     if let Ok(mut commit) = set.add_opt("main=main") {
//         let id = commit.commit().unwrap();
//         parser.set_callback(
//             id,
//             OptCallback::from_main(Box::new(SimpleMainCallback::new(
//                 |set: &dyn Set, args: &Vec<String>| {
//                     println!("In main Meeting {:?} ", args);
//                     Ok(true)
//                 },
//             ))),
//         )
//     }

//     set.subscribe_from(&mut parser);
//     parser.publish_to(Box::new(set));

//     let mut ai = ArgIterator::new();

//     ai.set_args(
//         &mut [
//             "mysql",
//             "--query",
//             "bar",
//             "--force",
//             "-id",
//             "-123",
//             "-i",
//             "E:\\rust\\getopt",
//             "-k",
//             "we",
//             "--keyword",
//             "are",
//             "submit",
//         ]
//         .iter()
//         .map(|&s| String::from(s)),
//     );
    
//     let ret = parser.parse(&mut ai);

//     dbg!(ret);
// }

// fn exmaple1() {
//     let count = Arc::new(Mutex::new(32));

//     let ref_count = count.clone();

//     let mut set = DefaultSet::new();
//     let mut parser = DelayParser::new(Box::new(DefaultIdGen::new(Identifier::new(0))));

//     set.initialize_utils().unwrap();
//     set.initialize_prefixs();
//     set.add_utils(Box::new(opt::example::PathUtils::new())).unwrap();

//     if let Ok(mut commit) = set.add_opt("-q=str") {
//         commit.add_alias("--", "query");
//         let id = commit.commit().unwrap();
//         parser.set_callback(
//             id,
//             OptCallback::Value(Box::new(SimpleValueCallback::new(move |opt: &dyn opt::Opt| {
//                 dbg!("got a opt: ", opt);
//                 let mut writer = ref_count.lock().unwrap();
//                 *writer = 42;
//                 Ok(true)
//             }))),
//         );
//     }

//     if let Ok(mut commit) = set.add_opt("-f=bool") {
//         commit.add_alias("--", "force");
//         commit.commit().unwrap();
//     }

//     if let Ok(mut commit) = set.add_opt("-k=array") {
//         commit.add_alias("--", "keyword");
//         commit.commit().unwrap();
//     }

//     if let Ok(mut commit) = set.add_opt("-id=int") {
//         commit.add_alias("--", "identifier");
//         commit.commit().unwrap();
//     }

//     if let Ok(mut commit) = set.add_opt("-i=path") {
//         commit.add_alias("--", "import");
//         commit.set_deafult_value(getopt_rs::opt::OptValue::from_any(Box::new(
//             std::path::PathBuf::from("E:\\rust"),
//         )));
//         commit.commit().unwrap();
//     }

//     if let Ok(mut commit) = set.add_opt("operator=pos@-1") {
//         let id = commit.commit().unwrap();
//         parser.set_callback(
//             id,
//             OptCallback::from_index(Box::new(SimpleIndexCallback::new(
//                 |set: &dyn Set, arg: &String| {
//                     println!("In pos Meeting {:?}", arg);
//                     Ok(true)
//                 },
//             ))),
//         )
//     }

//     if let Ok(mut commit) = set.add_opt("mysql=cmd") {
//         let id = commit.commit().unwrap();
//         parser.set_callback(
//             id,
//             OptCallback::from_main(Box::new(SimpleMainCallback::new(
//                 |set: &dyn Set, args: &Vec<String>| {
//                     println!("In cmd Meeting {:?}", args);
//                     Ok(true)
//                 },
//             ))),
//         )
//     }

//     if let Ok(mut commit) = set.add_opt("main=main") {
//         let id = commit.commit().unwrap();
//         parser.set_callback(
//             id,
//             OptCallback::from_main(Box::new(SimpleMainCallback::new(
//                 |set: &dyn Set, args: &Vec<String>| {
//                     println!("In main Meeting {:?} ", args);
//                     Ok(true)
//                 },
//             ))),
//         )
//     }

//     set.subscribe_from(&mut parser);
//     parser.publish_to(Box::new(set));

//     let mut ai = ArgIterator::new();

//     ai.set_args(
//         &mut [
//             "mysql",
//             "--query",
//             "bar",
//             "--force",
//             "-id",
//             "-123",
//             "-i",
//             "E:\\rust\\getopt",
//             "-k",
//             "we",
//             "--keyword",
//             "are",
//             "submit",
//         ]
//         .iter()
//         .map(|&s| String::from(s)),
//     );
    
//     let ret = parser.parse(&mut ai);

//     dbg!(ret);
// }


extern crate getopt_rs;

use getopt_rs::prelude::*;
use simplelog::*;
use std::sync::Arc;
use std::sync::Mutex;
use getopt_rs::tools;

#[async_std::main]
async fn main() {
    CombinedLogger::init(vec![
        SimpleLogger::new(LevelFilter::Warn, Config::default()),
        SimpleLogger::new(LevelFilter::Error, Config::default()),
        SimpleLogger::new(LevelFilter::Debug, Config::default()),
        SimpleLogger::new(LevelFilter::Info, Config::default()),
    ])
    .unwrap();

    #[cfg(feature="async")]
    example3().await;

    #[cfg(not(feature="async"))]
    example3();
}

#[cfg(not(feature="async"))]
fn example3() {
    use getopt_rs::callback::*;

    let cache: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(vec![]));
    let mut set = DefaultSet::new();
    let mut parser = tools::forward_parse(0);

    set.initialize_prefixs();
    set.initialize_utils().unwrap();

    if let Ok(mut commit) = set.add_opt("-d=bool") {
        commit.set_help("filter the file are directory");
        let id = commit.commit().unwrap();
        let cache_ref = cache.clone();
        parser.set_callback(
            id,
            OptCallback::Value(Box::new(SimpleValueCallback::new(move |_| {
                let mut writer = cache_ref.lock().unwrap();
                let ret = (*writer)
                    .iter()
                    .filter(|&v| std::path::Path::new(v.as_str()).is_dir())
                    .map(|v| v.clone())
                    .collect();
                *writer = ret;
                Ok(true)
            }))),
        );
    }
    if let Ok(mut commit) = set.add_opt("-f=bool") {
        commit.set_help("filter the file are normal file");
        let id = commit.commit().unwrap();
        let cache_ref = cache.clone();
        parser.set_callback(
            id,
            OptCallback::Value(Box::new(SimpleValueCallback::new(move |_| {
                let mut writer = cache_ref.lock().unwrap();
                let ret = (*writer)
                    .iter()
                    .filter(|&v| std::path::Path::new(v.as_str()).is_file())
                    .map(|v| v.clone())
                    .collect();
                *writer = ret;
                Ok(true)
            }))),
        );
    }
    if let Ok(mut commit) = set.add_opt("-l=bool") {
        commit.set_help("filter the file are symbol link");
        let id = commit.commit().unwrap();
        let cache_ref = cache.clone();
        parser.set_callback(
            id,
            OptCallback::Value(Box::new(SimpleValueCallback::new(move |_| {
                let mut writer = cache_ref.lock().unwrap();
                let ret = (*writer)
                    .iter()
                    .filter(|&v| std::path::Path::new(v.as_str()).read_link().is_ok())
                    .map(|v| v.clone())
                    .collect();
                *writer = ret;
                Ok(true)
            }))),
        );
    }
    if let Ok(mut commit) = set.add_opt("-s=uint") {
        commit.set_help("filter the file are large than size");
        let id = commit.commit().unwrap();
        let cache_ref = cache.clone();
        parser.set_callback(
            id,
            OptCallback::Value(Box::new(SimpleValueCallback::new(move |opt| {
                let mut writer = cache_ref.lock().unwrap();
                let ret = (*writer)
                    .iter()
                    .filter(|&v| {
                        let metadata = std::fs::metadata(v).unwrap();
                        metadata.len() > *opt.value().as_uint().unwrap()
                    })
                    .map(|v| v.clone())
                    .collect();
                *writer = ret;
                Ok(true)
            }))),
        );
    }
    if let Ok(mut commit) = set.add_opt("directory=pos@1") {
        commit.set_help("search the given directory");
        let id = commit.commit().unwrap();
        let cache_ref = cache.clone();
        parser.set_callback(
            id,
            OptCallback::Index(Box::new(SimpleIndexCallback::new(move |_, v| {
                let mut writer = cache_ref.lock().unwrap();
                for entry in std::fs::read_dir(v).unwrap() {
                    let entry = entry.unwrap();

                    (*writer).push(entry.path().to_str().unwrap().to_owned());
                }
                Ok(true)
            }))),
        );
    }
    if let Ok(mut commit) = set.add_opt("main=main") {
        commit.set_help("main function");
        let id = commit.commit().unwrap();
        let cache_ref = cache.clone();
        parser.set_callback(
            id,
            OptCallback::Main(Box::new(SimpleMainCallback::new(move |_, noa| {
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
            }))),
        );
    }

    getopt!(parser, set).unwrap();
}

#[cfg(feature="async")]
async fn example3() {

    use getopt_rs::callback::*;

    let cache: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(vec![]));
    let mut set = DefaultSet::new();
    let mut parser = tools::forward_parse(0);

    set.initialize_prefixs();
    set.initialize_utils().unwrap();

    #[derive(Debug)]
    struct AsyncValueCallback(Arc<Mutex<Vec<String>>>);

    #[async_trait::async_trait(?Send)]
    impl ValueCallback for AsyncValueCallback {
        async fn call(&mut self, opt: &dyn Opt) -> Result<bool> {
            let mut writer = self.0.lock().unwrap();
            let ret = match opt.name() {
                "d" => {
                    (*writer).iter()
                    .filter(|&v|{ std::path::Path::new(v.as_str()).is_dir()})
                    .map(|v| { v.clone() })
                    .collect()
                }
                "f" => {
                    (*writer).iter()
                    .filter(|&v|{ std::path::Path::new(v.as_str()).is_file()})
                    .map(|v| { v.clone() })
                    .collect()
                }
                "l" => {
                    (*writer).iter()
                    .filter(|&v|{ std::path::Path::new(v.as_str()).read_link().is_ok()})
                    .map(|v| { v.clone() })
                    .collect()
                }
                "s" => {
                    (*writer).iter()
                    .filter(|&v|{ std::fs::metadata(v).unwrap().len() > *opt.value().as_uint().unwrap() })
                    .map(|v| { v.clone() })
                    .collect()
                }
                _ => {
                    panic!("Unknow option name!")
                }
            };
            *writer = ret;
            Ok(true)
        }
    }

    if let Ok(mut commit) = set.add_opt("-d=bool") {
        let id = commit.commit().unwrap();
        parser.set_callback(id, OptCallback::Value(Box::new(AsyncValueCallback(cache.clone()))));
    }
    if let Ok(mut commit) = set.add_opt("-f=bool") {
        let id = commit.commit().unwrap();
        parser.set_callback(id, OptCallback::Value(Box::new(AsyncValueCallback(cache.clone()))));
    }
    if let Ok(mut commit) = set.add_opt("-l=bool") {
        let id = commit.commit().unwrap();
        parser.set_callback(id, OptCallback::Value(Box::new(AsyncValueCallback(cache.clone()))));
    }
    if let Ok(mut commit) = set.add_opt("-s=uint") {
        let id = commit.commit().unwrap();
        parser.set_callback(id, OptCallback::Value(Box::new(AsyncValueCallback(cache.clone()))));
    }

    #[derive(Debug)]
    struct AsyncIndexCallback(Arc<Mutex<Vec<String>>>);

    #[async_trait::async_trait(?Send)]
    impl<T: Proc, S: Set<T>> IndexCallback<T, S> for AsyncIndexCallback {
        async fn call(&mut self, _: &S, v: &String) -> Result<bool> {
            let mut writer = self.0.lock().unwrap();
            for entry in std::fs::read_dir(v).unwrap() {
                let entry = entry.unwrap();

                (*writer).push(entry.path().to_str().unwrap().to_owned());
            }
            Ok(true)
        }
    }
    if let Ok(mut commit) = set.add_opt("directory=pos@1") {
        let id = commit.commit().unwrap();
        parser.set_callback(id, OptCallback::Index(Box::new(AsyncIndexCallback(cache.clone()))));
    }

    #[derive(Debug)]
    struct AsyncMainCallback(Arc<Mutex<Vec<String>>>);

    #[async_trait::async_trait(?Send)]
    impl<T: Proc, S: Set<T>> MainCallback<T, S> for AsyncMainCallback {
        async fn call(&mut self, _: &S, noa: &Vec<String>) -> Result<bool> {
            let mut regex: Option<regex::Regex> = None;

            if noa.len() == 2 {
                regex = regex::Regex::new(noa[1].as_str()).ok();
            }
            for file in self.0.lock().unwrap().iter() {
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
    }
    if let Ok(mut commit) = set.add_opt("main=main") {
        let id = commit.commit().unwrap();
        parser.set_callback(id, OptCallback::Main(Box::new(AsyncMainCallback(cache.clone()))));
    }

    getopt!(parser, set).await.unwrap();
}
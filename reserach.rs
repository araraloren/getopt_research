
pub fn debug(s: String) {
    println!("{}", s);
}

mod processor {
    use opts::Opt;
    use context::Context;
    use super::debug;

    pub trait Message {
        type T;

        fn msg_id(&self) -> u64;
        
        fn data(&mut self) -> &mut Self::T;
    }

    pub trait Info<M: Message>: std::fmt::Debug {
        fn opt_id(&self) -> u64;

        fn check(&self, msg: &M) -> bool;

        fn process(&mut self, data: &mut M::T, opt: &mut dyn Opt);
    }

    pub trait Publisher<M: Message> {
        fn publish(&mut self, msg: M);

        fn subscribe(&mut self, info: Box<dyn Info<M>>);

        fn clean(&mut self);
    }

    pub trait Subscribe<M: Message> {
        fn subscribe(&self, publisher: &mut dyn Publisher<M>);
    }

    #[derive(Debug)]
    pub struct ContextProcessor {
        id: u64,
        context: Vec<Box<dyn Context<ContextProcessor, ContextProcessor>>>,
    }

    impl ContextProcessor {
        pub fn new(id: u64) -> Self {
            Self {
                id,
                context: vec![],
            }
        }

        pub fn add_context(&mut self, context: Box<dyn Context<ContextProcessor, ContextProcessor>>) {
            debug(format!("In func ContextProcessor.add_context"));
            self.context.push(context);
        }

        pub fn process(&self, opt: &mut dyn Opt) {
            debug(format!("In func ContextProcessor.process"));
            for context in &self.context {
                if context.match_opt(self, opt) {
                    context.set_opt(self, opt);
                }
            }
        }
    }

    impl Message for ContextProcessor {
        type T = ContextProcessor;

        fn msg_id(&self) -> u64 {
            self.id
        }
    
        fn data(&mut self) -> &mut ContextProcessor {
            self
        }
    }
}

mod opts {
    use super::debug;
    use processor::Info;
    use processor::Subscribe;
    use processor::Publisher;
    use processor::ContextProcessor;

    pub trait Identifer {
        fn id(&self) -> u64;
    }

    pub trait Name {
        fn name(&self) -> &str;
    
        fn match_name(&self, s: &str) -> bool;
    }
    
    pub trait Prefix {
        fn prefix(&self) -> &str;
    
        fn match_prefix(&self, s: &str) -> bool;
    }
    
    pub trait Optional {
        fn optional(&self) -> bool;
    
        fn match_optional(&self, b: bool) -> bool;
    }

    pub trait Opt: 
        Name + 
        Prefix + 
        Optional + 
        Identifer +
        std::fmt::Debug +
        Subscribe<ContextProcessor>
        { }

    pub struct Creator(pub Box<dyn Fn(u64, &'static str, &'static str) -> Box<dyn Opt>>);

    impl std::fmt::Debug for Creator {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("Creator")
                .field("Func", &"...")
                .finish()
        }
    }

    pub fn str_creator(id: u64, name: &'static str, prefix: &'static str) -> Box<dyn Opt> {
        Box::new(
            Str::new(id, name.to_owned(), prefix.to_owned(), true)
        )
    }

    #[derive(Debug)]
    pub struct Str {
        id: u64,
        name: String,
        prefix: String,
        optional: bool,
    }
    
    impl Str {
        pub fn new(id: u64, name: String, prefix: String, optional: bool) -> Self {
            Str {
                id, name, prefix, optional
            }
        }
    }

    impl Opt for Str { }

    impl Identifer for Str {
        fn id(&self) -> u64 {
            self.id
        }
    }

    impl Name for Str {
        fn name(&self) -> &str {
            &self.name
        }

        fn match_name(&self, s: &str) -> bool {
            self.name() == s
        }
    }

    impl Prefix for Str {
        fn prefix(&self) -> &str {
            &self.prefix
        }

        fn match_prefix(&self, s: &str) -> bool {
            self.prefix() == s
        }
    }

    impl Optional for Str {
        fn optional(&self) -> bool {
            self.optional
        }

        fn match_optional(&self, b: bool) -> bool {
            self.optional() == b
        }
    }

    #[derive(Debug)]
    pub struct StrInfo {
        id: u64,
    }

    impl StrInfo {
        pub fn new(id: u64) -> Self {
            Self {
                id,
            }
        }
    }

    impl Info<ContextProcessor> for StrInfo {
        fn opt_id(&self) -> u64 {
            self.id
        }

        fn check(&self, msg: &ContextProcessor) -> bool {
            debug(format!("In func Info.check"));
            true
        }

        fn process(&mut self, data: &mut ContextProcessor, opt: &mut dyn Opt) {
            debug(format!("In func Info.process"));
            data.process(opt);
        }
    }

    impl Subscribe<ContextProcessor> for Str {
        fn subscribe(&self, publisher: &mut dyn Publisher<ContextProcessor>) {
            debug(format!("In func Subscriber.subscribe"));
            publisher.subscribe(Box::new(StrInfo::new(self.id())))
        }
    }
}

mod context {
    use processor::Message;
    use opts::Opt;
    
    pub trait Context<T, M: Message>: std::fmt::Debug {
        fn match_opt(&self, cp: &M, opt: &mut dyn Opt) -> bool;
    
        fn set_opt(&self, cp: &M, opt: &mut dyn Opt);
    
        fn set_success(&mut self);
    
        fn is_success(&self);
    }

    pub struct ArgGetter(pub Option<Box<dyn Fn() -> String>>);

    impl std::fmt::Debug for ArgGetter {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("ArgGetter")
             .field("Func", &"...")
             .finish()
        }
    }

    pub mod option {
        use super::super::debug;
        use opts::Opt;
        use super::ArgGetter;
        use super::Context as TC;
        use processor::ContextProcessor;

        #[derive(Debug)]
        pub struct Context {
            prefix: String,
            name: String,
            argument: bool,
            getter: ArgGetter,
            skip: bool,
            success: bool,
        }

        impl<'a, 'b> Context {
            pub fn new(prefix: &'a str, name: &'b str, argument: bool, getter: ArgGetter, skip: bool) -> Self {
                Context {
                    prefix: String::from(prefix), name: String::from(name), argument, getter, skip, success: false,
                }
            }
        }

        impl TC<ContextProcessor, ContextProcessor> for Context {
            fn match_opt(&self, cp: &ContextProcessor, opt: &mut dyn Opt) -> bool {
                debug(format!("In func Context.match_opt"));
                if self.prefix == opt.prefix() {
                    if self.name == opt.name() {
                        return true;
                    }
                }
                false
            }
    
            fn set_opt(&self, cp: &ContextProcessor, opt: &mut dyn Opt) {
                debug(format!("In func Context.set_opt {:?}", opt));
            }
        
            fn set_success(&mut self) {

            }
        
            fn is_success(&self) {

            }
        }
    }
}

mod optset {
    use opts::Opt;
    use opts::Creator;
    use processor::Info;
    use context::ArgGetter;
    use processor::Publisher;
    use std::collections::HashMap;
    use processor::ContextProcessor;
    use context::option::Context as OptContext;

    #[derive(Debug)]
    pub struct OptSet {
        opts: Vec<Box<dyn Opt>>,
        creators: HashMap<&'static str, Creator>,
    }
    
    impl OptSet {
        pub fn new() -> OptSet {
            OptSet {
                opts: vec![],
                creators: HashMap::new(),
            }
        }

        pub fn add_creator(&mut self, s: &'static str, creator: Creator) {
            self.creators.insert(s, creator);
        }

        pub fn add_str_opt(&mut self, id: u64, name: &'static str, prefix: &'static str) {
            self.opts.push( self.creators["s"].0(id, name, prefix) )
        }

        pub fn get_opt(&self, id: u64) -> Option<&dyn Opt> {
            for opt in &self.opts {
                if opt.id() == id {
                    return Some(opt.as_ref())
                }
            }
            None
        }

        pub fn get_opt_mut(&mut self, id: u64) -> Option<&mut dyn Opt> {
            for opt in &mut self.opts {
                if opt.id() == id {
                    return Some(opt.as_mut())
                }
            }
            None
        }

        pub fn subscribe(&self, publisher: &mut dyn Publisher<ContextProcessor>) {
            for opt in &self.opts {
                opt.subscribe(publisher);
            }
        }

        pub fn len(&self) -> usize {
            self.opts.len()
        }
    }

    #[derive(Debug)]
    pub struct Parser {
        msg_id_counter: u64,
        os: Option<OptSet>,
        info: Vec<Box<dyn Info<ContextProcessor>>>,
    }

    impl Publisher<ContextProcessor> for Parser {
        fn publish(&mut self, msg: ContextProcessor) {
            let mut msg = msg;

            println!("got a cp => {:?}", msg);
            for index in 0 .. self.info.len() {
                let info = self.info.get_mut(index).unwrap();
                let opt = self.os.as_mut().unwrap().get_opt_mut(info.opt_id());

                if info.check(&msg) {
                    info.process(&mut msg, opt.unwrap());
                }
            }
        }

        fn subscribe(&mut self, info: Box<dyn Info<ContextProcessor>>) {
            self.info.push(info);
        }

        fn clean(&mut self) {
            self.info.clear();
        }
    }

    impl Parser {
        pub fn new() -> Self {
            Self {
                msg_id_counter: 0,
                os: None,
                info: vec![],
            }
        }

        pub fn set_optset(&mut self, os: OptSet) {
            self.os = Some(os);
        }

        pub fn parse(&mut self, args: &[&str]) {
            for arg in args {
                let mut cp = ContextProcessor::new(self.msg_id_counter);

                self.msg_id_counter += 1;
                cp.add_context(Box::new(OptContext::new("", arg, false, ArgGetter(None), false)));
                self.publish(cp);
            }
        }
    }
}


fn main() {
    use optset::*;
    use opts::Creator;

    let mut os = OptSet::new();

    os.add_creator("s", Creator(Box::new(opts::str_creator)));
    os.add_str_opt(1, "a", "");
    os.add_str_opt(2, "b", "");

    let mut parser = Parser::new();

    os.subscribe(&mut parser);
    parser.set_optset(os);
    parser.parse(&["a", "b", "c"]);
}

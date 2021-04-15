// use std::collections::HashMap;

// #[derive(Debug)]
// struct S {
//     name: String,
//     prefix: String,
// }

// trait Name {
//     fn name(&self) -> &String;

//     fn match_name(&self, s: &String) -> bool;
// }

// trait Prefix {
//     fn prefix(&self) -> &String;

//     fn match_prefix(&self, s: &String) -> bool;
// }

// trait Parser {
//     fn parse(&self, s: &String) -> bool;
// }

// trait Opt: Name + Prefix + Parser { }

// trait Int: Opt { }

// impl Int for S { }

// impl Opt for S { }

// impl Name for S {
//     fn name(&self) -> &String {
//         &self.name
//     }

//     fn match_name(&self, s: &String) -> bool {
//         self.name() == s
//     }
// }

// impl Prefix for S {
//     fn prefix(&self) -> &String {
//         &self.prefix
//     }

//     fn match_prefix(&self, s: &String) -> bool {
//         self.prefix() == s
//     }
// }

// impl Parser for S {
//     fn parse(&self, s: &String) -> bool {
//         println!("match {:?} with {}", self, s);
//         self.match_name(&s)
//     }
// }

// type Creator = Box<dyn Fn(&'static str) -> Box<dyn Opt>>;

// fn int_creator(name: &'static str) -> Box<dyn Opt> {
//     Box::new(S { name: name.to_owned(), prefix: "-".to_owned() })
// }

// struct Set {
//     opts: Vec<Box<dyn Opt>>,
//     creators: HashMap<&'static str, Creator>,
// }

// impl Set {
//     fn new() -> Self {
//         Self {
//             opts: vec![],
//             creators: HashMap::new(),
//         }
//     }

//     fn add(&mut self, s: &'static str) {
//         self.opts.push( self.creators["i"](s) )
//     }

//     fn parse(&mut self, args: &[&str]) {
//         for opt in &mut self.opts {
//             for arg in args {
//                 opt.parse(&String::from(*arg));
//             }
//         }
//     }
// }

// fn main() {
//     let mut set = Set::new();

//     set.creators.insert("i", Box::new(int_creator));
//     set.add("a");

//     set.parse(&["a", "b", "c"]);
// }

// trait Opt { }

// trait Int : Opt { }

// trait Str : Opt { }

// trait Flt : Opt { }

// trait Array : Opt { }

// trait Bool : Opt { }

// struct Set { }

// Opt<
//     Name, // "count"
//     Prefix, // "-"
//     Value, // 42
//     Callable, // |&Opt| { }
//     Optional, // true
//     Helper, // "count of ..."
//     DefaultValue, // 0
//     Match, // |&Info| -> bool { }
//     SetRef, // .setref() ?
//     Parser, // |&Info| { } ?
//     Setter, //
// >

// Cmd<
//     Name, // "count"
//     Index, // 0
//     Value, // "count"
//     Callable, // |&Cmd| { }
//     Optional, // true
//     Helper, // "count cmd"
//     Match, // |&Info| -> bool { }
//     SetRef, // .setref() ?
//     Parser, // |&Info| { } ?
//     Setter, //
// >

// Register<Creator, Parser>

// trait Opt: std::fmt::Debug {
//     fn name(&self) -> &String;
// }

// trait Int: Opt {
//     fn as_int(&self) -> i32;
// }

// impl Opt for Struct {
//     fn name(&self) -> &String {
//         &self.0
//     }
// }

// impl Int for Struct {
//     fn as_int(&self) -> i32 {
//         42
//     }
// }

mod ctx;
mod opt;
mod proc;
mod utils;
mod str;
mod err;
mod set;
mod parser;

fn main() {
    println!("{:?}", utils::CreatorInfo::new("a=c").unwrap());
    println!("{:?}", utils::CreatorInfo::new("a=c!").unwrap());
    println!("{:?}", utils::CreatorInfo::new("a=c!/").unwrap());
    println!("{:?}", utils::CreatorInfo::new("a=c/").unwrap());
    println!("{:?}", utils::CreatorInfo::new("a=c/!").unwrap());
    println!("{:?}", utils::CreatorInfo::new("count=c").unwrap());

    let mut ci = utils::CommandInfo::new(vec![
        String::from(""),
        String::from("-"),
        String::from("-/"),
        String::from("--"),
        String::from("--/"),
        ]);

    if ci.parse("-a") {
        println!("{:?}", ci);
    }
    if ci.parse("-a=b") {
        println!("{:?}", ci);
    }
    if ci.parse("-abcd") {
        println!("{:?}", ci);
    }
    if ci.parse("-/a") {
        println!("{:?}", ci);
    }
    if ci.parse("--abcd") {
        println!("{:?}", ci);
    }
    if ci.parse("--/abcd") {
        println!("{:?}", ci);
    }
    if ci.parse("--abcd=1") {
        println!("{:?}", ci);
    }
    if ci.parse("a") {
        println!("{:?}", ci);
    }
    if ci.parse("abcd") {
        println!("{:?}", ci);
    }
}

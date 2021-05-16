
use crate::proc::Proc;
use crate::proc::SequenceProc;
use crate::proc::Publisher;
use crate::proc::Info;
use crate::set::Set;
use crate::opt::Opt;
use crate::opt::Style;
use crate::opt::OptValue;
use crate::callback::OptCallback;
use crate::callback::CallbackType;
use crate::id::IdGenerator;
use crate::id::Identifier;
use crate::ctx::Context;
use crate::ctx::OptContext;
use crate::ctx::NonOptContext;
use crate::arg::IndexIterator;
use crate::arg::Argument;
use crate::error::Result;
use crate::error::Error;

use std::fmt::Debug;
use std::collections::HashMap;

pub trait Parser: Debug + Publisher<Box<dyn Proc>> {
    fn parse(&mut self, iter: &mut dyn IndexIterator) -> Result<Option<ReturnValue>>;

    fn publish_to(&mut self, set: Box<dyn Set>);

    fn set_id_generator(&mut self, id_generator: Box<dyn IdGenerator>);

    fn set_callback(&mut self, id: Identifier, callback: OptCallback);

    fn set(&self) -> &Option<Box<dyn Set>>;

    fn get_opt(&self, id: Identifier) -> Option<& dyn Opt>;

    fn get_opt_mut(&mut self, id: Identifier) -> Option<&mut dyn Opt>;

    fn noa(&self) -> &Vec<String>;

    fn check_opt(&self) -> Result<bool>;

    fn check_nonopt(&self) -> Result<bool>;

    fn check_other(&self) -> Result<bool>;

    fn reset(&mut self);
}

#[derive(Debug)]
pub struct ReturnValue<'a> {
    pub noa: Vec<String>,

    pub set: &'a dyn Set,
}

impl<'a> ReturnValue<'a> {
    pub fn new(noa: &Vec<String>, set: &'a dyn Set) -> Self {
        Self {
            noa: noa.clone(),
            set,
        }
    }
}

#[derive(Debug)]
pub struct ForwardParser {
    msg_id_gen: Box<dyn IdGenerator>,

    matched: bool,

    cached_infos: Vec<Box<dyn Info>>,

    noa: Vec<String>,

    set: Option<Box<dyn Set>>,

    argument_matched: bool,

    callbacks: HashMap<Identifier, OptCallback>,
}

impl ForwardParser {
    pub fn new(msg_id_gen: Box<dyn IdGenerator>) -> Self {
        Self {
            msg_id_gen: msg_id_gen,
            matched: false,
            cached_infos: vec![],
            noa: vec![],
            set: None,
            argument_matched: false,
            callbacks: HashMap::new(),
        }
    }

    pub fn set_argument_matched(&mut self) {
        self.argument_matched = true;
    }

    pub fn get_prefix(&self) -> &Vec<String> {
        self.set.as_ref().unwrap().get_prefix()
    }
}

impl Parser for ForwardParser {
    fn parse(&mut self, iter: &mut dyn IndexIterator) -> Result<Option<ReturnValue>> {
        if self.set.is_none() {
            return Ok(None);
        }

        debug!("---- In ForwardParser, start process option");

        while ! iter.reach_end() {
            let mut matched = false;

            iter.fill_current_and_next();
            self.argument_matched = false;
            debug!("**** ArgIterator [{:?}, {:?}]", iter.current(), iter.next());

            if let Ok(arg) = iter.parse(self.get_prefix()) {

                debug!("parse ... {:?}", arg);

                if ! matched {
                    let multiple_ctx = parser_gen_argument_style(&arg, iter.next());

                    if multiple_ctx.len() > 0 {
                        let mut cp = Box::new(SequenceProc::new(self.msg_id_gen.next_id()));

                        for ctx in multiple_ctx {
                            cp.append_ctx(ctx);
                        }

                        matched = self.publish(cp)?;
                    }
                }
                if ! matched {
                    let multiple_ctx = parser_gen_multiple_style(&arg, &None);

                    if multiple_ctx.len() > 0 {
                        let mut cp = Box::new(SequenceProc::new(self.msg_id_gen.next_id()));

                        for ctx in multiple_ctx {
                            cp.append_ctx(ctx);
                        }

                        matched = self.publish(cp)?;
                    }
                }
                if ! matched {
                    let multiple_ctx = parser_gen_boolean_style(&arg, &None);

                    if multiple_ctx.len() > 0 {
                        let mut cp = Box::new(SequenceProc::new(self.msg_id_gen.next_id()));

                        for ctx in multiple_ctx {
                            cp.append_ctx(ctx);
                        }

                        matched = self.publish(cp)?;
                    }
                }
            }

            // If next argument matched, skip it
            if matched && self.argument_matched {
                iter.skip();
            }
            if !matched {
                if let Some(arg) = iter.current() {
                    self.noa.push(arg.clone());
                }
            }

            iter.skip();
        }

        self.check_opt()?;

        let noa_total = self.noa().len();

        // process cmd and pos
        if noa_total > 0 {
            debug!("---- In ForwardParser, start process cmd");
            if let Some(ctx) = parser_gen_cmd_style(&self.noa()[0], noa_total as i64, 1) {
                let mut cp = Box::new(SequenceProc::new(self.msg_id_gen.next_id()));

                cp.append_ctx(ctx);
                self.publish(cp)?;
            }

            debug!("---- In ForwardParser, start process pos");
            for index in 1 ..= noa_total {
                if let Some(ctx) = parser_gen_pos_style(&self.noa()[index - 1], noa_total as i64, index as i64) {
                    let mut cp = Box::new(SequenceProc::new(self.msg_id_gen.next_id()));

                    cp.append_ctx(ctx);
                    self.publish(cp)?;
                }
            }
        }

        self.check_nonopt()?;

        debug!("---- In ForwardParser, start process main");
        if let Some(ctx) = parser_gen_main_style(&String::new(), 0, 0) {
            let mut cp = Box::new(SequenceProc::new(self.msg_id_gen.next_id()));

            cp.append_ctx(ctx);
            self.publish(cp)?;
        }

        self.check_other()?;

        Ok(Some(ReturnValue::new(&self.noa, self.set.as_ref().unwrap().as_ref())))
    }

    fn set_id_generator(&mut self, id_generator: Box<dyn IdGenerator>) {
        self.msg_id_gen = id_generator;
    }

    fn set_callback(&mut self, id: Identifier, callback: OptCallback) {
        self.callbacks.insert(id, callback);
    }

    fn publish_to(&mut self, set: Box<dyn Set>) {
        self.set = Some(set);
    }

    fn set(&self) -> &Option<Box<dyn Set>> {
        &self.set
    }

    fn get_opt(&self, id: Identifier) -> Option<& dyn Opt> {
        self.set.as_ref().unwrap().get_opt(id)
    }

    fn get_opt_mut(&mut self, id: Identifier) -> Option<&mut dyn Opt> {
        self.set.as_mut().unwrap().get_opt_mut(id)
    }

    fn noa(&self) -> &Vec<String> {
        &self.noa
    }

    fn check_opt(&self) -> Result<bool> {
        parser_default_opt_check(self.set.as_ref().unwrap().as_ref())
    }

    fn check_nonopt(&self) -> Result<bool> {
        parser_default_nonopt_check(self.set.as_ref().unwrap().as_ref())
    }

    fn check_other(&self) -> Result<bool> {
        Ok(true)
    }

    fn reset(&mut self) {
        self.noa.clear();
        self.set.as_mut().unwrap().reset();
        self.argument_matched = false;
    }
}

impl Publisher<Box<dyn Proc>> for ForwardParser {
    fn publish(&mut self, msg: Box<dyn Proc>) -> Result<bool> {
        let mut proc = msg;

        debug!("Receive msg<{:?}> => {:?}", &proc.id(), &proc);

        for index in 0 .. self.cached_infos.len() {
            let info = self.cached_infos.get_mut(index).unwrap();
            let opt = self.set.as_mut().unwrap().get_opt_mut(info.id()).unwrap(); // id always exist, so just unwrap
            let res = proc.process(opt)?;
            let need_invoke = opt.is_need_invoke();
            let callback_type = opt.callback_type();

            if res {
                if need_invoke {
                    opt.set_need_invoke(false);
                    if let Some(callback) = self.callbacks.get_mut(&opt.id()) {
                        debug!("!!!! Calling callback of {:?}", info.id());
                        match callback_type {
                            CallbackType::Value => {
                                callback.call_value(opt)?;
                            }
                            CallbackType::Index => {
                                let length = self.noa.len();
                                let index = opt.index().calc_index(length as i64);

                                if let Some(index) = index {
                                    let ret = callback.call_index(self.set.as_ref().unwrap().as_ref(), &self.noa[index as usize])?;
                                    // can we fix this long call?
                                    self.set.as_mut().unwrap().get_opt_mut(info.id()).unwrap().set_value(OptValue::from_bool(ret));
                                }
                            }
                            CallbackType::Main => {
                                let ret = callback.call_main(self.set.as_ref().unwrap().as_ref(), &self.noa)?;
                                // can we fix this long call?
                                self.set.as_mut().unwrap().get_opt_mut(info.id()).unwrap().set_value(OptValue::from_bool(ret));
                            }
                            _ => { }
                        }
                    }
                }
                if proc.is_need_argument() {
                    self.set_argument_matched();
                }
            }
        }

        Ok(proc.is_matched())
    }

    fn reg_subscriber(&mut self, info: Box<dyn Info>) {
        self.cached_infos.push(info);
    }

    fn clean(&mut self) {
        self.cached_infos.clear();
    }
}

pub fn parser_gen_argument_style(arg: &Argument, next_argument: &Option<String>) -> Vec<Box<dyn Context>> {
    let mut ret: Vec<Box<dyn Context>> = vec![];
    let default_value = String::default();

    match arg.get_value() {
        Some(value) => {
            ret.push(Box::new(OptContext::new(
                arg.get_prefix().unwrap_or(&default_value).clone(),
                arg.get_name().unwrap_or(&default_value).clone(),
                Some(value.clone()),
                Style::Argument,
                false,
            )))
        }
        None => {
            ret.push(Box::new(OptContext::new(
                arg.get_prefix().unwrap_or(&default_value).clone(),
                arg.get_name().unwrap_or(&default_value).clone(),
                next_argument.clone(),
                Style::Argument,
                true,
            )));
            if let Some(name) = arg.get_name() {
                if name.len() >= 2 {
                    let name_and_value = name.split_at(1);

                    ret.push(Box::new(OptContext::new(
                        arg.get_prefix().unwrap_or(&default_value).clone(),
                        name_and_value.0.to_owned(),
                        Some(name_and_value.1.to_owned()),
                        Style::Argument,
                        false,
                    )))
                }
            }
        }
    }
    ret
}

pub fn parser_gen_multiple_style(arg: &Argument, _: &Option<String>) -> Vec<Box<dyn Context>> {
    let mut ret: Vec<Box<dyn Context>> = vec![];

    if arg.get_value().is_none() {
        if arg.get_name().unwrap().len() > 1 {
            for char in arg.get_name().unwrap().chars() {
                ret.push(Box::new(OptContext::new(
                    arg.get_prefix().unwrap().clone(),
                    String::from(char),
                    None,
                    Style::Multiple,
                    false,
                )))
            }
        }
    }
    ret
}

pub fn parser_gen_boolean_style(arg: &Argument, _:  &Option<String>) -> Vec<Box<dyn Context>> {
    let mut ret: Vec<Box<dyn Context>> = vec![];

    match arg.get_value() {
        Some(_) => { },
        None => {
            ret.push(Box::new(OptContext::new(
                arg.get_prefix().unwrap().clone(),
                arg.get_name().unwrap().clone(),
                None,
                Style::Boolean,
                false,
            )));
        }
    }    
    ret
}

pub fn parser_gen_pos_style(noa: &String, total: i64, current: i64)-> Option<Box<dyn Context>> {
    Some(Box::new(NonOptContext::new( noa.clone(), Style::Pos, total, current )))
}

pub fn parser_gen_cmd_style(noa: &String, total: i64, current: i64)-> Option<Box<dyn Context>> {
    Some(Box::new(NonOptContext::new( noa.clone(), Style::Cmd, total, current )))
}

pub fn parser_gen_main_style(noa: &String, total: i64, current: i64)-> Option<Box<dyn Context>> {
    Some(Box::new(NonOptContext::new( noa.clone(), Style::Main, total, current )))
}

/// This function will call function [`check`](crate::opt::Type::check)
/// of options which type is [`Style::Boolean`], [`Style::Argument`]
/// or [`Style::Multiple`]
pub fn parser_default_opt_check(set: &dyn Set) -> Result<bool> {
        for opt in set.iter() {
            if opt.as_ref().is_style(Style::Boolean) 
            && opt.as_ref().is_style(Style::Argument) 
            && opt.as_ref().is_style(Style::Multiple) {
                opt.check()?;
            }
        }
        Ok(true)
}


pub fn parser_default_nonopt_check(set: &dyn Set) -> Result<bool> {
    let len = i64::MAX;
    let mut index_map: HashMap<i64, Vec<Identifier>> = HashMap::new();

    for opt in set.iter() {
        if opt.as_ref().is_style(Style::Pos) && opt.as_ref().is_style(Style::Cmd) 
            && opt.as_ref().is_style(Style::Main) {
            let entry = index_map
                .entry(opt.as_ref().index().calc_index(len).unwrap())
                .or_insert(vec![]);

            entry.push(opt.as_ref().id());
        } 
    }
    let mut force_names= vec![];

    for item in index_map.iter() {
        let mut valid = false;

        // first pos is a special position
        if item.0 == &1 {
            let mut cmd_count = 0;
            let mut cmd_valid = false;
            let mut pos_valid = false;
            let mut force_valid = false;        

            for id in item.1.iter() {
                let opt = set.get_opt(*id).unwrap();
                
                if opt.is_style(Style::Cmd) {
                    cmd_count += 1;
                    cmd_valid = cmd_valid || opt.check().unwrap_or(false);
                }
                else if opt.is_style(Style::Pos) {
                    let valid = opt.check().unwrap_or(false);

                    pos_valid = pos_valid || valid;
                    if valid {
                        force_valid = force_valid || opt.has_value();
                    }
                }
                force_names.push(format!("`{}{}`", opt.prefix(), opt.name()));
            }

            if cmd_count > 0 {
                if item.1.len() > cmd_count {
                    valid = cmd_valid || force_valid;
                } 
                else {
                    valid = cmd_valid;
                }
            } 
            else {
                valid = pos_valid;
            }

            if !valid {
                return Err(Error::NonOptionForceRequired(force_names.join(" or ")));
            }
        }
        else {
            for id in item.1.iter() {
                let opt = set.get_opt(*id).unwrap();
                
                valid = valid || opt.check().unwrap_or(false);
                force_names.push(format!("`{}{}`", opt.prefix(), opt.name()));
            }
            if !valid {
                return Err(Error::NonOptionForceRequired(force_names.join(" or ")));
            }
        }
        force_names.clear();
    }        
    Ok(true)
}

#[cfg(test)]
mod tests {
    use core::panicking::assert_failed;

    use super::*;
    use crate::{arg::ArgIterator, set::*};
    use crate::id::DefaultIdGen;
    use crate::opt::*;
    use crate::nonopt::*;
    use crate::callback::*;

    #[test]
    fn make_sure_forwardparser_work() {
        let id = DefaultIdGen::default();
        let mut set = DefaultSet::new();
        let mut parser = ForwardParser::new(Box::new(id));

        assert!(set.add_utils(Box::new(bool::BoolUtils::new())).unwrap_or(false));
        assert!(set.add_utils(Box::new(str::StrUtils::new())).unwrap_or(false));
        assert!(set.add_utils(Box::new(array::ArrayUtils::new())).unwrap_or(false));
        assert!(set.add_utils(Box::new(pos::PosUtils::new())).unwrap_or(false));
        assert!(set.add_utils(Box::new(cmd::CmdUtils::new())).unwrap_or(false));
        assert!(set.add_utils(Box::new(main::MainUtils::new())).unwrap_or(false));

        set.app_prefix("+".to_owned());

        if let Ok(mut commit) = set.add_opt("-c=array") {
            commit.commit().unwrap();
        }
        if let Ok(mut commit) = set.add_opt("-h=array") {
            commit.commit().unwrap();
        }
        if let Ok(mut commit) = set.add_opt("-cpp=array") {
            commit.commit().unwrap();
        }
        if let Ok(mut commit) = set.add_opt("-cfg=array") {
            commit.commit().unwrap();
        }
        if let Ok(mut commit) = set.add_opt("-m=array") {
            commit.commit().unwrap();
        }
        if let Ok(mut commit) = set.add_opt("-w=array") {
            commit.commit().unwrap();
        }
        if let Ok(mut commit) = set.add_opt("+a=array") {
            commit.commit().unwrap();
        }
        if let Ok(mut commit) = set.add_opt("-no=array") {
            commit.commit().unwrap();
        }
        if let Ok(mut commit) = set.add_opt("-i=bool") {
            commit.commit().unwrap();
        }
        if let Ok(mut commit) = set.add_opt("-only=str") {
            commit.commit().unwrap();
        }
        if let Ok(mut commit) = set.add_opt("-d=bool") {
            commit.add_alias("--", "debug");
            commit.commit().unwrap();
        }

        fn directory(set: &dyn Set, noa: &Vec<String>) -> Result<bool> {
            assert!(set.filter("c").unwrap().find().unwrap()
                       .value().as_vec().unwrap()
                       .contains(&String::from("c")));
            assert!(set.filter("cpp").unwrap().find().unwrap()
                       .value().as_vec().unwrap()
                       .contains(&String::from("cpp")));
            assert!(set.filter("cpp").unwrap().find().unwrap()
                       .value().as_vec().unwrap()
                       .contains(&String::from("c++")));
            assert!(set.filter("cpp").unwrap().find().unwrap()
                       .value().as_vec().unwrap()
                       .contains(&String::from("cxx")));
            assert!(set.filter("h").unwrap().find().unwrap()
                       .value().as_vec().unwrap()
                       .contains(&String::from("h")));
            assert!(set.filter("h").unwrap().find().unwrap()
                       .value().as_vec().unwrap()
                       .contains(&String::from("hpp")));
            assert!(set.filter("h").unwrap().find().unwrap()
                       .value().as_vec().unwrap()
                       .contains(&String::from("hxx")));
            assert!(set.filter("cfg").unwrap().find().unwrap()
                       .value().as_str().unwrap()
                       .eq("cpp"));
            assert!(set.filter("m").unwrap().find().unwrap()
                       .value().as_vec().unwrap()
                       .contains(&String::from("mk")));
            assert!(set.filter("m").unwrap().find().unwrap()
                       .value().as_vec().unwrap()
                       .contains(&String::from("m4")));
            assert!(set.filter("w").unwrap().find().unwrap()
                       .value().as_vec().unwrap()
                       .contains(&String::from("Makefile")));
            assert!(set.filter("a").unwrap().find().unwrap()
                       .value().as_vec().unwrap()
                       .contains(&String::from("png")));
            assert!(set.filter("a").unwrap().find().unwrap()
                       .value().as_vec().unwrap()
                       .contains(&String::from("jpg")));
            assert!(set.filter("i").unwrap().find().unwrap()
                       .value().as_bool().unwrap());
            assert!(set.filter("debug").unwrap().find().unwrap()
                       .value().as_bool().unwrap());
            assert_eq!(noa[0], String::from("download/sources"));
            Ok(true)
        }

        if let Ok(mut commit) = set.add_opt("directory=pos@1") {
            let id = commit.commit().unwrap();
            parser.set_callback(id, 
                OptCallback::from_main(Box::new(SimpleMainCallback::new(
                    directory
                ))));
        }

        if let Ok(mut commit) = set.add_opt("other=main") {
            let id = commit.commit().unwrap();
            parser.set_callback(id, 
                OptCallback::from_main(Box::new(SimpleMainCallback::new(
                    |_set, noa| {
                        assert_eq!(noa[0], String::from("download/sources"));
                        assert_eq!(noa[1], String::from("picture/pngs"));
                        assert_eq!(noa[1], String::from("picture/jpgs"));
                        Ok(true)
                    }
                )))
            );
        }

        let mut ai = ArgIterator::new();

        ai.set_args(&mut [
            "-c", "c",
            "-cpp=cxx", "-cpp", "c++", "-cpp", "cpp",
            "-h", "hpp", "-h=h", "-hhxx",
            "-cfg=cpp",
            "-m=mk", "-mm4",
            "-w", "Makefile",
            "+a", "png", "+a", "jpg",
            "-i",
            "--debug",
            "download/sources",
            "picture/pngs",
            "picture/jpgs",
        ].iter().map(|&v|String::from(v)));

        parser.publish_to(Box::new(set));
        parser.parse(&mut ai).unwrap();
        parser.reset();
        assert!(parser.set().as_ref().unwrap().filter("c").unwrap().find().unwrap()
                       .value().is_null());
        assert!(parser.set().as_ref().unwrap().filter("cpp").unwrap().find().unwrap()
                    .value().is_null());
        assert!(parser.set().as_ref().unwrap().filter("h").unwrap().find().unwrap()
                       .value().is_null());
        assert!(parser.set().as_ref().unwrap().filter("cfg").unwrap().find().unwrap()
                    .value().is_null());
        assert!(parser.set().as_ref().unwrap().filter("m").unwrap().find().unwrap()
                       .value().is_null());
        assert!(parser.set().as_ref().unwrap().filter("w").unwrap().find().unwrap()
                    .value().is_null());
        assert!(parser.set().as_ref().unwrap().filter("a").unwrap().find().unwrap()
                       .value().is_null());
        assert!(! parser.set().as_ref().unwrap().filter("i").unwrap().find().unwrap()
                    .value().as_bool_or_null().unwrap_or(&false));
        assert!(! parser.set().as_ref().unwrap().filter("debug").unwrap().find().unwrap()
                       .value().as_bool_or_null().unwrap_or(&false));
    }
}
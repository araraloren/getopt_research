
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
use crate::arg::Iterator as CIterator;
use crate::arg::Argument;
use crate::error::Result;

use std::fmt::Debug;
use std::collections::HashMap;

pub trait Parser: Debug + Publisher<Box<dyn Proc>> {
    fn parse(&mut self, iter: &mut dyn CIterator) -> Result<Option<ReturnValue>>;

    fn publish_to(&mut self, set: Box<dyn Set>);

    fn set_id_generator(&mut self, id_generator: Box<dyn IdGenerator>);

    fn set_callback(&mut self, id: Identifier, callback: OptCallback);

    fn set(&self) -> &Option<Box<dyn Set>>;

    fn get_opt(&self, id: Identifier) -> Option<& dyn Opt>;

    fn get_opt_mut(&mut self, id: Identifier) -> Option<&mut dyn Opt>;

    fn noa(&self) -> &Vec<String>;

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
}

impl Parser for ForwardParser {
    fn parse(&mut self, iter: &mut dyn CIterator) -> Result<Option<ReturnValue>> {
        if self.set.is_none() {
            return Ok(None);
        }
        iter.set_prefix(self.set.as_ref().unwrap().get_all_prefixs());
        debug!("---- In Parser, start process option");
        while ! iter.reach_end() {
            let mut matched = false;

            iter.fill_current_and_next();
            self.argument_matched = false;
            debug!("**** ArgIterator [{:?}, {:?}]", iter.current(), iter.next());
            if let Ok(arg) = iter.parse() {

                debug!("parse ... {:?}", arg);

                if ! matched {
                    if let Some(ctx) = generate_argument_style(&arg, iter.next()) {
                        let mut cp = Box::new(SequenceProc::new(self.msg_id_gen.next_id()));

                        cp.append_ctx(ctx);
                        matched = self.publish(cp)?;
                    }
                }
                if ! matched {
                    let multiple_ctx = generate_multiple_style(&arg, &None);

                    if multiple_ctx.len() > 0 {
                        let mut cp = Box::new(SequenceProc::new(self.msg_id_gen.next_id()));

                        for ctx in multiple_ctx {
                            cp.append_ctx(ctx);
                        }

                        matched = self.publish(cp)?;
                    }
                }
                if ! matched {
                    if let Some(ctx) = generate_boolean_style(&arg, &None) {
                        let mut cp = Box::new(SequenceProc::new(self.msg_id_gen.next_id()));

                        cp.append_ctx(ctx);
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

        let noa_total = self.noa().len();

        // process cmd and pos
        if noa_total > 0 {
            debug!("---- In Parser, start process pos");
            for index in 1 ..= noa_total {
                if let Some(ctx) = generate_nonoption_style(&self.noa()[index - 1], noa_total as i64, index as i64) {
                    let mut cp = Box::new(SequenceProc::new(self.msg_id_gen.next_id()));

                    cp.append_ctx(ctx);
                    self.publish(cp)?;
                }
            }
        }

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

    fn reset(&mut self) {
        self.cached_infos.clear();
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

pub fn generate_argument_style(arg: &Argument, next_argument: &Option<String>) -> Option<Box<dyn Context>> {
    match arg.get_value() {
        Some(value) => {
            Some(Box::new(OptContext::new(
                arg.get_prefix().unwrap().clone(),
                arg.get_name().unwrap().clone(),
                Some(value.clone()),
                Style::Argument,
                false,
            )))
        }
        None => {
            Some(Box::new(OptContext::new(
                arg.get_prefix().unwrap().clone(),
                arg.get_name().unwrap().clone(),
                next_argument.clone(),
                Style::Argument,
                true,
            )))
        }
    }
}

pub fn generate_multiple_style(arg: &Argument, _: &Option<String>) -> Vec<Box<dyn Context>> {
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

pub fn generate_boolean_style(arg: &Argument, _:  &Option<String>) -> Option<Box<dyn Context>> {
    match arg.get_value() {
        Some(_) => None,
        None => {
            Some(Box::new(OptContext::new(
                arg.get_prefix().unwrap().clone(),
                arg.get_name().unwrap().clone(),
                None,
                Style::Boolean,
                false,
            )))
        }
    }    
}

pub fn generate_nonoption_style(noa: &String, total: i64, current: i64)-> Option<Box<dyn Context>> {
    Some(Box::new(NonOptContext::new( noa.clone(), Style::Pos, total, current )))
}

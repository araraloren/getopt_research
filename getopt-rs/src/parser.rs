
use crate::{ctx::DelayContext, proc::Proc};
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

#[maybe_async::maybe_async(?Send)]
pub trait Parser: Debug + Publisher<Box<dyn Proc>> {
    async fn parse(&mut self, iter: &mut dyn IndexIterator) -> Result<Option<bool>>;

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
pub struct ForwardParser {
    msg_id_gen: Box<dyn IdGenerator>,

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

    pub fn invoke_callback(&mut self, id: &Identifier, callback_type: CallbackType) -> Result<bool> {
        let opt = self.set.as_mut().unwrap().get_opt_mut(id.clone()).unwrap();

        if let Some(callback) = self.callbacks.get_mut(id) {
            debug!("!!!! Calling callback of {:?}", id);
            match callback_type {
                CallbackType::Value => {
                    callback.call_value(opt)?;
                }
                CallbackType::Index => {
                    let length = self.noa.len();
                    let index = opt.index().calc_index(length as i64);

                    if let Some(index) = index {
                        let ret = callback.call_index(self.set.as_ref().unwrap().as_ref(), &self.noa[index as usize - 1])?;
                        // can we fix this long call?
                        self.set.as_mut().unwrap().get_opt_mut(id.clone()).unwrap().set_value(OptValue::from_bool(ret));
                    }
                }
                CallbackType::Main => {
                    let ret = callback.call_main(self.set.as_ref().unwrap().as_ref(), &self.noa)?;
                    // can we fix this long call?
                    self.set.as_mut().unwrap().get_opt_mut(id.clone()).unwrap().set_value(OptValue::from_bool(ret));
                }
                _ => { }
            }
        }
        Ok(true)
    }
}

#[maybe_async::maybe_async(?Send)]
impl Parser for ForwardParser {
    async fn parse(&mut self, iter: &mut dyn IndexIterator) -> Result<Option<bool>> {
        if self.set.is_none() {
            return Ok(None);
        }
        let opt_order = [
            GenStyle::GS_Equal_With_Value,
            GenStyle::GS_Argument,
            GenStyle::GS_Boolean,
            GenStyle::GS_Mutliple_Option,
            GenStyle::GS_Embedded_Value,
        ];

        debug!("---- In ForwardParser, start process option");
        while ! iter.reach_end() {
            let mut matched = false;

            iter.fill_current_and_next();
            self.argument_matched = false;
            debug!("**** ArgIterator [{:?}, {:?}]", iter.current(), iter.next());

            if let Ok(arg) = iter.parse(self.get_prefix()).await {
                debug!("parse ... {:?}", arg);
                for opt_style in &opt_order {
                    if ! matched {
                        let multiple_ctx = opt_style.gen_opt(&arg, iter.next());

                        if multiple_ctx.len() > 0 {
                            let mut cp = Box::new(SequenceProc::new(self.msg_id_gen.next_id()));

                            for ctx in multiple_ctx {
                                cp.app_ctx(ctx);
                            }

                            matched = self.publish(cp).await?;
                        }
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
            debug!("---- In ForwardParser, start process {:?}", GenStyle::GS_Non_Cmd);
            let non_opt_cmd = GenStyle::GS_Non_Cmd.gen_nonopt(&self.noa()[0], noa_total as i64, 1);

            if non_opt_cmd.len() > 0 {
                let mut cp = Box::new(SequenceProc::new(self.msg_id_gen.next_id()));

                for non_opt in non_opt_cmd {
                    cp.app_ctx(non_opt);
                }
                self.publish(cp).await?;
            }

            debug!("---- In ForwardParser, start process {:?}", GenStyle::GS_Non_Pos);
            for index in 1 ..= noa_total {
                let non_opt_pos = GenStyle::GS_Non_Pos.gen_nonopt(&self.noa()[index - 1], noa_total as i64, index as i64);

                if non_opt_pos.len() > 0  {
                    let mut cp = Box::new(SequenceProc::new(self.msg_id_gen.next_id()));

                    for non_opt in non_opt_pos {
                        cp.app_ctx(non_opt);
                    }
                    self.publish(cp).await?;
                }
            }
        }

        self.check_nonopt()?;

        debug!("---- In ForwardParser, start process {:?}", GenStyle::GS_Non_Main);
        let non_opt_main = GenStyle::GS_Non_Main.gen_nonopt(&String::new(), 0, 0);

        if non_opt_main.len() > 0 {
            let mut cp = Box::new(SequenceProc::new(self.msg_id_gen.next_id()));

            for main in non_opt_main {
                cp.app_ctx(main);
            }
            self.publish(cp).await?;
        }

        self.check_other()?;

        Ok(Some(true))
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

#[maybe_async::maybe_async(?Send)]
impl Publisher<Box<dyn Proc>> for ForwardParser {
    async fn publish(&mut self, msg: Box<dyn Proc>) -> Result<bool> {
        let mut proc = msg;

        debug!("Receive msg<{:?}> => {:?}", &proc.id(), &proc);

        for index in 0 .. self.cached_infos.len() {
            let info = self.cached_infos.get_mut(index).unwrap();
            let opt = self.set.as_mut().unwrap().get_opt_mut(info.id()).unwrap(); // id always exist, so just unwrap
            let res = proc.process(opt).await?;
            let need_invoke = opt.is_need_invoke();
            let callback_type = opt.callback_type();

            if res {
                if need_invoke {
                    let id = info.id();
                    
                    opt.set_need_invoke(false);
                    self.invoke_callback(&id, callback_type)?;
                }
            }
            if proc.is_matched() {
                debug!("Proc<{:?}> Matched", proc.id());
                break;
            }
        }
        if proc.is_matched() && proc.is_need_argument() {
            self.set_argument_matched();
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

#[derive(Debug)]
pub struct DelayParser {
    msg_id_gen: Box<dyn IdGenerator>,

    cached_infos: Vec<Box<dyn Info>>,

    noa: Vec<String>,

    set: Option<Box<dyn Set>>,

    argument_matched: bool,

    callbacks: HashMap<Identifier, OptCallback>,

    value_mapper: HashMap<Identifier, OptValue>,
}

impl DelayParser {
    pub fn new(msg_id_gen: Box<dyn IdGenerator>) -> Self {
        Self {
            msg_id_gen: msg_id_gen,
            cached_infos: vec![],
            noa: vec![],
            set: None,
            argument_matched: false,
            callbacks: HashMap::new(),
            value_mapper: HashMap::new(),
        }
    }

    pub fn set_argument_matched(&mut self) {
        self.argument_matched = true;
    }

    pub fn get_prefix(&self) -> &Vec<String> {
        self.set.as_ref().unwrap().get_prefix()
    }

    pub fn add_delay_value(&mut self, id: Identifier, value: OptValue) {
        self.value_mapper.insert(id, value);
    }

    pub fn invoke_callback(&mut self, id: &Identifier, callback_type: CallbackType) -> Result<bool> {
        let opt = self.set.as_mut().unwrap().get_opt_mut(id.clone()).unwrap();

        if let Some(callback) = self.callbacks.get_mut(id) {
            debug!("!!!! Calling callback of {:?}", id);
            match callback_type {
                CallbackType::Value => {
                    callback.call_value(opt)?;
                }
                CallbackType::Index => {
                    let length = self.noa.len();
                    let index = opt.index().calc_index(length as i64);

                    if let Some(index) = index {
                        let ret = callback.call_index(self.set.as_ref().unwrap().as_ref(), &self.noa[index as usize - 1])?;
                        // can we fix this long call?
                        self.set.as_mut().unwrap().get_opt_mut(id.clone()).unwrap().set_value(OptValue::from_bool(ret));
                    }
                }
                CallbackType::Main => {
                    let ret = callback.call_main(self.set.as_ref().unwrap().as_ref(), &self.noa)?;
                    // can we fix this long call?
                    self.set.as_mut().unwrap().get_opt_mut(id.clone()).unwrap().set_value(OptValue::from_bool(ret));
                }
                _ => { }
            }
        }
        Ok(true)
    }
}

#[maybe_async::maybe_async(?Send)]
impl Parser for DelayParser {
    async fn parse(&mut self, iter: &mut dyn IndexIterator) -> Result<Option<bool>> {
        if self.set.is_none() {
            return Ok(None);
        }
        let opt_order = [
            GenStyle::GS_Delay_Equal_With_Value,
            GenStyle::GS_Delay_Argument,
            GenStyle::GS_Delay_Boolean,
            GenStyle::GS_Delay_Mutliple_Option,
            GenStyle::GS_Delay_Embedded_Value,
        ];

        debug!("---- In ForwardParser, start process option");
        while ! iter.reach_end() {
            let mut matched = false;

            iter.fill_current_and_next();
            self.argument_matched = false;
            debug!("**** ArgIterator [{:?}, {:?}]", iter.current(), iter.next());

            if let Ok(arg) = iter.parse(self.get_prefix()).await {
                debug!("parse ... {:?}", arg);
                for opt_style in &opt_order {
                    if ! matched {
                        let multiple_ctx = opt_style.gen_opt(&arg, iter.next());

                        if multiple_ctx.len() > 0 {
                            let mut cp = Box::new(SequenceProc::new(self.msg_id_gen.next_id()));

                            for ctx in multiple_ctx {
                                cp.app_ctx(ctx);
                            }

                            matched = self.publish(cp).await?;
                        }
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
            debug!("---- In ForwardParser, start process {:?}", GenStyle::GS_Non_Cmd);
            let non_opt_cmd = GenStyle::GS_Non_Cmd.gen_nonopt(&self.noa()[0], noa_total as i64, 1);

            if non_opt_cmd.len() > 0 {
                let mut cp = Box::new(SequenceProc::new(self.msg_id_gen.next_id()));

                for non_opt in non_opt_cmd {
                    cp.app_ctx(non_opt);
                }
                self.publish(cp).await?;
            }

            debug!("---- In ForwardParser, start process {:?}", GenStyle::GS_Non_Pos);
            for index in 1 ..= noa_total {
                let non_opt_pos = GenStyle::GS_Non_Pos.gen_nonopt(&self.noa()[index - 1], noa_total as i64, index as i64);

                if non_opt_pos.len() > 0  {
                    let mut cp = Box::new(SequenceProc::new(self.msg_id_gen.next_id()));

                    for non_opt in non_opt_pos {
                        cp.app_ctx(non_opt);
                    }
                    self.publish(cp).await?;
                }
            }
        }

        // set delay value 
        {
            let ids: Vec<_> = self.value_mapper.keys().map(|v| v.clone()).collect();

            for id in ids {
                if let Some(x) = self.value_mapper.remove_entry(&id) {
                    let opt = self.set.as_mut().unwrap().get_opt_mut(id.clone()).unwrap();

                    opt.set_value(x.1);
                    
                    let callback_type = opt.callback_type();

                    opt.set_need_invoke(false);
                    self.invoke_callback(&id, callback_type)?;
                }
            }
        }

        self.check_nonopt()?;

        debug!("---- In ForwardParser, start process {:?}", GenStyle::GS_Non_Main);
        let non_opt_main = GenStyle::GS_Non_Main.gen_nonopt(&String::new(), 0, 0);

        if non_opt_main.len() > 0 {
            let mut cp = Box::new(SequenceProc::new(self.msg_id_gen.next_id()));

            for main in non_opt_main {
                cp.app_ctx(main);
            }
            self.publish(cp).await?;
        }

        self.check_other()?;

        Ok(Some(true))
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
        Ok(true)    
    }

    fn check_nonopt(&self) -> Result<bool> {
        parser_default_opt_check(self.set.as_ref().unwrap().as_ref())?;
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

#[maybe_async::maybe_async(?Send)]
impl Publisher<Box<dyn Proc>> for DelayParser {
    async fn publish(&mut self, msg: Box<dyn Proc>) -> Result<bool> {
        let mut proc = msg;
        let mut value_keeper: HashMap::<Identifier, OptValue> = HashMap::new();
        let mut process_id: Vec<Identifier> = vec![];

        debug!("Receive msg<{:?}> => {:?}", &proc.id(), &proc);
        
        for index in 0 .. self.cached_infos.len() {
            let info = self.cached_infos.get_mut(index).unwrap();
            let opt = self.set.as_mut().unwrap().get_opt_mut(info.id()).unwrap(); // id always exist, so just unwrap
            let res = proc.process(opt).await?;
            let mut need_invoke = opt.is_need_invoke();
            let callback_type = opt.callback_type();
            let id = info.id();

            if res {
                for ctx in proc.get_ctx() {
                    if ctx.is_matched() && !process_id.contains(&id) {
                        let default_string = String::default();
                        let v = ctx.get_next_argument().as_ref().unwrap_or(&default_string);
                        let value = opt.parse_value(v.as_str())?;

                        match ctx.get_style()  {
                            Style::Argument | Style::Boolean | Style::Multiple => {
                                value_keeper.insert(id.clone(), value);
                                need_invoke = false;
                            }
                            _ => {
                                // Set the option value if we using a delay context
                                opt.set_value(value);
                            }
                        }
                        process_id.push(id.clone());
                        break;
                    }
                }
                if need_invoke {
                    opt.set_need_invoke(false);
                    self.invoke_callback(&id, callback_type)?;
                }
            }

            if proc.is_matched() {
                debug!("Proc<{:?}> Matched", proc.id());
                break;
            }
        }

        if proc.is_matched() && proc.is_need_argument() {
            self.set_argument_matched();
        }

        for (id, value) in value_keeper {
            self.add_delay_value(id, value);
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

#[allow(non_camel_case_types)]
#[derive(Debug)]
enum GenStyle {
    GS_Equal_With_Value,
    GS_Argument,
    GS_Embedded_Value,
    GS_Mutliple_Option,
    GS_Boolean,
    GS_Delay_Equal_With_Value,
    GS_Delay_Argument,
    GS_Delay_Embedded_Value,
    GS_Delay_Mutliple_Option,
    GS_Delay_Boolean,
    GS_Non_Main,
    GS_Non_Pos,
    GS_Non_Cmd,
}

impl GenStyle {
    pub fn gen_opt(&self, arg: &Argument, next_argument: &Option<String>) -> Vec<Box<dyn Context>> {
        let mut ret: Vec<Box<dyn Context>> = vec![];
        let default_value = String::default();

        match self {
            Self::GS_Equal_With_Value => {
                if let Some(value) = arg.get_value() {
                    ret.push(Box::new(OptContext::new(
                    arg.get_prefix().unwrap_or(&default_value).clone(),
                    arg.get_name().unwrap_or(&default_value).clone(),
                    Some(value.clone()),
                    Style::Argument,
                    false)));
                }
            }
            Self::GS_Argument => {
                if arg.get_value().is_none() {
                    ret.push(Box::new(OptContext::new(
                        arg.get_prefix().unwrap_or(&default_value).clone(),
                        arg.get_name().unwrap_or(&default_value).clone(),
                        next_argument.clone(),
                        Style::Argument,
                        true)));
                }
            }
            Self::GS_Embedded_Value => {
                if arg.get_value().is_none() {
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
            Self::GS_Mutliple_Option => {
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
            }
            Self::GS_Boolean => {
                if arg.get_value().is_none() {
                    ret.push(Box::new(OptContext::new(
                        arg.get_prefix().unwrap().clone(),
                        arg.get_name().unwrap().clone(),
                        None,
                        Style::Boolean,
                        false,
                    )));
                }
            }
            Self::GS_Delay_Equal_With_Value => {
                if let Some(value) = arg.get_value() {
                    ret.push(Box::new(DelayContext::new(
                    arg.get_prefix().unwrap_or(&default_value).clone(),
                    arg.get_name().unwrap_or(&default_value).clone(),
                    Some(value.clone()),
                    Style::Argument,
                    false)));
                }
            }
            Self::GS_Delay_Argument => {
                if arg.get_value().is_none() {
                    ret.push(Box::new(DelayContext::new(
                        arg.get_prefix().unwrap_or(&default_value).clone(),
                        arg.get_name().unwrap_or(&default_value).clone(),
                        next_argument.clone(),
                        Style::Argument,
                        true)));
                }
            }
            Self::GS_Delay_Embedded_Value => {
                if arg.get_value().is_none() {
                    if let Some(name) = arg.get_name() {
                        if name.len() >= 2 {
                            let name_and_value = name.split_at(1);

                            ret.push(Box::new(DelayContext::new(
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
            Self::GS_Delay_Mutliple_Option => {
                if arg.get_value().is_none() {
                    if arg.get_name().unwrap().len() > 1 {
                        for char in arg.get_name().unwrap().chars() {
                            ret.push(Box::new(DelayContext::new(
                                arg.get_prefix().unwrap().clone(),
                                String::from(char),
                                None,
                                Style::Multiple,
                                false,
                            )))
                        }
                    }
                }
            }
            Self::GS_Delay_Boolean => {
                if arg.get_value().is_none() {
                    ret.push(Box::new(DelayContext::new(
                        arg.get_prefix().unwrap().clone(),
                        arg.get_name().unwrap().clone(),
                        None,
                        Style::Boolean,
                        false,
                    )));
                }
            }
            _ => { }
        }
        ret
    }

    pub fn gen_nonopt(&self, noa: &String, total: i64, current: i64)-> Vec<Box<dyn Context>> {
        let mut ret: Vec<Box<dyn Context>> = vec![];

        match self {
            Self::GS_Non_Pos => {
                ret.push(Box::new(NonOptContext::new( noa.clone(), Style::Pos, total, current )));
            }
            Self::GS_Non_Cmd => {
                ret.push(Box::new(NonOptContext::new( noa.clone(), Style::Cmd, total, current )));
            }
            Self::GS_Non_Main => {
                ret.push(Box::new(NonOptContext::new( noa.clone(), Style::Main, total, current )));
            }
            _ => { }
        }
        ret
    }
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

        set.initialize_prefixs();
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

    #[test]
    fn make_sure_delayparser_work() {
        use std::sync::Arc;
        use std::sync::Mutex;

        let directorys = Arc::new(Mutex::new(vec![]));
        let main_ref = directorys.clone();

        let id = DefaultIdGen::default();
        let mut set = DefaultSet::new();
        let mut parser = DelayParser::new(Box::new(id));

        assert!(set.add_utils(Box::new(bool::BoolUtils::new())).unwrap_or(false));
        assert!(set.add_utils(Box::new(str::StrUtils::new())).unwrap_or(false));
        assert!(set.add_utils(Box::new(array::ArrayUtils::new())).unwrap_or(false));
        assert!(set.add_utils(Box::new(pos::PosUtils::new())).unwrap_or(false));
        assert!(set.add_utils(Box::new(cmd::CmdUtils::new())).unwrap_or(false));
        assert!(set.add_utils(Box::new(main::MainUtils::new())).unwrap_or(false));

        set.initialize_prefixs();
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
            let _id = commit.commit().unwrap();
        }

        if let Ok(mut commit) = set.add_opt("other=main") {
            let id = commit.commit().unwrap();
            parser.set_callback(id, 
                OptCallback::from_main(Box::new(SimpleMainCallback::new(
                    move |set, noa| {
                        assert_eq!(noa[0], String::from("download/sources"));
                        assert_eq!(noa[1], String::from("picture/pngs"));
                        assert_eq!(noa[1], String::from("picture/jpgs"));
                        let mut writer = main_ref.lock().unwrap();

                        writer.push(noa[0].clone());
                        writer.push(noa[1].clone());
                        writer.push(noa[2].clone());

                        directory(set, noa).unwrap();
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
use crate::ctx::Context;
use crate::ctx::OptContext;
use crate::opt::Style;
use crate::proc::Info;
use crate::proc::Proc;
use crate::proc::Publisher;
use crate::set::Set;
use crate::utils::CommandInfo;

use crate::id::DefaultIdGen;
use crate::id::IdGenerator;

#[derive(Debug)]
pub struct Parser {
    msg_id_gen: Box<dyn IdGenerator>,

    set: Option<Set>,

    info: Vec<Box<dyn Info>>,

    arg: Option<String>,

    next_arg: Option<String>,

    index: usize,

    total: usize,

    args: Vec<String>,

    noa: Vec<String>,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            msg_id_gen: Box::new(DefaultIdGen::new()),
            set: None,
            info: vec![],
            arg: None,
            next_arg: None,
            index: 0,
            total: 0,
            args: vec![],
            noa: vec![],
        }
    }

    pub fn set(&self) -> Option<&Set> {
        if self.set.is_none() {
            panic!("In Parser, can not find Set !!!");
        }
        self.set.as_ref()
    }

    pub fn set_mut(&mut self) -> Option<&mut Set> {
        if self.set.is_none() {
            panic!("In Parser, can not find Set !!!");
        }
        self.set.as_mut()
    }

    pub fn publish_to(&mut self, set: Set) -> &mut Self {
        self.set = Some(set);
        self
    }

    pub fn get_prefixs(&self) -> Vec<String> {
        self.set().unwrap().collect_prefix()
    }

    pub fn init<T: Iterator<Item = String>>(&mut self, args: T) {
        for arg in args {
            self.args.push(arg);
        }
        self.reset();
    }

    pub fn init_default(&mut self) {
        self.init(std::env::args());
    }

    pub fn parse(&mut self) {
        let mut matched = false;
        let mut ci = CommandInfo::new(self.get_prefixs());

        while !self.iterator_reach_end() {
            self.fill_current_and_next_arg();

            debug!(
                "------ current arg = `{:?}`, next args = `{:?}`",
                &self.arg, &self.next_arg
            );

            if ci.parse(self.arg.as_ref().unwrap().as_str()) {
                if let Some(ctx) = self.gen_argument_style(&ci) {
                    let mut cp = Proc::new(self.msg_id_gen.next_id());

                    cp.append_ctx(ctx);

                    debug!("parser.1 broadcast option style has argument ...");
                    matched = self.publish(cp);
                }

                if ! matched {
                    let multiple_ctx = self.gen_multiple_style(&ci);

                    if multiple_ctx.len() > 0 {
                        let mut cp = Proc::new(self.msg_id_gen.next_id());

                        for ctx in multiple_ctx {
                            cp.append_ctx(ctx);
                        }

                        debug!("parser.2 broadcast option combined style ...");
                        matched = self.publish(cp);
                    }
                }

                if ! matched {
                    if let Some(ctx) = self.gen_boolean_style(&ci) {
                        let mut cp = Proc::new(self.msg_id_gen.next_id());

                        cp.append_ctx(ctx);

                        debug!("parser.3 broadcast boolean option style ...");
                        matched = self.publish(cp);
                    }
                }

                if ! matched {
                    self.ignore_current();
                }
            }

            ci.reset();
            self.skip_next_arg();
        }
    }

    fn iterator_reach_end(&self) -> bool {
        self.current_index() >= self.total_index()
    }

    fn fill_current_and_next_arg(&mut self) {
        self.arg = Some(self.args[self.current_index()].clone());
        self.next_arg = if self.current_index() + 1 < self.total_index() {
            Some(self.args[self.current_index() + 1].clone())
        } else {
            None
        };
    }

    fn current_index(&self) -> usize {
        self.index
    }

    fn total_index(&self) -> usize {
        self.total
    }

    fn reset(&mut self) {
        self.index = 0;
        self.total = self.args.len();
    }

    fn skip_next_arg(&mut self) {
        self.index += 1;
    }

    fn ignore_current(&mut self) {
        self.noa.push(self.arg.as_ref().unwrap().clone());
    }

    fn gen_argument_style(&self, ci: &CommandInfo) -> Option<Box<dyn Context>> {
        match ci.get_value() {
            Some(value) => Some(Box::new(OptContext::new(
                ci.get_prefix().unwrap().clone(),
                ci.get_name().unwrap().clone(),
                Some(value.clone()),
                Style::Argument,
                false,
            ))),
            None => Some(Box::new(OptContext::new(
                ci.get_prefix().unwrap().clone(),
                ci.get_name().unwrap().clone(),
                self.next_arg.clone(),
                Style::Argument,
                true,
            ))),
        }
    }

    fn gen_multiple_style(&self, ci: &CommandInfo) -> Vec<Box<dyn Context>> {
        let mut ret: Vec<Box<dyn Context>> = vec![];

        if ci.get_value().is_none() {
            if ci.get_name().unwrap().len() > 1 {
                for char in ci.get_name().unwrap().chars() {
                    ret.push(Box::new(OptContext::new(
                        ci.get_prefix().unwrap().clone(),
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

    fn gen_boolean_style(&self, ci: &CommandInfo) -> Option<Box<dyn Context>> {
        match ci.get_value() {
            Some(_) => None,
            None => Some(Box::new(OptContext::new(
                ci.get_prefix().unwrap().clone(),
                ci.get_name().unwrap().clone(),
                None,
                Style::Boolean,
                false,
            ))),
        }
    }
}

impl Publisher<Proc> for Parser {
    fn publish(&mut self, msg: Proc) -> bool {
        let mut proc = msg;

        debug!("publish msg: {:?}", proc);
        for index in 0..self.info.len() {
            let info = self.info.get_mut(index).unwrap();

            proc.run(self.set.as_mut().unwrap().get_opt_mut(info.id()).unwrap());
        }
        
        debug!("running result {{ matched: {}, skip_next: {} }} ", proc.is_matched(), proc.is_skip_next_arg());
        if proc.is_matched() {
            if proc.is_skip_next_arg() {
                self.skip_next_arg();
            }
        }

        proc.is_matched()
    }

    fn subscribe(&mut self, info: Box<dyn Info>) {
        self.info.push(info);
    }

    fn clean(&mut self) {
        self.info.clear();
    }
}

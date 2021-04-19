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
}

impl Parser {
    pub fn new() -> Self {
        Self {
            msg_id_gen: Box::new(DefaultIdGen::new()),
            set: None,
            info: vec![],
            arg: None,
            next_arg: None,
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

    pub fn parse(&mut self, args: &'static [&str]) {
        let mut index = 0usize;
        let mut count = args.len();
        let mut ci = CommandInfo::new(self.get_prefixs());

        while index < count {
            self.arg = Some(String::from(args[index]));
            self.next_arg = if index + 1 >= count {
                None
            } else {
                Some(String::from(args[index + 1]))
            };

            if ci.parse(self.arg.as_ref().unwrap().as_str()) {
                if let Some(ctx) = self.gen_argument_style(&ci) {
                    let mut cp = Proc::new(self.msg_id_gen.next_id());

                    cp.append_ctx(ctx);
                    self.publish(cp);
                }

                let multiple_ctx = self.gen_multiple_style(&ci);

                if multiple_ctx.len() > 0 {
                    let mut cp = Proc::new(self.msg_id_gen.next_id());

                    for ctx in multiple_ctx {
                        cp.append_ctx(ctx);
                    }
                    self.publish(cp);
                }

                if let Some(ctx) = self.gen_boolean_style(&ci) {
                    let mut cp = Proc::new(self.msg_id_gen.next_id());

                    cp.append_ctx(ctx);
                    self.publish(cp);
                }
            }

            ci.reset();
            index += 1;
        }
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
                false,
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
    fn publish(&mut self, msg: Proc) {
        let mut proc = msg;

        debug!("get msg: {:?}", proc);
        for index in 0..self.info.len() {
            let info = self.info.get_mut(index).unwrap();

            proc.run(self.set.as_mut().unwrap().get_opt_mut(info.id()).unwrap());
        }
    }

    fn subscribe(&mut self, info: Box<dyn Info>) {
        self.info.push(info);
    }

    fn clean(&mut self) {
        self.info.clear();
    }
}

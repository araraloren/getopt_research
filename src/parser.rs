use crate::set::Set;
use crate::opt::Style;
use crate::utils::CommandInfo;
use crate::ctx::OptContext;
use crate::proc::Info;
use crate::proc::Proc;
use crate::proc::Publisher;

use crate::id::IdGenerator;
use crate::id::DefaultIdGen;

#[derive(Debug)]
pub struct Parser {
    msg_id_gen: Box<dyn IdGenerator>,

    set: Option<Set>,

    info: Vec<Box<dyn Info>>,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            msg_id_gen: Box::new(DefaultIdGen::new()),
            set: None,
            info: vec![]
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
        let mut arg_index = 0usize;
        let mut ci = CommandInfo::new(self.get_prefixs());

        while arg_index < args.len() {
            let arg = args[arg_index];
            let mut cp = Proc::new(self.msg_id_gen.next_id());
            
            if ci.parse(arg) {
                dbg!(&ci);
                cp.append_ctx(Box::new(OptContext::new(
                    ci.get_prefix().unwrap().clone(), 
                    ci.get_name().unwrap().clone(),
                    if arg_index >= args.len() - 1 { None } else { Some(String::from(args[arg_index + 1])) },
                    Style::Argument,
                    false
                )));
                self.publish(cp);
            }

            ci.reset();
            arg_index += 1;
        }
    }
}

impl Publisher<Proc> for Parser {
    fn publish(&mut self, msg: Proc) {
        let mut proc = msg;

        debug!("get msg: {:?}", proc);
        for index in 0 .. self.info.len() {
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

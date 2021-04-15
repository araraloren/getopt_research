use crate::set::Set;
use crate::opt::Style;
use crate::ctx::ArgGetter;
use crate::ctx::OptContext;
use crate::proc::Info;
use crate::proc::Proc;
use crate::proc::Publisher;

#[derive(Debug)]
pub struct Parser {
    msg_id_counter: u64,

    set: Option<Set>,

    info: Vec<Box<dyn Info<Proc>>>,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            msg_id_counter: 0,
            set: None,
            info: vec![]
        }
    }

    pub fn set(&mut self, set: Set) -> &mut Self {
        self.set = Some(set);
        self
    }

    pub fn parse(&mut self, args: &[&str]) {
        for arg in args {
            let mut cp = Proc::new(self.msg_id_counter, Style::Argument);
            
            cp.append_ctx(Box::new(OptContext::new("", arg, ArgGetter(None), true, false)));
            self.msg_id_counter += 1;
            self.publish(cp);
        }
    }
}

impl Publisher<Proc> for Parser {
    fn publish(&mut self, msg: Proc) {
        let mut msg = msg;

        for index in 0 .. self.info.len() {
            let info = self.info.get_mut(index).unwrap();
            let opt  = self.set.as_mut().unwrap().get_opt_mut(info.info_id());

            if info.check(&msg) {
                info.process(&mut msg, opt.unwrap());
            }
        }
    }

    fn subscribe(&mut self, info: Box<dyn Info<Proc>>) {
        self.info.push(info);
    }

    fn clean(&mut self) {
        self.info.clear();
    }
}

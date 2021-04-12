use crate::opt::Opt;
use crate::proc::Proc;
use crate::proc::Message;

use std::fmt::Debug;

pub trait Context<M: Message>: Debug {
    fn match_msg(&self, msg: &M, opt: &dyn Opt) -> bool;

    fn process(&self, msg: &M, opt: &mut dyn Opt);

    fn set_match(&mut self);

    fn matched(&self) -> bool;
}

pub struct ArgGetter(pub Option<Box<dyn Fn() -> String>>);

impl Debug for ArgGetter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ArgGetter").field("Fn", &"()").finish()
    }
}

#[derive(Debug)]
pub struct OptContext {
    // prefix of option
    prefix: String,

    // name of option
    name: String,

    // a function that can get argument of option
    args: ArgGetter,

    // do we need an argument
    argument: bool,

    // can we skip next argument when matched
    can_skip: bool,

    // are we matched
    matched: bool,
}

impl<'a, 'b> OptContext {
    pub fn new(
        prefix: &'a str,
        name: &'b str,
        args: ArgGetter,
        argument: bool,
        can_skip: bool,
    ) -> Self {
        Self {
            prefix: String::from(prefix),
            name: String::from(name),
            argument,
            args,
            can_skip,
            matched: false,
        }
    }
}

impl Context<Proc> for OptContext {
    fn match_msg(&self, msg: &Proc, opt: &dyn Opt) -> bool {
        true
    }

    fn process(&self, msg: &Proc, opt: &mut dyn Opt) {

    }

    fn set_match(&mut self) {
        self.matched = true;
    }

    fn matched(&self) -> bool {
        self.matched
    }
}

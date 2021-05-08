
use std::fmt::Debug;

use crate::error::Result;
use crate::opt::Opt;
use crate::ctx::Context;
use crate::id::Identifier;

pub trait Message: Debug {
    fn id(&self) -> Identifier;
}

pub trait Info: Debug {
    fn id(&self) -> Identifier;
}

pub trait Publisher<M: Message> {
    fn publish(&mut self, msg: M) -> Result<bool>;

    fn reg_subscriber(&mut self, info: Box<dyn Info>);

    fn clean(&mut self);
}

pub trait Subscriber {
    fn subscribe_from(&self, publisher: &mut dyn Publisher<Box<dyn Proc>>);
}

pub trait Proc: Debug {
    fn id(&self) -> Identifier;

    fn append_ctx(&mut self, ctx: Box<dyn Context>);

    fn process(&mut self, opt: &mut dyn Opt) -> Result<bool>;

    fn is_need_argument(&self) -> bool;

    fn is_matched(&self) -> bool;
}

impl Message for Box<dyn Proc> {
    fn id(&self) -> Identifier {
        Proc::id(self.as_ref())
    }
}

/// Default `Proc`, it will match every `Context` with given `Opt`.
/// It will call `Contex::process` on the `Opt` if matched.
#[derive(Debug)]
pub struct SequenceProc {
    id: Identifier,

    contexts: Vec<Box<dyn Context>>,

    need_argument: bool,

    matched: bool,
}

impl SequenceProc {
    pub fn new(id: Identifier) -> Self {
        Self {
            id,
            contexts: vec![],
            need_argument: false,
            matched: false,
        }
    }
}

impl Proc for SequenceProc {
    fn id(&self) -> Identifier {
        self.id
    }

    fn append_ctx(&mut self, ctx: Box<dyn Context>) {
        self.contexts.push(ctx);
    }

    fn process(&mut self, opt: &mut dyn Opt) -> Result<bool> {
        if self.is_matched() {
            return Ok(true);
        }

        self.matched = true;
        self.need_argument = false;

        for ctx in self.contexts.iter_mut() {
            if ! ctx.is_matched() {
                if ctx.match_opt(opt) {
                    ctx.process(opt)?;
                    self.need_argument = self.need_argument || ctx.is_need_argument();
                }
                else {
                    self.matched = false;
                }
            }
        }
        Ok(self.matched)
    }

    fn is_matched(&self) -> bool {
        self.matched
    }

    fn is_need_argument(&self) -> bool {
        self.need_argument
    }
}

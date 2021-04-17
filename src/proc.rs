use crate::ctx::Context;
use crate::opt::Opt;
use crate::opt::Style;

use std::fmt::Debug;

pub trait Message: Debug {
    type Data;

    fn id(&self) -> u64;

    fn data(&mut self) -> &mut Self::Data;
}

pub trait Info: Debug {
    fn id(&self) -> u64;
}

pub trait Publisher<M: Message> {
    fn publish(&mut self, msg: M);

    fn subscribe(&mut self, info: Box<dyn Info>);

    fn clean(&mut self);
}

#[derive(Debug)]
pub struct Proc {
    // id of current Proc
    proc_id: u64,

    // context need process
    ctxs: Vec<Box<dyn Context<Proc>>>,
}

impl Proc {
    pub fn new(id: u64) -> Self {
        Self {
            proc_id: id,
            ctxs: vec![],
        }
    }

    pub fn append_ctx(&mut self, ctx: Box<dyn Context<Proc>>) {
        self.ctxs.push(ctx);
    }

    pub fn check(&self, opt: &dyn Opt) -> bool {
        for ctx in &self.ctxs {
            if ctx.match_msg(self, opt) {

            }
            else {
                return false;
            }
        }
        true
    }

    pub fn run(&mut self, opt: &mut dyn Opt) {
        debug!("");
    }
}

impl Message for Proc {
    type Data = Proc;

    fn id(&self) -> u64 {
        self.proc_id
    }

    fn data(&mut self) -> &mut Self::Data {
        self
    }
}

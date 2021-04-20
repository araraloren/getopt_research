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
    fn publish(&mut self, msg: M) -> bool;

    fn subscribe(&mut self, info: Box<dyn Info>);

    fn clean(&mut self);
}

#[derive(Debug)]
pub struct Proc {
    // id of current Proc
    proc_id: u64,

    // context need process
    ctxs: Vec<Box<dyn Context>>,

    // can we skip next argument
    skip_next_arg: bool,

    // can we matched success
    matched: bool,
}

impl Proc {
    pub fn new(id: u64) -> Self {
        Self {
            proc_id: id,
            ctxs: vec![],
            skip_next_arg: false,
            matched: false,
        }
    }

    pub fn append_ctx(&mut self, ctx: Box<dyn Context>) {
        self.ctxs.push(ctx);
    }

    pub fn run(&mut self, opt: &mut dyn Opt) {
        if self.is_matched() {
            debug!("skip running -- already matched ...");
            return;
        }
        self.matched = true;
        self.skip_next_arg = false;

        for ctx in &mut self.ctxs {
            if !ctx.is_matched() {
                if ctx.match_opt(opt) {
                    ctx.set_matched();
                    ctx.process(opt);
                    self.skip_next_arg = self.skip_next_arg || ctx.is_skip_next_arg();
                }
                else {
                    self.matched = false;
                }
            }
        }
    }

    pub fn is_skip_next_arg(&self) -> bool {
        self.skip_next_arg
    }

    pub fn is_matched(&self) -> bool {
        self.matched
    }

    pub fn len(&self) -> usize {
        self.ctxs.len()
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

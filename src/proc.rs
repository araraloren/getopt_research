use crate::ctx::Context;
use crate::opt::Opt;
use crate::opt::Style;

use std::fmt::Debug;

pub trait Message: Debug {
    type Data;

    fn msg_id(&self) -> u64;

    fn data(&mut self) -> &mut Self::Data;
}

pub trait Info<M: Message>: Debug {
    fn info_id(&self) -> u64;

    fn check(&self, msg: &M) -> bool;

    fn process(&mut self, data: &mut M::Data, opt: &mut dyn Opt);
}

pub trait Publisher<M: Message> {
    fn publish(&mut self, msg: M);

    fn subscribe(&mut self, info: Box<dyn Info<M>>);

    fn clean(&mut self);
}

#[derive(Debug)]
pub struct Proc {
    // id of current Proc
    proc_id: u64,

    // current option style 
    style: Style,

    // context need process
    ctxs: Vec<Box<dyn Context<Proc>>>,
}

impl Proc {
    pub fn new(id: u64, style: Style) -> Self {
        Self {
            proc_id: id,
            style,
            ctxs: vec![],
        }
    }

    pub fn append_ctx(&mut self, ctx: Box<dyn Context<Proc>>) {
        self.ctxs.push(ctx);
    }

    pub fn run(&mut self, opt: &mut dyn Opt) {

    }
}

impl Message for Proc {
    type Data = Proc;

    fn msg_id(&self) -> u64 {
        self.proc_id
    }

    fn data(&mut self) -> &mut Self::Data {
        self
    }
}

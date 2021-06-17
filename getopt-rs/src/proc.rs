
use std::fmt::Debug;
use async_trait::async_trait;

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

/// Publisher is the collection of [`Subscriber`].
#[async_trait(?Send)]
pub trait Publisher<M: Message> {
    #[cfg(feature="async")]
    async fn publish(&mut self, msg: M) -> Result<bool>;

    #[cfg(not(feature="async"))]
    fn publish(&mut self, msg: M) -> Result<bool>;

    fn reg_subscriber(&mut self, info: Box<dyn Info>);

    fn clean(&mut self);
}

pub trait Subscriber {
    fn subscribe_from(&self, publisher: &mut dyn Publisher<Box<dyn Proc>>);
}

/// Proc hold and process the [`Context`] created by [`Parser`](crate::parser::Parser).
#[async_trait(?Send)]
pub trait Proc: Debug {
    fn id(&self) -> Identifier;

    /// Append the context to current Proc
    fn app_ctx(&mut self, ctx: Box<dyn Context>);

    /// Get the context in the Proc
    fn get_ctx(&self, index: usize) -> Option<&Box<dyn Context>>;

    /// Process the option
    #[cfg(not(feature="async"))]
    fn process(&mut self, opt: &mut dyn Opt) -> Result<Option<u64>>;

    /// Process the option
    #[cfg(feature="async")]
    async fn process(&mut self, opt: &mut dyn Opt) -> Result<Option<u64>>;

    /// If the matched option need argument
    fn is_need_argument(&self) -> bool;

    /// If all the context matched
    fn is_matched(&self) -> bool;

    fn len(&self) -> usize;
}

impl Message for Box<dyn Proc> {
    fn id(&self) -> Identifier {
        Proc::id(self.as_ref())
    }
}

/// Default [`Proc`], it will match every [`Context`] with given [`Opt`].
/// It will call [`Context::process`] on the [`Opt`] if matched.
#[derive(Debug)]
pub struct SequenceProc {
    id: Identifier,

    contexts: Vec<Box<dyn Context>>,

    need_argument: bool,

    matched_index: Option<u64>,
}

impl SequenceProc {
    pub fn new(id: Identifier) -> Self {
        Self {
            id,
            contexts: vec![],
            need_argument: false,
            matched_index: None,
        }
    }
}

impl From<Identifier> for SequenceProc {
    fn from(id: Identifier) -> Self {
        SequenceProc::new(id)
    }
}

#[async_trait(?Send)]
impl Proc for SequenceProc {
    fn id(&self) -> Identifier {
        self.id
    }

    fn app_ctx(&mut self, ctx: Box<dyn Context>) {
        self.contexts.push(ctx);
    }

    fn get_ctx(&self, index: usize) -> Option<&Box<dyn Context>> {
        self.contexts.get(index)
    }

    #[cfg(not(feature="async"))]
    fn process(&mut self, opt: &mut dyn Opt) -> Result<Option<u64>> {
        if self.is_matched() {
            debug!("Skip process {:?}, it matched", Proc::id(self));
            return Ok(self.matched_index);
        }
        let mut matched = false;

        self.matched_index = None;
        self.need_argument = false;
        for ctx in self.contexts.iter_mut() {
            if ! ctx.is_matched() {
                if ctx.match_opt(opt) {
                    ctx.process(opt)?;
                    self.need_argument = self.need_argument || ctx.is_need_argument();
                    matched = true;
                }
            }
        }
        if matched {
            // currently, SequenceProc not prcess non-option index problem
            self.matched_index = Some(0);
        }
        Ok(self.matched_index)
    }

    #[cfg(feature="async")]
    async fn process(&mut self, opt: &mut dyn Opt) -> Result<u64> {
        if self.is_matched() {
            debug!("Skip process {:?}, it matched", Proc::id(self));
            return Ok(self.matched_index.as_ref());
        }
        let mut matched = false;

        self.matched_index = None;
        self.need_argument = false;
        for ctx in self.contexts.iter_mut() {
            if ! ctx.is_matched() {
                if ctx.match_opt(opt) {
                    ctx.process(opt)?;
                    self.need_argument = self.need_argument || ctx.is_need_argument();
                    matched = true;
                }
            }
        }
        if matched {
            // currently, SequenceProc not prcess non-option index problem
            self.matched_index = Some(0);
        }
        Ok(self.matched_index.as_ref())
    }

    fn is_matched(&self) -> bool {
        let mut ret = true;
        self.contexts.iter().for_each(|v| ret = ret && v.is_matched());
        ret
    }

    fn is_need_argument(&self) -> bool {
        self.need_argument
    }

    fn len(&self) -> usize {
        self.contexts.len()
    }
}


/// Default [`Proc`], it will match the [`Context`] with given [`Opt`].
/// It will call [`Context::process`] on the [`Opt`] if matched.
#[derive(Debug)]
pub struct SingleCtxProc {
    id: Identifier,

    context: Option<Box<dyn Context>>,

    need_argument: bool,

    matched_index: Option<u64>,
}

impl SingleCtxProc {
    pub fn new(id: Identifier) -> Self {
        Self {
            id,
            context: None,
            need_argument: false,
            matched_index: None,
        }
    }
}

impl From<Identifier> for SingleCtxProc {
    fn from(id: Identifier) -> Self {
        Self::new(id)
    }
}

#[async_trait(?Send)]
impl Proc for SingleCtxProc {
    fn id(&self) -> Identifier {
        self.id
    }

    fn app_ctx(&mut self, ctx: Box<dyn Context>) {
        self.context = Some(ctx);
    }

    fn get_ctx(&self, index: usize) -> Option<&Box<dyn Context>> {
        if index == 0 { self.context.as_ref() } else { None }
    }

    #[cfg(not(feature="async"))]
    fn process(&mut self, opt: &mut dyn Opt) -> Result<Option<u64>> {
        if self.is_matched() {
            debug!("Skip process {:?}, it matched", Proc::id(self));
            return Ok(self.matched_index);
        }
        self.need_argument = false;
        self.matched_index = None;
        if let Some(ctx) = &mut self.context {
            if ! ctx.is_matched() {
                if ctx.match_opt(opt) {
                    ctx.process(opt)?;
                    self.need_argument = self.need_argument || ctx.is_need_argument();
                    self.matched_index = Some(ctx.get_matched_index().unwrap().clone());
                }
            }
        }
        Ok(self.matched_index)
    }

    #[cfg(feature="async")]
    async fn process(&mut self, opt: &mut dyn Opt) -> Result<Option<u64>> {
        if self.is_matched() {
            debug!("Skip process {:?}, it matched", Proc::id(self));
            return Ok(self.matched_index);
        }
        self.need_argument = false;
        self.matched_index = None;
        if let Some(ctx) = &self.context {
            if ! ctx.is_matched() {
                if ctx.match_opt(opt) {
                    ctx.process(opt)?;
                    self.need_argument = self.need_argument || ctx.is_need_argument();
                    self.matched_index = Some(ctx.get_matched_index().unwrap());
                }
            }
        }
        Ok(self.matched_index)
    }

    fn is_matched(&self) -> bool {
        self.context.as_ref().unwrap().is_matched()
    }

    fn is_need_argument(&self) -> bool {
        self.need_argument
    }

    fn len(&self) -> usize {
        if self.context.is_some() { 1 } else { 0 }
    }
}
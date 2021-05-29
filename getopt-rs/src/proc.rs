
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

pub trait Subscriber<T: Proc> {
    fn subscribe_from(&self, publisher: &mut dyn Publisher<T>);
}

/// Proc hold and process the [`Context`] created by [`Parser`](crate::parser::Parser).
#[async_trait(?Send)]
pub trait Proc: Debug + From<Identifier> {
    fn id(&self) -> Identifier;

    /// Append the context to current Proc
    fn app_ctx(&mut self, ctx: Box<dyn Context>);

    /// Get all the context in the Proc
    fn get_ctx(&self) -> &Vec<Box<dyn Context>>;

    /// Process the option
    #[cfg(not(feature="async"))]
    fn process(&mut self, opt: &mut dyn Opt) -> Result<bool>;

    /// Process the option
    #[cfg(feature="async")]
    async fn process(&mut self, opt: &mut dyn Opt) -> Result<bool>;

    /// If the matched option need argument
    fn is_need_argument(&self) -> bool;

    /// If all the context matched
    fn is_matched(&self) -> bool;
}

impl<T: Proc> Message for T {
    fn id(&self) -> Identifier {
        Proc::id(self)
    }
}

/// Default [`Proc`], it will match every [`Context`] with given [`Opt`].
/// It will call [`Context::process`] on the [`Opt`] if matched.
#[derive(Debug)]
pub struct SequenceProc {
    id: Identifier,

    contexts: Vec<Box<dyn Context>>,

    need_argument: bool,
}

impl SequenceProc {
    pub fn new(id: Identifier) -> Self {
        Self {
            id,
            contexts: vec![],
            need_argument: false,
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

    fn get_ctx(&self) -> &Vec<Box<dyn Context>> {
        &self.contexts
    }

    #[cfg(not(feature="async"))]
    fn process(&mut self, opt: &mut dyn Opt) -> Result<bool> {
        if self.is_matched() {
            debug!("Skip process {:?}, it matched", Proc::id(self));
            return Ok(true);
        }
        let mut matched = false;

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
        Ok(matched)
    }

    #[cfg(feature="async")]
    async fn process(&mut self, opt: &mut dyn Opt) -> Result<bool> {
        if self.is_matched() {
            debug!("Skip process {:?}, it matched", Proc::id(self));
            return Ok(true);
        }
        let mut matched = false;

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
        Ok(matched)
    }

    fn is_matched(&self) -> bool {
        let mut ret = true;
        self.get_ctx().iter().for_each(|v| ret = ret && v.is_matched());
        ret
    }

    fn is_need_argument(&self) -> bool {
        self.need_argument
    }
}
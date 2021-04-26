use getopt_rs::ctx::Context;
use getopt_rs::id::Identifier;
use getopt_rs::opt::Opt;
use getopt_rs::proc::Message;
use getopt_rs::proc::Proc;

#[derive(Debug)]
struct DefaultProc(Identifier);

impl Proc for DefaultProc {
    fn id(&self) -> Identifier {
        self.0
    }

    fn append_ctx(&mut self, ctx: Box<dyn Context>) {}

    fn process(&mut self, opt: &mut dyn Opt) {}

    fn is_need_argument(&self) -> bool {
        true
    }

    fn is_matched(&self) -> bool {
        true
    }
}

fn accept_message<M: Message>(m: M) {
    dbg!(m.id());
}

fn main() {
    accept_message(Box::new(DefaultProc(Identifier::new(0))) as Box<dyn Proc>);

    dbg!(getopt_rs::utils::CreateInfo::parse("o=a", "").unwrap());
    dbg!(getopt_rs::utils::CreateInfo::parse("o=a!", "").unwrap());

    let mut w = C(vec![]);

    let mut b = w.a();

    b.t(1);
    b.t(2);
    b.c();
    
    dbg!(w);
}

#[derive(Debug)]
pub struct B<'a> {
    v: Vec<i32>,
    a: &'a mut A,
}

pub trait A: std::fmt::Debug {
    fn a(&mut self) -> B;

    fn p(&mut self, i: i32);
}

#[derive(Debug)]
pub struct C(Vec<i32>);

impl A for C {
    fn a(&mut self) -> B {
        B {
            v: vec![],
            a: self,
        }
    }

    fn p(&mut self, i: i32) {
        self.0.push(i);
    }
}

impl<'a> B<'a> {
    fn t(&mut self, i: i32) {
        self.v.push(i);
    }

    fn c(&mut self) {
        for v in &self.v {
            self.a.p(*v)
        }
    }
}
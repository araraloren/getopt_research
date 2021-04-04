#[derive(Debug)]
pub enum Style { X, L, S, Z, C, B, }

pub trait Name {
    fn name(&self) -> &str;

    fn match_name(&self, s: &str) -> bool;
}

pub trait Prefix {
    fn prefix(&self) -> &str;

    fn match_prefix(&self, s: &str) -> bool;
}

pub trait Optional {
    fn optional(&self) -> bool;

    fn match_optional(&self, b: bool) -> bool;
}

pub trait Opt: Name + Prefix + Optional {}

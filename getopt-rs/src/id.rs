
use std::fmt::Debug;
use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub struct Identifier(u64);

impl Identifier {
    pub fn new(id: u64) -> Self {
        Self(id)
    }

    pub fn inc(&mut self) -> &mut Self {
        self.0 += 1;
        self
    }

    pub fn set(&mut self, id: u64) -> &mut Self {
        self.0 = id;
        self
    }

    pub fn get(&self) -> u64 {
        self.0
    }
}

impl Add<Self> for Identifier {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0)
    }
}

impl Add<u64> for Identifier {
    type Output = Self;

    fn add(self, other: u64) -> Self {
        Self(self.0 + other)
    }
}

pub trait IdGenerator: Debug {
    /// Get next identifier
    fn next_id(&mut self) -> Identifier;

    /// Set the identifier to `id`
    fn reset(&mut self, id: Identifier);
}

/// Default identifier generator, not thread safe
#[derive(Debug, Default)]
pub struct DefaultIdGen {
    id: Identifier
}

impl DefaultIdGen {
    pub fn new(id: Identifier) -> Self {
        Self {
            id
        }
    }
}

impl IdGenerator for DefaultIdGen {
    fn next_id(&mut self) -> Identifier {
        let id = self.id.clone();
        self.id.inc();
        id
    }

    fn reset(&mut self, id: Identifier) {
        self.id = id;
    }
}
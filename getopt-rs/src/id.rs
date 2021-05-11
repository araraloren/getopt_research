
use std::hash::Hash;
use std::fmt::Debug;
use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default, Hash)]
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
    /// Get next [`Identifier`]
    fn next_id(&mut self) -> Identifier;

    /// Set the identifier to `id`
    fn reset(&mut self, id: Identifier);
}

/// Default [`Identifier`] generator, not thread safe
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

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_id_generate() {
        let mut gen = DefaultIdGen::new(Identifier::new(1));

        assert_eq!(gen.next_id(), Identifier::new(1));
        assert_eq!(gen.next_id(), Identifier::new(2));
        assert_eq!(gen.next_id(), Identifier::new(3));

        let mut id = gen.next_id();

        assert_eq!(id.get(), 4);
        assert_eq!(id.inc().get(), 5);
    }
}
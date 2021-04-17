
pub trait IdGenerator: std::fmt::Debug {
    fn next_id(&mut self) -> u64;

    fn reset(&mut self, id: u64);
}

#[derive(Debug)]
pub struct DefaultIdGen(u64);

impl DefaultIdGen {
    pub fn new() -> Self {
        Self(1)
    }

    pub fn new_start(id: u64) -> Self {
        Self(id)
    }
}

impl IdGenerator for DefaultIdGen {
    fn next_id(&mut self) -> u64 {
        let id = self.0;
        self.0 = self.0 + 1;
        return id;
    }

    fn reset(&mut self, id: u64) {
        self.0 = id;
    }
}

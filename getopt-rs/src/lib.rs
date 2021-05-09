
pub mod id;
pub mod error;
pub mod callback;
pub mod opt;
pub mod ctx;
pub mod set;
pub mod arg;
pub mod proc;
pub mod utils;
pub mod parser;
pub mod getopt;
pub mod nonopt;

#[macro_use]
extern crate log;

pub mod prelude {
    pub use crate::opt::{Style};
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

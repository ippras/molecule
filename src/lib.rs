#![feature(decl_macro)]
#![feature(lazy_cell)]

pub use atom;
pub use counter::{counter, Counter};
pub use error::{Error, Result};

mod counter;
mod error;

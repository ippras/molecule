#![feature(decl_macro)]
#![feature(lazy_cell)]

pub use self::saturation::{Saturable, Saturation};
pub use atom;
pub use counter::{counter, Counter};
pub use cu::Cu;
pub use error::{Error, Result};

mod counter;
mod cu;
mod error;
mod saturation;

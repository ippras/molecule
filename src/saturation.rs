use crate::{
    atom::{isotopes::*, Isotope},
    Counter,
};
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display, Formatter};

const H: Isotope = Isotope::H(H::One);
const C: Isotope = Isotope::C(C::Twelve);

/// Saturation
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum Saturation {
    Saturated,
    Unsaturated,
}

impl Display for Saturation {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Saturated if f.alternate() => f.write_str("Saturated"),
            Self::Unsaturated if f.alternate() => f.write_str("Unsaturated"),
            Self::Saturated => f.write_str("S"),
            Self::Unsaturated => f.write_str("U"),
        }
    }
}

/// Saturable
pub trait Saturable {
    fn unsaturated(&self) -> usize;

    fn saturated(&self) -> bool {
        self.unsaturated() == 0
    }

    fn saturation(&self) -> Saturation {
        if self.saturated() {
            Saturation::Saturated
        } else {
            Saturation::Unsaturated
        }
    }
}

impl Saturable for Counter {
    fn unsaturated(&self) -> usize {
        self.count(C) - self.count(H) / 2
    }
}

use crate::{
    atom::{isotopes::*, Isotope},
    counter, Counter,
};
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display, Formatter};

const H: Isotope = Isotope::H(H::One);
const C: Isotope = Isotope::C(C::Twelve);

// /// Equivalent carbon number
// ///
// /// `ECN = CN - 2DB`
// pub trait Ecn {
//     fn ecn(&self) -> usize;
// }

// impl Ecn for Counter {
//     fn ecn(&self) -> usize {
//         if self.is_empty() {
//             return 0;
//         }
//         let c = self.get(&C).expect("expected some `C` atoms").get();
//         let h = self.get(&H).expect("expected some `H` atoms").get();
//         h - c
//     }
// }

/// CU
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Cu {
    pub c: usize,
    pub u: usize,
}

impl Cu {
    pub const fn h(&self) -> usize {
        2 * self.c - 2 * self.u
    }
}

impl Display for Cu {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Display::fmt(&self.c, f)?;
        f.write_str(":")?;
        Display::fmt(&self.u, f)
    }
}

impl From<Cu> for (usize, usize) {
    fn from(value: Cu) -> Self {
        (value.c, value.u)
    }
}

impl From<Cu> for Counter {
    fn from(value: Cu) -> Self {
        let Cu { c, u } = value;
        counter! {
            C => c,
            H => 2 * c - 2 * u,
        }
    }
}

impl From<(usize, usize)> for Cu {
    fn from(value: (usize, usize)) -> Self {
        Self {
            c: value.0,
            u: value.1,
        }
    }
}

impl<'a> From<&'a Counter> for Cu {
    fn from(value: &'a Counter) -> Self {
        let c = value.get(&C).map_or(0, |c| c.get());
        let h = value.get(&H).map_or(0, |h| h.get());
        let u = c - h / 2;
        Self { c, u }
    }
}

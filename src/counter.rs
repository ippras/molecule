use crate::{Error, Result};
use atom::Isotope;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    fmt::{self, Display, Formatter},
    num::NonZeroUsize,
    ops::Deref,
    str::FromStr,
    sync::LazyLock,
};

pub macro counter {
    ($($key:expr => $value:expr),* $(,)*) => {{
        let mut _map = ::std::collections::BTreeMap::<Isotope, NonZeroUsize>::new();
        $(
            _map.entry(($key).into())
                .and_modify(|value| *value = value.saturating_add($value))
                .or_insert(NonZeroUsize::new($value).unwrap());
        )*
        Counter(_map)
    }}
}

/// Atom counter
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Counter(BTreeMap<Isotope, NonZeroUsize>);

impl Counter {
    pub fn new(counter: BTreeMap<Isotope, NonZeroUsize>) -> Self {
        Self(counter)
    }

    pub fn weight(&self) -> f64 {
        self.0
            .iter()
            .map(|(isotope, count)| isotope.relative_atomic_mass().value * count.get() as f64)
            .sum()
    }
}

impl Deref for Counter {
    type Target = BTreeMap<Isotope, NonZeroUsize>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for Counter {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        for (atom, count) in &self.0 {
            write!(f, "{atom:?}")?;
            if count.get() > 1 {
                write!(f, "{count}")?;
            }
        }
        Ok(())
    }
}

impl FromIterator<(Isotope, NonZeroUsize)> for Counter {
    fn from_iter<T: IntoIterator<Item = (Isotope, NonZeroUsize)>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl FromStr for Counter {
    type Err = Error;

    fn from_str(value: &str) -> Result<Self> {
        static ATOM_COUNT: LazyLock<Regex> = LazyLock::new(|| {
            Regex::new("([A-Z][a-z]*)([0-9]*)").expect("lazy static `ATOM_COUNT`")
        });

        let mut counter = BTreeMap::new();
        for captures in ATOM_COUNT.captures_iter(value) {
            let parsed = (
                captures[1].parse()?,
                match &captures[2] {
                    capture if capture.is_empty() => NonZeroUsize::MIN,
                    capture => capture.parse::<NonZeroUsize>()?,
                },
            );
            counter
                .entry(parsed.0)
                .and_modify(|count: &mut NonZeroUsize| {
                    *count = count.saturating_add(parsed.1.get())
                })
                .or_insert(parsed.1);
        }
        Ok(Self(counter))
    }
}

#[test]
fn test() {
    use atom::isotopes::{C, H, O};

    assert_eq!(
        Ok(counter! {
            C::Twelve => 2,
            H::One => 5,
            O::Sixteen => 1,
            H::One => 1,
        }),
        "C2H5OH".parse()
    );
}

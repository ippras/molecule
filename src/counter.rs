use crate::{Error, Result};
use atom::{Atom, ABRIDGED};
use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    fmt::{self, Display, Formatter},
    num::NonZeroUsize,
    ops::Deref,
    str::FromStr,
};

/// Atom counter
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Counter(BTreeMap<Atom, NonZeroUsize>);

impl Counter {
    pub fn new(counter: BTreeMap<Atom, NonZeroUsize>) -> Self {
        Self(counter)
    }

    pub fn weight(&self) -> f64 {
        self.0
            .iter()
            .map(|(atom, count)| atom.weight::<ABRIDGED>().unwrap().average() * count.get() as f64)
            .sum()
    }
}

impl Deref for Counter {
    type Target = BTreeMap<Atom, NonZeroUsize>;

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

impl FromIterator<(Atom, NonZeroUsize)> for Counter {
    fn from_iter<T: IntoIterator<Item = (Atom, NonZeroUsize)>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl FromStr for Counter {
    type Err = Error;

    fn from_str(value: &str) -> Result<Self> {
        lazy_static! {
            static ref ATOM_COUNT: Regex =
                Regex::new("([A-Z][a-z]*)([0-9]*)").expect("ATOM_COUNT regex");
        }

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
    use atom::Atom::{C, H, O};
    use maplit::btreemap;

    assert_eq!(
        Ok(Counter(btreemap! {
            H => NonZeroUsize::new(6).unwrap(),
            C => NonZeroUsize::new(2).unwrap(),
            O => NonZeroUsize::new(1).unwrap(),
        })),
        "C2H5OH".parse()
    );
}

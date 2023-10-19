use atom::isotopes::{C, H, O};
use molecule::counter;

#[test]
fn counter() {
    let counter = counter! {
        C::Twelve => 2,
        H::One => 5,
        O::Sixteen => 1,
        H::One => 1,
    };
    assert_eq!(counter.to_string(), "H6C2O");
    assert_eq!(Ok(counter), "C2H5OH".parse());
}

#[macro_use]
extern crate rugl;

use rugl::prelude::*;

// Default parameter test
fn main() {
    assert_eq!(rugl!(), Rugl{..Default::default()});
}
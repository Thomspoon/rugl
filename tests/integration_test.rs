#[macro_use]
extern crate rugl;

use rugl::prelude::*;

// Default parameter test
#[test]
fn rugl_default() {
    assert_eq!(rugl!(), Rugl{..Default::default()});
}
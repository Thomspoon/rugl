#[macro_use]
extern crate rugl;

use rugl::prelude::*;

// Invalid parameter test (nothing to parse)
fn main() {
    rugl!(
        test: {}
    ); //~ 9:9: 9:13: no rules expected the token `test`
}
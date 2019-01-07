#[macro_use]
extern crate rugl;
use rugl::*;

/// Invalid parameter test
fn main() {
    rugl!(); //~ ERROR unexpected end of macro invocation
}
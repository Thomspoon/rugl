#[macro_use]
extern crate rugl;

/// Invalid parameter test
fn main() {
    rugl_main!(test); //~ ERROR test is not a valid argument to rugl_main
}
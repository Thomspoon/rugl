use rugl_main;
use rugl_proc_macro::HelloWorld;


/// Invalid parameter test
fn main() {
    rugl_main!(test); //~ ERROR test is not a valid argument to rugl_main
}
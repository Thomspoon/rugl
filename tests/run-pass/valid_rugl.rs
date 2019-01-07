#[macro_use]
extern crate rugl;

fn main() {
    rugl!(
        vertex: Shader,
        fragment: Shader,
        attributes: Vec<Attribute>,
        uniforms: Vec<Uniform>
    );
}
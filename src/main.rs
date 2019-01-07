#[macro_use]
extern crate rugl;

use rugl::rugl::Rugl;
use rugl::shader::Shader;
use rugl::attribute::Attribute;
use rugl::uniform::Uniform;

fn main() {
    let r = rugl!(
        vertex: {
            "I am a vertex shader!"
        },
        fragment: {
            "I am a fragment shader!"
        },
        attributes: {
            position: [
                [1, 1],
                [1, 1]
            ],
            
            speed: [
                [1, 1]
            ],
        },
        uniforms: {
            color: [0.0, 0.1, 0.2, 0.3]
        }
    );

    println!("{:?}", r);
}
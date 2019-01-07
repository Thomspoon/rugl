/*! 
An ergonomic macro for creating themetic WebGL applications!

# Syntax

```ignore
rugl_main! {
    vertex: `
        precision mediump float;
        attribute vec2 position;
        void main() {
            gl_Position = vec4(position, 0, 1);
        }
    `;
    fragment: `
        precision mediump float;
        uniform vec3 color;
        void main() {
            gl_FragColor = color;
        }
    `;

    attributes: {
        position: [
            [-1, 0],
            [0, -1],
            [1, 1]
        ]
    }

    uniforms: {
        color: [1, 0, 0, 1]
    }
}
*/

use crate::shader::Shader;
use crate::attribute::Attribute;

trait HelloWorld {
    fn hello_world();
}

/// The internal Rugl struct holds the vertex and fragment shaders,
/// and internal pointers to any attributes and uniforms used in 
/// a design.
#[derive(Debug, HelloWorld)]
struct Rugl {
    vertex: Shader,
    fragment: Shader,
    attributes: Vec<Attribute>,
    uniforms: String
}

#[macro_export]
macro_rules! rugl_main {
    ($($i:ident: $j:expr),+) => {
        fn main() {
            Rugl {
                $($i: $j),*
            };
        }
    }
}
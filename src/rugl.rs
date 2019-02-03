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
    },

    count: 3
}
*/

use std::borrow::Cow;

use crate::shader::Shader;
use crate::attribute::Attribute;
use crate::uniform::Uniform;

/// The internal Rugl struct holds the vertex and fragment shaders,
/// and internal pointers to any attributes and uniforms used in 
/// a design.
#[derive(Debug, Default, PartialEq)]
pub struct Rugl<'a> {
    pub vertex: Cow<'a, Shader<'a>>,
    pub fragment: Cow<'a, Shader<'a>>,
    pub attributes: Vec<Attribute>,
    pub uniforms: Vec<Uniform>,
    pub count: i32
}

impl<'a> Rugl<'a> {
    pub fn get_vertex_shader(&self) -> &Shader {
        &self.vertex
    }

    pub fn get_fragment_shader(&self) -> &Shader {
        &self.fragment
    }

    pub fn get_attributes(&self) -> &Vec<Attribute> {
        &self.attributes
    }
    
    pub fn get_uniforms(&self) -> &Vec<Uniform> {
        &self.uniforms
    }

    pub fn get_count(&self) -> &i32 {
        &self.count
    }
}

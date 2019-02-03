
use std::borrow::Cow;
use std::marker::PhantomData;

use wasm_bindgen::prelude::*;

mod rugl;
mod shader;

#[macro_use]
mod macros;

mod qualifier;
mod attribute;
mod uniform;
pub mod webgl;

//pub mod prelude {
#[doc(no_inline)]
pub use crate::rugl::Rugl;
pub use crate::attribute::Attribute;
pub use crate::uniform::Uniform;
pub use crate::shader::Shader;
pub use crate::webgl::*;
//}

use crate::webgl::{Primitive, WebGlContext, ShaderType};

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let r = rugl!(
        vertex: {
            r#"
                attribute vec4 position;
                void main() {
                    gl_Position = position;
                }
            "#
        },
        fragment: {
            r#"
                void main() {
                    gl_FragColor = vec4(0.5, 1.0, 1.0, 1.0);
                }
            "#
        },
        attributes: {
            position: [
                [-0.7, -0.7, 0.0],
                [ 0.7, -0.7, 0.0], 
                [ 0.0,  0.7, 0.0]
            ],
        },
        uniforms: {
            color: [0.0, 0.1, 0.2, 0.3]
        },

        count: { 3 }
    );

    let mut webgl = WebGlContext::new("canvas")?;

    webgl.clear_with_color([0.2, 0.3, 0.1, 1.0]);

    let vertex = webgl.compile_shader(
        ShaderType::Vertex(
            r.get_vertex_shader().inner(),
            PhantomData
        )
    )?;

    let fragment = webgl.compile_shader(
        ShaderType::Fragment(
            r.get_fragment_shader().inner(),
            PhantomData
        )
    )?;

    webgl.link_and_add_program(&[vertex, fragment])?;
    webgl.use_program()?;

    for attribute in r.get_attributes() {
        // Unpack Vec<Qualifer>
        let mut attr_data = Vec::new();
        for layer in attribute.get_qualifiers() {
            attr_data.extend_from_slice(&layer.to_vec());
        }

        webgl.create_buffer_with_data(Primitive::Attribute, attribute.get_name(), &attr_data[..], *r.get_count())?;
        webgl.bind_buffer_with_name(Primitive::Attribute, attribute.get_name())?;
        webgl.enable_attribute(attribute.get_name())?;
       
    }

    webgl.draw_triangles(*r.get_count());

    log!("{:?}", r);

    Ok(())
}

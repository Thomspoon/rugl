use wasm_bindgen::prelude::*;

mod rugl;
mod shader;

#[macro_use]
mod macros;

mod qualifier;
mod attribute;
mod uniform;
mod webgl;

pub mod prelude {
    #[doc(no_inline)]
    pub use crate::rugl::Rugl;
    pub use crate::attribute::Attribute;
    pub use crate::uniform::Uniform;
    pub use crate::shader::Shader;
    pub use crate::webgl::*;
}

use crate::webgl::{Primitive, WebGlContext, ShaderType};

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let mut webgl = WebGlContext::new("canvas")?;

    webgl.clear_with_color([0.1, 0.2, 0.3, 1.0]);

    let vertex = webgl.compile_shader(
        ShaderType::Vertex(
            r#"
                attribute vec4 position;
                void main() {
                    gl_Position = position;
                }
            "#
        )
    )?;

    let fragment = webgl.compile_shader(
        ShaderType::Fragment(
            r#"
                void main() {
                    gl_FragColor = vec4(0.5, 1.0, 1.0, 1.0);
                }
            "#
        )
    )?;

    webgl.link_and_add_program(&[vertex, fragment])?;
    webgl.use_program()?;

    let position: [f32; 9] = [-0.7, -0.7, 0.0, 0.7, -0.7, 0.0, 0.0, 0.7, 0.0];

    webgl.create_buffer_with_data(Primitive::Attribute, "position", &position[..], 3)?;
    webgl.bind_buffer_with_name(Primitive::Attribute, "position")?;
    webgl.enable_attribute("position")?;
    webgl.draw_triangles(3);

    Ok(())
}

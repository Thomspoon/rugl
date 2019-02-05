use wasm_bindgen::prelude::*;

mod rugl;

#[macro_use]
mod macros;

pub mod webgl;

//pub mod prelude {
#[doc(no_inline)]
pub use crate::rugl::{Rugl, RuglInner};
pub use crate::webgl::*;
//}

use crate::webgl::WebGlContext;

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let _ = rugl!(
        vertex: {
            "
                attribute vec4 position;
                void main() {
                    gl_Position = position;
                }
            "
        },
        fragment: {
            "
                precision mediump float;
                uniform vec4 color;

                void main() {
                    gl_FragColor = color;
                }
            "
        },
        attributes: {
            position: [
                [-0.7, -0.7, 0.0],
                [ 0.7, -0.7, 0.0],
                [ 0.0,  0.7, 0.0]
            ],
        },
        uniforms: {
            color: [0.0, 0.9, 0.5, 0.3]
        },

        count: { 3 }
    )
    .unwrap()
    .step()?;

    Ok(())
}

mod rugl;
#[macro_use]
mod macros;
mod webgl;

#[macro_export]
macro_rules! rugl_main {
    ($($tt:tt)*) => {
        use wasm_bindgen::*;

        #[wasm_bindgen(start)]
        pub fn start() -> Result<(), JsValue> {
            let mut rugl = rugl_inner!($($tt)*)?;

            let dynamic = rugl.is_dynamic();
            if dynamic {
                // Define an empty function and clone it
                let dynamic_fn_empty = Rc::new(RefCell::new(None));
                let dynamic_fn = dynamic_fn_empty.clone();

                // Overwrite dynamic function
                *dynamic_fn.borrow_mut() = Some(Closure::wrap(Box::new( move || {
                    let _ = rugl.step();
                    request_animation_frame(
                        dynamic_fn_empty
                            .borrow()
                            .as_ref()
                            .unwrap()
                    );
                }) as Box<FnMut()>));

                request_animation_frame(
                    dynamic_fn
                        .borrow()
                        .as_ref()
                        .unwrap()
                );
            } else {
                let _ = rugl.step();
            }

            Ok(())
        }

        fn window() -> web_sys::Window {
            web_sys::window().expect("no global `window` exists")
        }

        pub fn request_animation_frame(f: &Closure<FnMut()>) {
            window()
                .request_animation_frame(f.as_ref().unchecked_ref())
                .expect("should register `requestAnimationFrame` OK");
        }
    }
}

#[macro_use]
pub mod prelude {
    pub use super::*;
    pub use crate::rugl::{Rugl, RuglInner};
    pub use crate::webgl::*;
    pub use rugl_main as rugl;
    pub use std::rc::Rc;
    pub use std::cell::RefCell;
    pub use js_sys::*;
    pub use web_sys;
    pub use wasm_bindgen;
    pub use wasm_bindgen::prelude::*;
    pub use wasm_bindgen::JsCast;
}

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
            let _ = rugl_inner!($($tt)*)
                .unwrap()
                .step()?;

            Ok(())
        }

    }
}

#[macro_use]
pub mod prelude {
    pub use super::*;

    pub use crate::rugl::{Rugl, RuglInner};
    pub use crate::webgl::*;
    pub use wasm_bindgen::prelude::*;
    pub use rugl_main as rugl;
}
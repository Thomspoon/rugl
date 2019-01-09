pub mod rugl;
pub mod shader;

#[macro_use]
mod macros;

pub mod qualifier;
pub mod attribute;
pub mod uniform;

pub mod prelude {
    #[doc(no_inline)]
    pub use crate::rugl::Rugl;
    pub use crate::attribute::Attribute;
    pub use crate::uniform::Uniform;
    pub use crate::shader::Shader;
}

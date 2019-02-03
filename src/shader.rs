/*!
The VertexShader class encapsulates any datatype that can be coerced to a string.
 */

#[derive(Debug, PartialEq)]
pub struct Shader {
    internal: String
}

impl Shader {
    pub fn new<T: Into<String>>(internal: T) -> Self {
        Self {
            internal: internal.into()
        }
    }
}

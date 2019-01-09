/*!
The VertexShader class encapsulates any datatype that can be coerced to a string.
 */

#[derive(Debug, PartialEq)]
pub struct Shader {
    internal: String
}

impl From<String> for Shader {
    fn from(internal: String) -> Self {
        Self {
            internal
        }
    }
}

impl<'a> From<&'a str> for Shader {
    fn from(internal: &'a str) -> Self {
        Self {
            internal: internal.to_owned()
        }
    }
}
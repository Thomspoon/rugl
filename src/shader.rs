/*!
The VertexShader class encapsulates any datatype that can be coerced to a string.
 */

use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Shader<'a> {
    internal: Cow<'a, str>
}

impl<'a> Shader<'a> {
    pub fn new<T: Into<&'a str>>(internal: T) -> Self {
        Self {
            internal: Cow::from(internal.into())
        }
    }

    pub fn inner(&'a self) -> &'a str {
        // TODO: This is wonky
        self.internal.as_ref()
    }
}

/// WebGlBuffer wrapper
use web_sys::{WebGlBuffer, WebGlUniformLocation};

use crate::webgl::JsArray;
use crate::webgl::UniformInner;

pub enum BufferInternal {
    Attribute(JsArray, u32),
    Uniform(UniformInner, WebGlUniformLocation),
}

impl BufferInternal {
    pub fn get_attribute(&self) -> (&JsArray, &u32) {
        match self {
            BufferInternal::Attribute(data, location) => (data, location),
            BufferInternal::Uniform(_, _) => panic!("Not an attribute!"),
        }
    }

    pub fn get_uniform(&self) -> (&UniformInner, &WebGlUniformLocation) {
        match self {
            BufferInternal::Attribute(_, _) => panic!("Not a uniform!"),
            BufferInternal::Uniform(data, location) => (data, location),
        }
    }
}

pub struct Buffer {
    buffer: Option<WebGlBuffer>,
    data: BufferInternal,
    count: i32,
}

impl Buffer {
    pub fn new(buffer: Option<WebGlBuffer>, data: BufferInternal, count: i32) -> Self {
        Self {
            buffer,
            data,
            count,
        }
    }

    pub fn get_buffer(&self) -> Option<&WebGlBuffer> {
        self.buffer.as_ref()
    }

    pub fn get_data(&self) -> &BufferInternal {
        &self.data
    }

    pub fn get_count(&self) -> &i32 {
        &self.count
    }
}

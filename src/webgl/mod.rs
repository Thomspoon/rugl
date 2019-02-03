/// Webassembly Context

use std::borrow::Cow;
use std::mem::size_of_val;
use std::collections::HashMap;

mod shader;
mod program;

pub use shader::{Shader, ShaderType};
pub use program::Program;
use web_sys::{WebGlBuffer, WebGlRenderingContext};
use js_sys::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

pub struct WebGlContext {
    context: WebGlRenderingContext,
    _canvas: web_sys::HtmlCanvasElement,
    program: Option<Program>,
    attributes: HashMap<String, Buffer>,
    uniforms: HashMap<String, Buffer>,
    memory: JsValue
}

impl WebGlContext {
    pub fn new<'a, T: Into<&'a str>>(
        id: T
    ) -> Result<Self, JsValue> {
        let document = web_sys::window().unwrap().document().unwrap();
        let _canvas = document
            .get_element_by_id(id.into())
            .ok_or_else(|| String::from("Unable to get Canvas element!"))?;
        let _canvas: web_sys::HtmlCanvasElement = _canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;

        let context = _canvas
            .get_context("webgl")?
            .unwrap()
            .dyn_into::<WebGlRenderingContext>()?;

        let memory = wasm_bindgen::memory()
            .dyn_into::<WebAssembly::Memory>()?
            .buffer();

        Ok(WebGlContext {
            context,
            _canvas,
            program: None,
            attributes: HashMap::new(),
            uniforms: HashMap::new(),
            memory
        })
    }

    /// Compile shaders
    pub fn compile_shader<'a, T: Into<Cow<'a, str>>>(
        &self, 
        shader: ShaderType<'a, T>
    ) -> Result<Shader, String> {
        Shader::new(&self.context, shader)
    }

    /// Link shaders to program
    pub fn link_program<'a, Shaders: IntoIterator<Item = &'a Shader>>(
        &mut self, 
        shaders: Shaders
    ) -> Result<Program, String> {
        Program::new(&self.context, shaders.into_iter())
    }

    /// Link shaders to program, adding the program to our internal hashmap
    pub fn link_and_add_program<'a, Shaders: IntoIterator<Item = &'a Shader>>(
        &mut self, 
        shaders: Shaders
    ) -> Result<(), String> {
        let program = self.link_program(shaders)?;
        let _ = std::mem::replace(&mut self.program, Some(program));
        Ok(())
    }

    /// Use internal program
    pub fn use_program(&self) -> Result<(), String> {
        match &self.program {
            Some(program) => { self.context.use_program(Some(&program.inner())); Ok(()) }
            None => Err(String::from("Program has not been setup yet!"))
        }
    }

    /// Create a buffer
    pub fn create_buffer(&self) -> Result<WebGlBuffer, String> {
        let buffer = self.context.create_buffer().ok_or("Unable to create buffer")?;
        Ok(buffer)
    }

    pub fn create_buffer_with_data<'a, Name: Into<Cow<'a, str>>, Type: FromSlice>(
        &mut self, 
        primitive: Primitive, 
        name: Name, 
        data: Type,
        count: i32,
    ) -> Result<(), String> {
        let qualifer_name = name.into();
        let buffer = self.create_buffer()?;
        let data = FromSlice::from_slice(&mut self.memory, data);

        match primitive {
            Primitive::Attribute => {
                let position = self.context.get_attrib_location(
                    self.program
                        .as_ref()
                        .unwrap()
                        .inner(), 
                    &qualifer_name
                );

                if position < 0 {
                    log!("The qualifer did not exist");
                } 

                self.attributes.insert(qualifer_name.into_owned(), Buffer::new(
                    buffer,
                    data,
                    position,
                    count
                ))
            },
            Primitive::Uniform => {
                // TODO: Uniform locations have a struct, not an i32, fix the buffer
                //       to either account for both, or split up the buffers

                // let position = self.context.get_uniform_location(
                //     self.program
                //         .as_ref()
                //         .unwrap()
                //         .inner(), 
                //     &qualifer_name
                // ).unwrap().into();

                // if position < 0 {
                //     log!("The qualifer did not exist");
                // } 

                self.attributes.insert(qualifer_name.into_owned(), Buffer::new(
                    buffer,
                    data,
                    0,
                    count
                ))
            }
        };

        Ok(())
    }

    /// Bind an array to the context
    pub fn bind_buffer_with_name<'a, Name: Into<Cow<'a, str>>>(
        &self, 
        primitive: Primitive, 
        name: Name
    ) -> Result<(), String> {
        match primitive {
            Primitive::Attribute => {
                let attribute = self.attributes.get(&name.into().into_owned());
                match attribute {
                    Some(attribute) => { 
                        self.context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(attribute.get_buffer())); 
                        self.context.buffer_data_with_array_buffer_view(
                            WebGlRenderingContext::ARRAY_BUFFER,
                            attribute.get_data().to_object(),
                            WebGlRenderingContext::STATIC_DRAW,
                        );
                        Ok(()) 
                    }
                    None => Err(String::from("Attribute does not exist!"))
                }
            },
            Primitive::Uniform => {
                let uniform = self.uniforms.get(&name.into().into_owned());
                match uniform {
                    Some(uniform) => { 
                        self.context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(uniform.get_buffer())); 
                        self.context.buffer_data_with_array_buffer_view(
                            WebGlRenderingContext::ARRAY_BUFFER,
                            uniform.get_data().to_object(),
                            WebGlRenderingContext::STATIC_DRAW,
                        );
                        Ok(()) 
                    }
                    None => Err(String::from("Uniform does not exist!"))
                }
            }
        }
    }

    /// Clear and set background color
    pub fn clear_with_color(&self, color: [f32; 4]) {
        self.context.clear_color(color[0], color[1], color[2], color[3]);
        self.context.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);
    }

    pub fn enable_attribute<'a, T: Into<Cow<'a, str>>>(&self, name: T) -> Result<(), String> {
        let attribute = self.attributes.get(&name.into().into_owned());
        match attribute {
            Some(attribute) => { 
                self.context.vertex_attrib_pointer_with_i32(
                    *attribute.get_position() as _, 
                    *attribute.get_count() as _, 
                    WebGlRenderingContext::FLOAT, 
                    false, 0, 0
                );
                self.context.enable_vertex_attrib_array(0);
                Ok(())
            }
            None => Err(String::from("Attribute does not exist!"))
        }
    }

    /// Draw triangles
    pub fn draw_triangles(
        &self,
        count: i32
    ) {
        self.context.draw_arrays(
            WebGlRenderingContext::TRIANGLES,
            0,
            count,
        );
    }

    pub fn context(&self) -> &WebGlRenderingContext {
        &self.context
    }
}

pub enum JsArray {
    Uint8Array(Uint8Array),
    Uint16Array(Uint16Array),
    Uint32Array(Uint32Array),
    Int8Array(Int8Array),
    Int16Array(Int16Array),
    Int32Array(Int32Array),
    Float32Array(Float32Array),
    Float64Array(Float64Array)
}

impl JsArray {
    fn to_object(&self) -> &Object {
        match self {
            JsArray::Uint8Array(arr)   => arr.as_ref(), 
            JsArray::Uint16Array(arr)  => arr.as_ref(), 
            JsArray::Uint32Array(arr)  => arr.as_ref(), 
            JsArray::Int8Array(arr)    => arr.as_ref(), 
            JsArray::Int16Array(arr)   => arr.as_ref(), 
            JsArray::Int32Array(arr)   => arr.as_ref(), 
            JsArray::Float32Array(arr) => arr.as_ref(), 
            JsArray::Float64Array(arr) => arr.as_ref()
        }
    }
}

pub trait FromSlice {
    fn from_slice(memory: &mut JsValue, data: Self) -> JsArray;
}

macro_rules! from_slice {
    ($type:ty, $id:ident) => {
        impl FromSlice for $type {
            fn from_slice(memory: &mut JsValue, data: $type) -> JsArray {
                let data_size = data.len();
                let data_pointer = data.as_ptr() as u32;
                let data_normilization = size_of_val(&data[0]) as u32;
                let data_location = data_pointer / data_normilization;

                // Check to see if our memory was resized before we use it
                // TODO: Handle error case
                let memory_buffer = wasm_bindgen::memory()
                        .dyn_into::<WebAssembly::Memory>().unwrap()
                .buffer();

                // Replace our internal memory
                if memory_buffer != *memory {
                    std::mem::replace(memory, memory_buffer);
                } 

                // Return js_sys value
                JsArray::$id($id::new(memory).subarray(data_location, data_location + data_size as u32))
            }
        }
    };
}

from_slice!(&[u8],  Uint8Array);
from_slice!(&[u16], Uint16Array);
from_slice!(&[u32], Uint32Array);
from_slice!(&[i8],  Int8Array);
from_slice!(&[i16], Int16Array);
from_slice!(&[i32], Int32Array);
from_slice!(&[f32], Float32Array);
from_slice!(&[f64], Float64Array);

pub enum Primitive {
    Attribute,
    Uniform
}

pub struct Buffer {
    buffer: WebGlBuffer,
    data: JsArray,
    position: i32,
    count: i32
}

impl Buffer {
    pub fn new(buffer: WebGlBuffer, data: JsArray, position: i32, count: i32) -> Self {
        Self {
            buffer, 
            data,
            position,
            count
        }
    }

    pub fn get_buffer(&self) -> &WebGlBuffer {
        &self.buffer
    }

    pub fn get_data(&self) -> &JsArray {
        &self.data
    }

    pub fn get_position(&self) -> &i32 {
        &self.position
    }

    pub fn get_count(&self) -> &i32 {
        &self.count
    }
}
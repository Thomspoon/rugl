/*!
An ergonomic macro for creating themetic stateless WebGL applications!

# Syntax

```ignore
rugl_main! {
    vertex: "
        precision mediump float;
        attribute vec2 position;
        void main() {
            gl_Position = vec4(position, 0, 1);
        }
    ";
    fragment: "
        precision mediump float;
        uniform vec3 color;
        void main() {
            gl_FragColor = color;
        }
    ";

    attributes: {
        position: [
            [-1, 0],
            [0, -1],
            [1, 1]
        ]
    }

    uniforms: {
        color: [1, 0, 0, 1]
    },

    count: 3
}
*/

use std::borrow::Cow;

use crate::webgl::{Attribute, Uniform, WebGlContext};

#[derive(Debug)]
pub struct Rugl<'a> {
    pub inner: RuglInner<'a>,
    pub context: WebGlContext,
}

impl Rugl<'_> {
    pub fn step(&mut self) -> Result<(), String> {
        self.context.clear_with_color([1.0, 1.0, 1.0, 1.0]);

        for attribute in self.inner.get_attributes() {
            self.context.enable_attribute(attribute.get_name())?;
        }

        self.context.draw_triangles(*self.inner.get_count());

        Ok(())
    }
}

/// The internal Rugl struct holds the vertex and fragment shaders,
/// and internal vectors to any attributes and uniforms used in
/// a design.
#[derive(Debug)]
pub struct RuglInner<'a> {
    pub vertex: Cow<'a, str>,
    pub fragment: Cow<'a, str>,
    pub attributes: Vec<Attribute>,
    pub uniforms: Vec<Uniform>,
    pub count: i32,
}

impl<'a> RuglInner<'a> {
    pub fn get_vertex_shader(&self) -> &str {
        &self.vertex
    }

    pub fn get_fragment_shader(&self) -> &str {
        &self.fragment
    }

    pub fn get_attributes(&mut self) -> &Vec<Attribute> {
        &self.attributes
    }

    pub fn get_mut_attributes(&mut self) -> &mut Vec<Attribute> {
        &mut self.attributes
    }

    pub fn get_uniforms(&self) -> &Vec<Uniform> {
        &self.uniforms
    }

    pub fn get_mut_uniforms(&mut self) -> &mut Vec<Uniform> {
        &mut self.uniforms
    }

    pub fn get_count(&self) -> &i32 {
        &self.count
    }
}

/// A macro that enables giving named-arguments to the rugl struct, and sets up the WebGlContext
#[macro_export]
macro_rules! rugl_inner {
    (
        $( $i:ident: { $($tokens:tt)* } ),*
    ) => {{
        #[inline]
        fn build_inner<'a>() -> Result<(RuglInner<'a>, WebGlContext), JsValue> {
            use std::borrow::Cow;

            let mut context = WebGlContext::new("canvas")?;

            let mut inner = RuglInner {
                $($i: rugl_type!($i: $($tokens)*),)*
            };

            let vertex = context.compile_shader(
                ShaderType::Vertex(
                    inner.get_vertex_shader(),
                    std::marker::PhantomData
                )
            )?;

            let fragment = context.compile_shader(
                ShaderType::Fragment(
                    inner.get_fragment_shader(),
                    std::marker::PhantomData
                )
            )?;

            context.link_and_add_program(&[vertex, fragment])?;
            context.use_program()?;

            let count = inner.get_count().clone();

            for attribute in inner.get_mut_attributes() {

                // Unpack Vec<Qualifer>
                let mut attr_data = Vec::new();
                for layer in attribute.get_qualifiers() {
                    attr_data.extend_from_slice(&layer.to_vec());
                }

                context.create_buffer_with_data(attribute.get_name(), &attr_data[..], count)?;
                context.bind_buffer_with_name(attribute.get_name())?;
                context.enable_attribute(attribute.get_name())?;
            }

            for uniform in inner.get_mut_uniforms() {
                context.create_uniform(uniform.get_name(), uniform.inner())?;
                context.bind_uniform(uniform.get_name())?;
            }


            Ok((inner, context))
        }

        match build_inner() {
            Ok((inner, context)) => Ok(Rugl { inner, context }),
            Err(err) => {
                //TODO: Proper error handling
                log!("There was an error! {}", err.as_string().unwrap());
                Err("There was a problem!!!".to_owned())
            }
        }
    }}
}

#[doc(hidden)]
#[macro_export]
macro_rules! rugl_type {
    (vertex: $($tokens:tt)+) => {
        Cow::Borrowed($($tokens)*)
    };
    (fragment: $($tokens:tt)+) => {
        Cow::Borrowed($($tokens)*)
    };
    (attributes: $($tokens:tt)+) => {
        parse_ident!(@attribute $($tokens)*)
    };
    (uniforms: $($tokens:tt)+) => {
        parse_ident!(@uniform $($tokens)*)
    };
    (count: $expr:expr) => {
        $expr
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! parse_ident {
    // Parse each attribute to generate ident and data array
    (@attribute
        $($id:ident: [$($tokens:tt)*]),+ $(,)*
    ) => {
        vec![$( Attribute::from((stringify!($id).to_owned(), determine_bracket_replace!($($tokens)*)) )),*]
    };
    // Parse each uniform to generate ident and data array
    (@uniform
        $($id:ident: [$($tokens:tt)*]),+ $(,)*
    ) => {
        vec![$( Uniform::from((stringify!($id).to_owned(), UniformInner::from(determine_bracket_replace!($($tokens)*))) )),*]
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! determine_bracket_replace {
    // Replace brackets with parenthesis and return as an array
    ($([$($tokens:tt)*]),*) => {
        [ $( ($($tokens)*) ),* ]
    };

    // Not actually replacing brackets, just passing to an array
    ($($tokens:tt)*) => {
        [ $($tokens)* ]
    }
}

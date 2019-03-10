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

pub struct Rugl<'a> {
    pub inner: RuglInner<'a>,
    pub context: WebGlContext,
}

impl Rugl<'_> {
    pub fn step(&mut self) -> Result<(), String> {
        self.context.clear_with_color(self.inner.clear);

        for attribute in self.inner.get_attributes() {
            self.context.enable_attribute(attribute.get_name())?;
        }

        let tick = self.inner.tick;
        for uniform in self.inner.get_mut_uniforms() {
            self.context.update_uniform(uniform.get_name(), tick)?;
        }

        self.context.draw_triangles(*self.inner.get_count());

        self.inner.tick += 0.001;

        Ok(())
    }
    pub fn is_dynamic(&self) -> bool {
        self.inner.is_dynamic()
    }
}

/// The internal Rugl struct holds the vertex and fragment shaders,
/// and internal vectors to any attributes and uniforms used in
/// a design.
#[derive(Default)]
pub struct RuglInner<'a> {
    /// Clear color
    pub clear: [f64; 4],
    /// Application vertex shader
    pub vertex: Cow<'a, str>,
    /// Application fragment shader
    pub fragment: Cow<'a, str>,
    /// Collection of attributes in the application
    pub attributes: Vec<Attribute>,
    /// Collection of uniforms in the application
    pub uniforms: Vec<Uniform>,
    /// Number of primitives to draw
    pub count: i32,
    /// Determine if we need to dynamically update the screen
    pub dynamic: bool, 
    /// Internal time tick
    pub tick: f64,
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

    pub fn is_dynamic(&self) -> bool {
        self.dynamic
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
                ..Default::default()
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

            let mut dynamic = false;
            for uniform in inner.get_mut_uniforms() {
                if uniform.get_data().is_dynamic() {
                    dynamic = true;
                }
                context.create_uniform(uniform.get_name(), uniform.get_data())?;
                context.bind_uniform(uniform.get_name())?;
            }

            inner.dynamic = dynamic;

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
        vec![$( Attribute::from((stringify!($id).to_owned(), [$($tokens)*])) ),*]
    };
    // Parse uniform to generate variable-use function
    (@uniform
        $($tokens:tt)*
    ) => {{
        let mut uniforms: Vec<Uniform> = Vec::new();
        parse_ident!(@uniform_inner uniforms, $($tokens)*);
        uniforms
    }};
    // Uniform Array
    (@uniform_inner $expr:expr, $id:ident: [$($tokens:expr),*] ) => {
        $expr.push(Uniform::from((stringify!($id).to_owned(), UniformInner::from([$($tokens),*]))));
    };
    (@uniform_inner $expr:expr, $id:ident: [$($tokens:expr),*], $($extra:tt)* ) => {
        parse_ident!(@uniform_inner $expr, $id: [$($tokens),*]);
        parse_ident!(@uniform_inner $expr, $($extra)* );
    };
    // Uniform Function without type
    (@uniform_inner $expr:expr, $id:ident: |$($fn_head:ident),*| $fn_body:expr ) => {
        $expr.push(Uniform::from((stringify!($id).to_owned(), UniformInner::from(Rc::new(|$($fn_head),*| $fn_body) as Rc<Fn(f64)->_>))));
    };
    (@uniform_inner $expr:expr, $id:ident: |$($fn_head:ident),*| $fn_body:expr, $($extra:tt)* ) => {
        parse_ident!(@uniform_inner $expr, $id: |$($fn_head),*| $fn_body );
        parse_ident!(@uniform_inner $expr, $($extra)* );
    };
    // Uniform Function with type
    (@uniform_inner $expr:expr, $id:ident: |$($fn_head:ident: $fn_type:ty),*| $fn_body:expr ) => {
        $expr.push(Uniform::from((stringify!($id).to_owned(), UniformInner::from(Rc::new(|$($fn_head: $fn_type),*| $fn_body) as Rc<Fn(f64)->_>))));
    };
    (@uniform_inner $expr:expr, $id:ident: |$($fn_head:ident: $fn_type:ty),*| $fn_body:expr, $($extra:tt)* ) => {        
        parse_ident!(@uniform_inner $expr, $id: |$($fn_head: $fn_type),*| $fn_body );
        parse_ident!(@uniform_inner $expr, $($extra)* );
    };
    // Uniform Direct Expression
    (@uniform_inner $expr:expr, $id:ident: $data:expr ) => {
        $expr.push(Uniform::from((stringify!($id).to_owned(), UniformInner::from($data))));
    };
    (@uniform_inner $expr:expr, $id:ident: $data:expr, $($extra:tt)* ) => {
        parse_ident!(@uniform_inner $expr, $id: $data );
        parse_ident!(@uniform_inner $expr, $($extra)* );
    };
    // Empty base for trailing commas
    (@uniform_inner $expr:expr,) => {
    };
}

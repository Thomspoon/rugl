/// WebGl Shader
use std::borrow::Cow;
use std::marker::PhantomData;

use web_sys::{WebGlRenderingContext, WebGlShader};

#[derive(Debug)]
pub enum ShaderType<'a, T: Into<Cow<'a, str>>> {
    Vertex(T, PhantomData<&'a T>),
    Fragment(T, PhantomData<&'a T>),
}

impl<'a, T: Into<Cow<'a, str>>> ShaderType<'a, T> {
    pub fn into_inner(self) -> String {
        match self {
            ShaderType::Vertex(inner, _) => inner.into().into_owned(),
            ShaderType::Fragment(inner, _) => inner.into().into_owned(),
        }
    }

    pub fn into_gl_type(&self) -> u32 {
        match self {
            ShaderType::Vertex(_, _) => WebGlRenderingContext::VERTEX_SHADER,
            ShaderType::Fragment(_, _) => WebGlRenderingContext::FRAGMENT_SHADER,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Shader {
    internal: WebGlShader,
}

impl Shader {
    pub fn new<'a, T: Into<Cow<'a, str>>>(
        context: &WebGlRenderingContext,
        shader: ShaderType<'a, T>,
    ) -> Result<Shader, String> {
        let shader_context = context
            .create_shader(shader.into_gl_type())
            .ok_or_else(|| String::from("Unable to create shader object"))?;

        context.shader_source(&shader_context, &shader.into_inner());
        context.compile_shader(&shader_context);

        if context
            .get_shader_parameter(&shader_context, WebGlRenderingContext::COMPILE_STATUS)
            .as_bool()
            .unwrap_or(false)
        {
            Ok(Shader {
                internal: shader_context,
            })
        } else {
            Err(context
                .get_shader_info_log(&shader_context)
                .unwrap_or_else(|| "Unknown error creating shader".into()))
        }
    }

    pub fn as_ref(&self) -> &WebGlShader {
        &self.internal
    }
}

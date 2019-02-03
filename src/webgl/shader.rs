/// WebGl Shader

use web_sys::{WebGlShader, WebGlRenderingContext};

pub enum ShaderType<T: Into<String>> {
    Vertex(T),
    Fragment(T)
}

impl<T: Into<String>> ShaderType<T> {
    pub fn into_inner(self) -> String {
        match self {
            ShaderType::Vertex(inner) => inner.into(),
            ShaderType::Fragment(inner) => inner.into()
        }
    }

    pub fn into_gl_type(&self) -> u32 {
        match self {
            ShaderType::Vertex(_) => WebGlRenderingContext::VERTEX_SHADER,
            ShaderType::Fragment(_) => WebGlRenderingContext::FRAGMENT_SHADER
        }
    }
}

pub struct Shader {
    internal: WebGlShader
}

impl Shader {
    pub fn new<'a, T: Into<String>>(
        context: &WebGlRenderingContext, 
        shader: ShaderType<T>
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
            Ok(Shader{ internal: shader_context })
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

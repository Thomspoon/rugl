/*!
The Uniform class holds a vector of the inner types and offers many coersions of other types.
 */

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UniformInner {
    Uniform1i(i32),
    Uniform1f(f32),
    Uniform2i(i32, i32),
    Uniform2f(f32, f32),
    Uniform3i(i32, i32, i32),
    Uniform3f(f32, f32, f32),
    Uniform4i(i32, i32, i32, i32),
    Uniform4f(f32, f32, f32, f32),
}

#[derive(Debug, PartialEq)]
pub struct Uniform {
    name: String,
    data: UniformInner,
}

impl Uniform {
    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn inner(&self) -> UniformInner {
        self.data
    }
}

impl From<(String, UniformInner)> for Uniform {
    fn from(uniform: (String, UniformInner)) -> Self {
        Uniform {
            name: uniform.0,
            data: UniformInner::from(uniform.1),
        }
    }
}

impl From<i32> for UniformInner {
    fn from(item: i32) -> Self {
        UniformInner::Uniform1i(item)
    }
}

impl From<f32> for UniformInner {
    fn from(item: f32) -> Self {
        UniformInner::Uniform1f(item)
    }
}

impl From<[i32; 2]> for UniformInner {
    fn from(item: [i32; 2]) -> Self {
        UniformInner::Uniform2i(item[0], item[1])
    }
}

impl From<[i32; 3]> for UniformInner {
    fn from(item: [i32; 3]) -> Self {
        UniformInner::Uniform3i(item[0], item[1], item[2])
    }
}

impl From<[f32; 2]> for UniformInner {
    fn from(item: [f32; 2]) -> Self {
        UniformInner::Uniform2f(item[0], item[1])
    }
}

impl From<[f32; 3]> for UniformInner {
    fn from(item: [f32; 3]) -> Self {
        UniformInner::Uniform3f(item[0], item[1], item[2])
    }
}

impl From<[f32; 4]> for UniformInner {
    fn from(item: [f32; 4]) -> Self {
        UniformInner::Uniform4f(item[0], item[1], item[2], item[3])
    }
}

/*!
The Uniform class holds a vector of the inner types and offers many coersions of other types.
 */

use std::rc::Rc;

#[derive(Clone)]
pub enum UniformInner {
    Uniform1i(i32, Option<Rc<Fn(f64) -> i32>>),
    Uniform1f(f64, Option<Rc<Fn(f64) -> f64>>),
    Uniform2i(i32, i32, Option<Rc<Fn(f64) -> [i32; 2]>>),
    Uniform2f(f64, f64, Option<Rc<Fn(f64) -> [f64; 2]>>),
    Uniform3i(i32, i32, i32, Option<Rc<Fn(f64) -> [i32; 3]>>),
    Uniform3f(f64, f64, f64, Option<Rc<Fn(f64) -> [f64; 3]>>),
    Uniform4i(i32, i32, i32, i32, Option<Rc<Fn(f64) -> [i32; 4]>>),
    Uniform4f(f64, f64, f64, f64, Option<Rc<Fn(f64) -> [f64; 4]>>),
}

#[derive(Clone)]
pub struct Uniform {
    name: String,
    data: UniformInner,
}

impl Uniform {
    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_data(&self) -> UniformInner {
        self.data.clone()
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
        UniformInner::Uniform1i(item, None)
    }
}

impl From<Rc<Fn(f64)->i32>> for UniformInner {
    fn from(func: Rc<Fn(f64)->i32>) -> Self {
        UniformInner::Uniform1i(0, Some(func))
    }
}

impl From<f64> for UniformInner {
    fn from(item: f64) -> Self {
        UniformInner::Uniform1f(item, None)
    }
}

impl From<Rc<Fn(f64)->f64>> for UniformInner {
    fn from(func: Rc<Fn(f64)->f64>) -> Self {
        UniformInner::Uniform1f(0.0, Some(func))
    }
}

impl From<[i32; 2]> for UniformInner {
    fn from(item: [i32; 2]) -> Self {
        UniformInner::Uniform2i(item[0], item[1], None)
    }
}

impl From<Rc<Fn(f64)->[i32; 2]>> for UniformInner {
    fn from(func: Rc<Fn(f64)->[i32; 2]>) -> Self {
        UniformInner::Uniform2i(0, 0, Some(func))
    }
}

impl From<[i32; 3]> for UniformInner {
    fn from(item: [i32; 3]) -> Self {
        UniformInner::Uniform3i(item[0], item[1], item[2], None)
    }
}

impl From<Rc<Fn(f64)->[i32; 3]>> for UniformInner {
    fn from(func: Rc<Fn(f64)->[i32; 3]>) -> Self {
        UniformInner::Uniform3i(0, 0, 0, Some(func))
    }
}

impl From<[f64; 2]> for UniformInner {
    fn from(item: [f64; 2]) -> Self {
        UniformInner::Uniform2f(item[0], item[1], None)
    }
}

impl From<Rc<Fn(f64)->[f64; 2]>> for UniformInner {
    fn from(func: Rc<Fn(f64)->[f64; 2]>) -> Self {
        UniformInner::Uniform2f(0.0, 0.0, Some(func))
    }
}

impl From<[f64; 3]> for UniformInner {
    fn from(item: [f64; 3]) -> Self {
        UniformInner::Uniform3f(item[0], item[1], item[2], None)
    }
}

impl From<Rc<Fn(f64)->[f64; 3]>> for UniformInner {
    fn from(func: Rc<Fn(f64)->[f64; 3]>) -> Self {
        UniformInner::Uniform3f(0.0, 0.0, 0.0, Some(func))
    }
}

impl From<[f64; 4]> for UniformInner {
    fn from(item: [f64; 4]) -> Self {
        UniformInner::Uniform4f(item[0], item[1], item[2], item[3], None)
    }
}

impl From<Rc<Fn(f64)->[f64; 4]>> for UniformInner {
    fn from(func: Rc<Fn(f64)->[f64; 4]>) -> Self {
        UniformInner::Uniform4f(0.0, 0.0, 0.0, 0.0, Some(func))
    }
}
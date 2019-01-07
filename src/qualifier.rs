/*!
The Qualifer class is a single data-type used by an Attribute or Uniform
 */

#[derive(Debug)]
pub enum Qualifier {
    Int(i32),
    Float(f32),
    Vec2(i32, i32),
    Vec3(i32, i32, i32),
    Vec2f(f32, f32),
    Vec3f(f32, f32, f32)
}

impl From<i32> for Qualifier {
    fn from(item: i32) -> Self {
        Qualifier::Int(item)
    }
}

impl From<f32> for Qualifier {
    fn from(item: f32) -> Self {
        Qualifier::Float(item)
    }
}

impl From<(i32, i32)> for Qualifier {
    fn from(item: (i32, i32)) -> Self {
        Qualifier::Vec2(item.0, item.1)
    }
}

impl From<(i32, i32, i32)> for Qualifier {
    fn from(item: (i32, i32, i32)) -> Self {
        Qualifier::Vec3(item.0, item.1, item.2)
    }
}

impl From<(f32, f32)> for Qualifier {
    fn from(item: (f32, f32)) -> Self {
        Qualifier::Vec2f(item.0, item.1)
    }
}

impl From<(f32, f32, f32)> for Qualifier {
    fn from(item: (f32, f32, f32)) -> Self {
        Qualifier::Vec3f(item.0, item.1, item.2)
    }
}

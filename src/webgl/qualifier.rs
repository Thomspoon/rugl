/*!
The Qualifer class is a single data-type used by an Attribute or Uniform
 */

#[derive(Debug, PartialEq)]
pub enum Qualifier {
    Int(i32),
    Float(f32),
    Vec2(i32, i32),
    Vec3(i32, i32, i32),
    Vec2f(f32, f32),
    Vec3f(f32, f32, f32),
    Vec4f(f32, f32, f32, f32),
}

impl Qualifier {
    pub fn to_vec(&self) -> Vec<f32> {
        match self {
            Qualifier::Int(inner) => vec![*inner as f32],
            Qualifier::Float(inner) => vec![*inner],
            Qualifier::Vec2(inner1, inner2) => vec![*inner1 as f32, *inner2 as f32],
            Qualifier::Vec3(inner1, inner2, inner3) => {
                vec![*inner1 as f32, *inner2 as f32, *inner3 as f32]
            }
            Qualifier::Vec2f(inner1, inner2) => vec![*inner1, *inner2],
            Qualifier::Vec3f(inner1, inner2, inner3) => vec![*inner1, *inner2, *inner3],
            Qualifier::Vec4f(inner1, inner2, inner3, inner4) => {
                vec![*inner1, *inner2, *inner3, *inner4]
            }
        }
    }
}

impl From<[i32; 1]> for Qualifier {
    fn from(item: [i32; 1]) -> Self {
        Qualifier::Int(item[0])
    }
}

impl From<i32> for Qualifier {
    fn from(item: i32) -> Self {
        Qualifier::Int(item)
    }
}

impl From<[f32; 1]> for Qualifier {
    fn from(item: [f32; 1]) -> Self {
        Qualifier::Float(item[0])
    }
}

impl From<f32> for Qualifier {
    fn from(item: f32) -> Self {
        Qualifier::Float(item)
    }
}

impl From<[i32; 2]> for Qualifier {
    fn from(item: [i32; 2]) -> Self {
        Qualifier::Vec2(item[0], item[1])
    }
}

impl From<[i32; 3]> for Qualifier {
    fn from(item: [i32; 3]) -> Self {
        Qualifier::Vec3(item[0], item[1], item[2])
    }
}

impl From<[f32; 2]> for Qualifier {
    fn from(item: [f32; 2]) -> Self {
        Qualifier::Vec2f(item[0], item[1])
    }
}

impl From<[f32; 3]> for Qualifier {
    fn from(item: [f32; 3]) -> Self {
        Qualifier::Vec3f(item[0], item[1], item[2])
    }
}

impl From<[f32; 4]> for Qualifier {
    fn from(item: [f32; 4]) -> Self {
        Qualifier::Vec4f(item[0], item[1], item[2], item[3])
    }
}

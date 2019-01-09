/*!
The Uniform class holds a vector of the inner types and offers many coersions of other types.
 */

use crate::qualifier::Qualifier;

#[derive(Debug, PartialEq)]
pub struct Uniform {
    name: String,
    data: Vec<Qualifier>
}

__impl_from_repeat!(i32, Uniform, UniformInner::Float);
__impl_from_repeat!(f32, Uniform, UniformInner::Float);
__impl_from_repeat!((i32, i32), Uniform, UniformInner::Vec2);
__impl_from_repeat!((i32, i32, i32), Uniform, UniformInner::Vec3);
__impl_from_repeat!((f32, f32), Uniform, UniformInner::Vec2f);
__impl_from_repeat!((f32, f32, f32), Uniform, UniformInner::Vec3f);

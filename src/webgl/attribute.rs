/*!
The Attribute class holds a vector of the inner types and offers many coersions of other types.
 */

use super::qualifier::Qualifier;

#[derive(Debug, PartialEq)]
pub struct Attribute {
    name: String,
    data: Vec<Qualifier>,
}

impl Attribute {
    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_qualifiers(&self) -> &Vec<Qualifier> {
        &self.data
    }
}

// Array impls
__impl_from_repeat_vec!(i32, Attribute, Qualifier::Float);
__impl_from_repeat_vec!(f32, Attribute, Qualifier::Float);
__impl_from_repeat_vec!((i32, i32), Attribute, Qualifier::Vec2);
__impl_from_repeat_vec!((i32, i32, i32), Attribute, Qualifier::Vec3);
__impl_from_repeat_vec!((f32, f32), Attribute, Qualifier::Vec2f);
__impl_from_repeat_vec!((f32, f32, f32), Attribute, Qualifier::Vec3f);
__impl_from_repeat_vec!((f32, f32, f32, f32), Attribute, Qualifier::Vec4f);

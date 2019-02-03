/*!
The Attribute class holds a vector of the inner types and offers many coersions of other types.
 */

use crate::qualifier::Qualifier;

#[derive(Debug, PartialEq)]
pub struct Attribute {
    name: String,
    data: Vec<Qualifier>
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
__impl_from_repeat!(i32, Attribute, AttributeInner::Float);
__impl_from_repeat!(f32, Attribute, AttributeInner::Float);
__impl_from_repeat!((i32, i32), Attribute, AttributeInner::Vec2);
__impl_from_repeat!((i32, i32, i32), Attribute, AttributeInner::Vec3);
__impl_from_repeat!((f32, f32), Attribute, AttributeInner::Vec2f);
__impl_from_repeat!((f32, f32, f32), Attribute, AttributeInner::Vec3f);

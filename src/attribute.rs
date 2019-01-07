/*!
The Attribute class holds a vector of the inner types and offers many coersions of other types.
 */

use crate::qualifier::Qualifier;

#[derive(Debug)]
pub struct Attribute {
    name: String,
    data: Vec<Qualifier>
}

// Array impls
__impl_from_repeat!((i32, i32), Attribute, AttributeInner::Vec2);
__impl_from_repeat!((i32, i32, i32), Attribute, AttributeInner::Vec3);

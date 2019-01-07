/*!
The Attribute class holds a vector of the inner types and offers many coersions of other types.
 */

/// Internal types supported via attributes
#[derive(Debug)]
enum AttributeInner {
    Vec2((i32, i32)),
    Vec3((i32, i32, i32))
}

#[derive(Debug)]
pub struct Attribute {
    inner: Vec<AttributeInner>
}

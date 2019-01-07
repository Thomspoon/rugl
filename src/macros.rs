/*!
Macros used for various utilities
!*/

#[doc(hidden)]
#[macro_export]
macro_rules! determine_bracket_replace {
    // Replace brackets with parenthesis and return as an array
    ($([$($tokens:tt)*]),*) => {
        [ $( ($($tokens)*) ),* ]
    };

    // Not actually replacing brackets, just passing to an array
    ($($tokens:tt)*) => {
        [ $($tokens)* ]
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! parse_ident {
    // Parse each attribute to generate ident and data array
    (@attribute
        $($id:ident: [$($tokens:tt)*]),+ $(,)*
    ) => {
        vec![$( Attribute::from((stringify!($id).to_owned(), determine_bracket_replace!($($tokens)*)) )),*]
    };
    // Parse each uniform to generate ident and data array
    (@uniform
        $($id:ident: [$($tokens:tt)*]),+ $(,)*
    ) => {
        vec![$( Uniform::from((stringify!($id).to_owned(), determine_bracket_replace!($($tokens)*)) )),*]
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! rugl_type {
    (vertex: $($tokens:tt)+) => {
        Shader::from($($tokens)*)
    };
    (fragment: $($tokens:tt)+) => {
        Shader::from($($tokens)*)
    };
    (attributes: $($tokens:tt)+) => {
        parse_ident!(@attribute $($tokens)*)
    };
    (uniforms: $($tokens:tt)+) => {
        parse_ident!(@uniform $($tokens)*)
    }
}

/// Simple macro giving named-arguments to the rugl type
#[macro_export]
macro_rules! rugl {
    (
        $( $i:ident: { $($tokens:tt)* } ),*
    ) => {
        Rugl {
            $($i: rugl_type!($i: $($tokens)*)),*
        };
    }
}


#[doc(hidden)]
macro_rules! __impl_from_for_type {
    ($type:ty, $num:expr, $impl_type:ty, $impl_subtype:tt::$impl_subtype_variant:tt) => {
        impl From<(String, [$type; $num])> for $impl_type {
            fn from(items: (String, [$type; $num])) -> Self {
                let mut v: Vec<Qualifier> = Vec::new();
                for item in &items.1 {
                    let variant = Qualifier::from(*item);
                    v.push(variant)
                }

                Self {
                    name: items.0,
                    data: v
                }
            }
        }
    };
}

#[doc(hidden)]
macro_rules! __impl_from_repeat {
    ($type:ty, $impl_type:ty, $impl_subtype:tt::$impl_subtype_variant:tt) => {
        __impl_from_for_type!($type, 1, $impl_type, $impl_subtype::$impl_subtype_variant);
        __impl_from_for_type!($type, 2, $impl_type, $impl_subtype::$impl_subtype_variant);
        __impl_from_for_type!($type, 3, $impl_type, $impl_subtype::$impl_subtype_variant);
        __impl_from_for_type!($type, 4, $impl_type, $impl_subtype::$impl_subtype_variant);
        __impl_from_for_type!($type, 5, $impl_type, $impl_subtype::$impl_subtype_variant);
        __impl_from_for_type!($type, 6, $impl_type, $impl_subtype::$impl_subtype_variant);
        __impl_from_for_type!($type, 7, $impl_type, $impl_subtype::$impl_subtype_variant);
        __impl_from_for_type!($type, 8, $impl_type, $impl_subtype::$impl_subtype_variant);
        __impl_from_for_type!($type, 9, $impl_type, $impl_subtype::$impl_subtype_variant);
        __impl_from_for_type!($type, 10, $impl_type, $impl_subtype::$impl_subtype_variant);
        __impl_from_for_type!($type, 11, $impl_type, $impl_subtype::$impl_subtype_variant);
        __impl_from_for_type!($type, 12, $impl_type, $impl_subtype::$impl_subtype_variant);
        __impl_from_for_type!($type, 13, $impl_type, $impl_subtype::$impl_subtype_variant);
        __impl_from_for_type!($type, 14, $impl_type, $impl_subtype::$impl_subtype_variant);
        __impl_from_for_type!($type, 15, $impl_type, $impl_subtype::$impl_subtype_variant);
        __impl_from_for_type!($type, 16, $impl_type, $impl_subtype::$impl_subtype_variant);
    }
}
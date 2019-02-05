/*!
Macros used for various utilities, this is the ugly file.
!*/

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
#[macro_export]
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

// TODO: Remove once const generics gets implemented
#[doc(hidden)]
macro_rules! __impl_from_for_type_vec {
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
                    data: v,
                }
            }
        }
    };
}

#[doc(hidden)]
macro_rules! __impl_from_repeat_vec {
    ($type:ty, $impl_type:ty, $impl_subtype:tt::$impl_subtype_variant:tt) => {
        __impl_from_for_type_vec!($type, 1, $impl_type, $impl_subtype::$impl_subtype_variant);
        __impl_from_for_type_vec!($type, 2, $impl_type, $impl_subtype::$impl_subtype_variant);
        __impl_from_for_type_vec!($type, 3, $impl_type, $impl_subtype::$impl_subtype_variant);
        __impl_from_for_type_vec!($type, 4, $impl_type, $impl_subtype::$impl_subtype_variant);
        __impl_from_for_type_vec!($type, 5, $impl_type, $impl_subtype::$impl_subtype_variant);
        __impl_from_for_type_vec!($type, 6, $impl_type, $impl_subtype::$impl_subtype_variant);
        __impl_from_for_type_vec!($type, 7, $impl_type, $impl_subtype::$impl_subtype_variant);
        __impl_from_for_type_vec!($type, 8, $impl_type, $impl_subtype::$impl_subtype_variant);
        __impl_from_for_type_vec!($type, 9, $impl_type, $impl_subtype::$impl_subtype_variant);
        __impl_from_for_type_vec!($type, 10, $impl_type, $impl_subtype::$impl_subtype_variant);
        __impl_from_for_type_vec!($type, 11, $impl_type, $impl_subtype::$impl_subtype_variant);
        __impl_from_for_type_vec!($type, 12, $impl_type, $impl_subtype::$impl_subtype_variant);
        __impl_from_for_type_vec!($type, 13, $impl_type, $impl_subtype::$impl_subtype_variant);
        __impl_from_for_type_vec!($type, 14, $impl_type, $impl_subtype::$impl_subtype_variant);
        __impl_from_for_type_vec!($type, 15, $impl_type, $impl_subtype::$impl_subtype_variant);
        __impl_from_for_type_vec!($type, 16, $impl_type, $impl_subtype::$impl_subtype_variant);
    };
}

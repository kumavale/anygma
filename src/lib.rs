//! This crate makes it easy to define arrays containing different types.
//!
//! ## Examples
//!
//! ```
//! use anygma::ary_anyref;
//!
//! let a = ary_anyref![0, 'a', "str"];
//! assert_eq!(a[0].downcast_ref::<i32>(), Some(&0));
//! assert_eq!(a[1].downcast_ref::<char>(), Some(&'a'));
//! assert_eq!(a[2].downcast_ref::<&str>(), Some(&"str"));
//! ```
//!
//! ```
//! use anygma::ary_tref;
//!
//! let a = ary_tref![&dyn std::fmt::Debug; 0, 'a', "str"];
//! println!("{:?}", a);
//! ```
//!
//! You can also create your own new macros using [ary_tref!]
//!
//! ```
//! # use anygma::ary_tref;
//! macro_rules! ary_debug {
//!     ( $( $value:expr ),+ $(,)? ) => {
//!         ary_tref![&dyn std::fmt::Debug; $($value),+]
//!     };
//! }
//!
//! let a = ary_debug![0, 'a', "str"];
//! println!("{:?}", a);
//! ```

/// This macro is a convenient way to create arrays of trait objects that you specify.
///
/// You can define an array of type `[&dyn T]`.
#[macro_export]
macro_rules! ary_tref {
    ( $ty:ty ) => { [] as [$ty; 0] };
    ( $ty:ty; $value:expr; $n:expr ) => {
        [&$value as $ty; $n]
    };
    ( $ty:ty; $( $value:expr ),+ $(,)? ) => {
        [ $(&$value as $ty),+ ]
    };
}

/// This macro is a convenient way to create arrays of trait objects that you specify.
///
/// You can define an array of type `[Box<dyn T>]`.
#[macro_export]
macro_rules! ary_tbox {
    ( $ty:ty ) => { [] as [Box<$ty>; 0] };
    ( $ty:ty; $value:expr; $n:expr ) => {
        [Box::new($value) as Box<$ty>; $n]
    };
    ( $ty:ty; $( $value:expr ),+ $(,)? ) => {
        [ $(Box::new($value) as Box<$ty>),+ ]
    };
}

/// This macro is a convenient way to create an array of Any trait objects.
///
/// You can define an array of type `[&dyn std::any::Any]`.
#[macro_export]
macro_rules! ary_anyref {
    () => { [] as [&dyn std::any::Any; 0] };
    ( $value:expr; $n:expr ) => {
        [&$value as &dyn std::any::Any; $n]
    };
    ( $( $value:expr ),+ $(,)? ) => {
        [ $(&$value as &dyn std::any::Any),+ ]
    };
}

/// This macro is a convenient way to create an array of Any trait objects.
///
/// You can define an array of type `[Box<dyn std::any::Any>]`.
#[macro_export]
macro_rules! ary_anybox {
    () => { [] as [Box<dyn std::any::Any>; 0] };
    ( $( $value:expr ),+ $(,)? ) => {
        [ $(Box::new($value) as Box<dyn std::any::Any>),+ ]
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! ary_debug {
        ( $( $value:expr ),+ $(,)? ) => {
            ary_tref![&dyn std::fmt::Debug; $($value),+]
        };
    }

    #[derive(Debug, PartialEq)]
    enum Animal {
        Cat,
    }

    #[test]
    fn test_ary_anyref() {
        let a = ary_anyref![];
        assert!(a.is_empty());

        let a = ary_anyref![0; 42];
        assert_eq!(a.len(), 42);
        assert_eq!(a[0].downcast_ref::<i32>(), Some(&0));

        let a = ary_anyref![0];
        assert_eq!(a[0].downcast_ref::<i32>(), Some(&0));

        let a = ary_anyref![0, 1, 2,];
        assert_eq!(a[0].downcast_ref::<i32>(), Some(&0));
        assert_eq!(a[1].downcast_ref::<i32>(), Some(&1));
        assert_eq!(a[2].downcast_ref::<i32>(), Some(&2));

        let a = ary_anyref![0, 'a', "str", Animal::Cat];
        assert_eq!(a[0].downcast_ref::<i32>(), Some(&0));
        assert_eq!(a[1].downcast_ref::<char>(), Some(&'a'));
        assert_eq!(a[2].downcast_ref::<&str>(), Some(&"str"));
        assert_eq!(a[3].downcast_ref::<Animal>(), Some(&Animal::Cat));
    }

    #[test]
    fn test_ary_anybox() {
        let a = ary_anybox![];
        assert!(a.is_empty());

        let a = ary_anybox![0];
        assert_eq!(a[0].downcast_ref::<i32>(), Some(&0));

        let a = ary_anybox![0, 1, 2,];
        assert_eq!(a[0].downcast_ref::<i32>(), Some(&0));
        assert_eq!(a[1].downcast_ref::<i32>(), Some(&1));
        assert_eq!(a[2].downcast_ref::<i32>(), Some(&2));

        let a = ary_anybox![0, 'a', "str", Animal::Cat];
        assert_eq!(a[0].downcast_ref::<i32>(), Some(&0));
        assert_eq!(a[1].downcast_ref::<char>(), Some(&'a'));
        assert_eq!(a[2].downcast_ref::<&str>(), Some(&"str"));
        assert_eq!(a[3].downcast_ref::<Animal>(), Some(&Animal::Cat));
    }

    #[test]
    fn test_ary_tref() {
        let a = ary_tref![&dyn std::fmt::Debug; 0, 'a', "str", Animal::Cat];
        println!("{:?}", a);

        let a = ary_debug![0, 'a', "str", Animal::Cat];
        println!("{:?}", a);
    }

    #[test]
    fn test_ary_tbox() {
        let a = ary_tbox![dyn std::fmt::Debug; 0, 'a', "str", Animal::Cat];
        println!("{:?}", a);
    }

    #[test]
    fn test_nested() {
        let a = 0;
        let b = ary_anybox![a];
        let c = ary_anybox![b];
        assert_eq!(
            c[0].downcast_ref::<[Box<dyn std::any::Any>; 1]>().unwrap()[0].downcast_ref::<i32>(),
            Some(&0)
        );
    }

    #[test]
    fn test_vec_ref() {
        let mut a = ary_anyref![0, 'a', "str", Animal::Cat].to_vec();
        a.push(&3.14);
        assert_eq!(a[4].downcast_ref::<f64>(), Some(&3.14));
    }
}

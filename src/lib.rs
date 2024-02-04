#[macro_export]
macro_rules! ary_t {
    ( $ty:ty ) => { [] as [$ty; 0] };
    ( $ty:ty; $value:expr; $n:expr ) => {
        [&$value as $ty; $n]
    };
    ( $ty:ty; $( $value:expr ),+ $(,)? ) => {
        [ $(&$value as $ty),+ ]
    };
}

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
            ary_t![&dyn std::fmt::Debug; $($value),+]
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
    fn test_ary_t() {
        let a = ary_t![&dyn std::fmt::Debug; 0, 'a', "str", Animal::Cat];
        println!("{:?}", a);

        let a = ary_debug![0, 'a', "str", Animal::Cat];
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

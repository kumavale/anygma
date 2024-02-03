#[macro_export]
macro_rules! ary_ref {
    () => { (&[]) as &[&dyn std::any::Any] };
    ( $value:expr; $n:expr ) => {
        &[&$value as &dyn std::any::Any; $n] as &[&dyn std::any::Any; $n]
    };
    ( $( $value:expr ),+ $(,)? ) => {
        &[ $(&$value as &dyn std::any::Any),+ ] as &[&dyn std::any::Any]
    };
}

#[macro_export]
macro_rules! ary_box {
    () => { (&[]) as &[Box<dyn std::any::Any>] };
    ( $( $value:expr ),+ $(,)? ) => {
        &[ $(Box::new($value) as Box<dyn std::any::Any>),+ ] as &[Box<dyn std::any::Any>]
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq)]
    enum Animal {
        Cat,
    }

    #[test]
    fn test_ary_ref() {
        let a = ary_ref![];
        assert!(a.is_empty());

        let a = ary_ref![0; 42];
        assert_eq!(a.len(), 42);
        assert_eq!(a[0].downcast_ref::<i32>(), Some(&0));

        let a = ary_ref![0];
        assert_eq!(a[0].downcast_ref::<i32>(), Some(&0));

        let a = ary_ref![0, 1, 2,];
        assert_eq!(a[0].downcast_ref::<i32>(), Some(&0));
        assert_eq!(a[1].downcast_ref::<i32>(), Some(&1));
        assert_eq!(a[2].downcast_ref::<i32>(), Some(&2));

        let a = ary_ref![0, 'a', "str", Animal::Cat];
        assert_eq!(a[0].downcast_ref::<i32>(), Some(&0));
        assert_eq!(a[1].downcast_ref::<char>(), Some(&'a'));
        assert_eq!(a[2].downcast_ref::<&str>(), Some(&"str"));
        assert_eq!(a[3].downcast_ref::<Animal>(), Some(&Animal::Cat));
    }

    #[test]
    fn test_ary_box() {
        let a = ary_box![];
        assert!(a.is_empty());

        let a = ary_box![0];
        assert_eq!(a[0].downcast_ref::<i32>(), Some(&0));

        let a = ary_box![0, 1, 2,];
        assert_eq!(a[0].downcast_ref::<i32>(), Some(&0));
        assert_eq!(a[1].downcast_ref::<i32>(), Some(&1));
        assert_eq!(a[2].downcast_ref::<i32>(), Some(&2));

        let a = ary_box![0, 'a', "str", Animal::Cat];
        assert_eq!(a[0].downcast_ref::<i32>(), Some(&0));
        assert_eq!(a[1].downcast_ref::<char>(), Some(&'a'));
        assert_eq!(a[2].downcast_ref::<&str>(), Some(&"str"));
        assert_eq!(a[3].downcast_ref::<Animal>(), Some(&Animal::Cat));
    }
}

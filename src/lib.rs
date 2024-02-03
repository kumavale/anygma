#[macro_export]
macro_rules! ary_ref {
    () => { (&[]) as &[&dyn std::any::Any] };
    ( $( $value:expr ),+ $(,)? ) => {
        &[ $(&$value as &dyn std::any::Any),+ ] as &[&dyn std::any::Any]
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ary_ref() {
        let a = ary_ref![];
        assert!(a.is_empty());
        let a = ary_ref![0];
        assert_eq!(a[0].downcast_ref::<i32>(), Some(&0));
    }
}

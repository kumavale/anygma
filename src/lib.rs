#[macro_export]
macro_rules! ary_ref {
    ( $( $value:expr ),+ $(,)? ) => {
        &[ $(&$value as &dyn std::any::Any),+ ] as &[&dyn std::any::Any]
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ary_ref() {
        let a = ary_ref![0];
        assert_eq!(a[0].downcast_ref::<i32>(), Some(&0));
    }
}

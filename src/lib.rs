#[macro_export]
macro_rules! assert_err {
    ($error:expr , $expected_message:expr) => ({
        assert_eq!(&($error).unwrap_err(), &($expected_message));
    })
}

#[macro_export]
macro_rules! assert_none {
    ($option:expr) => ({
        assert!($option.is_none(), "assertion failed: `{:?}` is not None", $option);
    })
}

#[macro_export]
macro_rules! assert_greater_than {
    ($left:expr, $right:expr) => ({
        assert!($left > $right, "assertion failed: `{:?}` is not greater than `{:?}`", $left, $right);
    })
}


#[cfg(test)]
mod tests {

    mod expr {
        #[test]
        fn should_work_with_identifiers() {
            let err: Result<u32, &str> = Err("Error message");
            assert_err!(err, "Error message");
        }

        #[test]
        fn should_work_with_functions() {
            fn f() -> Result<u8, String> {
                Err("It failed".to_owned())
            }

            assert_err!(f(), "It failed");
        }

        #[test]
        fn should_work_with_structs() {
            struct Foo;

            impl Foo {
                pub fn bar(&self) -> Result<u8, String> {
                    Err("Foo failed to bar".to_owned())
                }
            }

            let foo = Foo;
            assert_err!(foo.bar(), "Foo failed to bar");
        }
    }

    mod assert_err {
        #[test]
        #[should_panic(expected = "called `Result::unwrap_err()` on an `Ok` value: 1")]
        fn should_fail_if_err_is_not_returned() {
            let err: Result<u8, &str> = Ok(1);
            assert_err!(err, "error message");
        }
    }

    mod assert_none {
        #[test]
        fn should_not_fail_on_none() {
            let err: Option<u8> = None;
            assert_none!(err);
        }

        #[test]
        #[should_panic(expected = "assertion failed: `Some(1)` is not None")]
        fn should_fail_if_none_is_not_returned() {
            let err: Option<u8> = Some(1);
            assert_none!(err);
        }
    }

    mod assert_greater_than {
        #[test]
        fn should_pass() {
            assert_greater_than!(1, 0);
        }

        #[test]
        #[should_panic(expected = "assertion failed: `0` is not greater than `1`")]
        fn should_fail() {
            assert_greater_than!(0, 1);
        }

        #[test]
        #[should_panic(expected = "assertion failed: `0` is not greater than `0`")]
        fn should_fail_if_equal() {
            assert_greater_than!(0, 0);
        }
    }
}

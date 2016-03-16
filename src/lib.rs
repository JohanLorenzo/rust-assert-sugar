macro_rules! _expr {
    ($e:expr) => {
        $e
    }
}

macro_rules! _assert_operation {
    ($left:expr , $right:expr, $operator:tt, $($arg:tt)+) => ({
        match (&($left), &($right)) {
            (left_val, right_val) => {
                let operation_result = _expr!(left_val $operator right_val);
                if !operation_result {
                    panic!($($arg)+);
                }
            }
        }
    })
}

#[macro_export]
macro_rules! assert_equal {
    ($left:expr, $right:expr, $($arg:tt)+) => (
        _assert_operation!($left, $right, ==, $($arg)+);
    );
}


#[macro_export]
macro_rules! assert_err {
    ($error:expr , $expected_message:expr) => ({
        assert_equal!(&($error).unwrap_err(), &($expected_message), "assertion failed: error is not equal to {}", $expected_message);
    })
}

#[macro_export]
macro_rules! assert_none {
    ($option:expr) => ({
        assert!($option.is_none(), "assertion failed: `{:?}` is not None", $option);
    })
}

#[macro_export]
macro_rules! assert_not_eq {
    ($left:expr , $right:expr) => ({
        _assert_operation!($left, $right, !=, "assertion failed: `{:?}` is equal to `{:?}`", $left, $right)
    })

}


#[macro_export]
macro_rules! assert_greater_than {
    ($left:expr, $right:expr) => ({
        _assert_operation!($left, $right, >, "assertion failed: `{:?}` is not greater than `{:?}`", $left, $right);
    })
}

#[macro_export]
macro_rules! assert_greater_or_eq {
    ($left:expr, $right:expr) => ({
        _assert_operation!($left, $right, >=, "assertion failed: `{:?}` is not greater or equal to `{:?}`", $left, $right);
    })
}

#[macro_export]
macro_rules! assert_less_than {
    ($left:expr, $right:expr) => ({
        _assert_operation!($left, $right, <, "assertion failed: `{:?}` is not less than `{:?}`", $left, $right);
    })
}

#[macro_export]
macro_rules! assert_less_or_eq {
    ($left:expr, $right:expr) => ({
        _assert_operation!($left, $right, <=, "assertion failed: `{:?}` is not less or equal to `{:?}`", $left, $right);
    })
}

#[macro_export]
macro_rules! assert_length {
    ($array:expr , $expected_length:expr) => ({
        let length = &($array).len();
        assert_equal!(length, &($expected_length), "assertion failed: `{:?}` has a length of {}. {} was was expected.",
            $array, length, $expected_length);
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

    mod assert_not_eq {
        #[test]
        fn should_pass() {
            assert_not_eq!(1, 0);
        }

        #[test]
        #[should_panic(expected = "assertion failed: `0` is equal to `0`")]
        fn should_fail() {
            assert_not_eq!(0, 0);
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

    mod assert_greater_or_eq {
        #[test]
        fn should_pass() {
            assert_greater_or_eq!(1, 0);
        }

        #[test]
        #[should_panic(expected = "assertion failed: `0` is not greater or equal to `1`")]
        fn should_fail() {
            assert_greater_or_eq!(0, 1);
        }

        #[test]
        fn should_pass_if_equal() {
            assert_greater_or_eq!(0, 0);
        }
    }

    mod assert_less_than {
        #[test]
        fn should_pass() {
            assert_less_than!(0, 1);
        }

        #[test]
        #[should_panic(expected = "assertion failed: `1` is not less than `0`")]
        fn should_fail() {
            assert_less_than!(1, 0);
        }

        #[test]
        #[should_panic(expected = "assertion failed: `0` is not less than `0`")]
        fn should_fail_if_equal() {
            assert_less_than!(0, 0);
        }
    }

    mod assert_less_or_eq {
        #[test]
        fn should_pass() {
            assert_less_or_eq!(0, 1);
        }

        #[test]
        #[should_panic(expected = "assertion failed: `1` is not less or equal to `0`")]
        fn should_fail() {
            assert_less_or_eq!(1, 0);
        }

        #[test]
        fn should_pass_if_equal() {
            assert_less_or_eq!(0, 0);
        }
    }

    mod assert_length {
        #[test]
        fn should_pass() {
            let vector = vec!["a","b"];
            assert_length!(vector, 2);
        }

        #[test]
        #[should_panic(expected = "assertion failed: `[\"a\", \"b\"]` has a length of 2. 1 was was expected.")]
        fn should_fail() {
            let vector = vec!["a","b"];
            assert_length!(vector, 1);
        }
    }
}

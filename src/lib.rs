#![cfg_attr(test, feature(plugin, const_fn))]
#![cfg_attr(test, plugin(stainless))] // Test runner


#[macro_export]
macro_rules! assert_err {
    ($left:expr , $right:expr) => ({
        assert_eq!(&($left).unwrap_err(), &($right));
    })
}


#[cfg(test)]
describe! asserts {

    describe! assert_err {
        it "should work with identifiers" {
            let err: Result<u32, &str> = Err("Error message");
            assert_err!(err, "Error message");
        }

        it "should work with functions" {
            fn f() -> Result<u8, String> {
                Err("It failed".to_owned())
            }

            assert_err!(f(), "It failed");
        }

        it "should work with structs" {
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
}

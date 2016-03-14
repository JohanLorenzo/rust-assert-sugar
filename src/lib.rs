#![cfg_attr(test, feature(plugin, const_fn))]
#![cfg_attr(test, plugin(stainless))] // Test runner


#[macro_export]
macro_rules! assert_err {
    ($left:ident , $right:expr) => ({
        assert_eq!(&($left).unwrap_err(), &($right));
    })
}


#[cfg(test)]
describe! asserts {
    it "should work with Err" {
        let err: Result<u32, &str> = Err("Error message");
        assert_err!(err, "Error message");
    }
}

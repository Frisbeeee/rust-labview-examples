extern crate libc;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn increment_integer() {
        let input_integer: u32 = 1003;
        let incremented = rust_increment(input_integer);
        assert_eq!(incremented, 1004);
    }
}

// Uses the Rust Omnibus integer approach:
// http://jakegoulding.com/rust-ffi-omnibus/integers/

#[no_mangle]
pub extern "C" fn rust_increment(value: u32) -> u32 {
    value + 1
}
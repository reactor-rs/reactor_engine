#![macro_use]

/// Macro to get c strings from literals without runtime overhead
/// Literal must not contain any interior nul bytes!
#[macro_export]
macro_rules! c_str {
    ($literal:expr) => {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(concat!($literal, "\0").as_bytes())
    }
}

#[cfg(test)]
mod tests {
    use std::ffi::CString;

    #[test]
    fn literal_to_c_str() {
        assert_eq!(CString::new("A").unwrap().as_c_str(), unsafe { c_str!('A') });
        assert_eq!(CString::new("42").unwrap().as_c_str(), unsafe { c_str!(42) });
        assert_eq!(CString::new("0.").unwrap().as_c_str(), unsafe { c_str!(0.) });
        assert_eq!(CString::new("true").unwrap().as_c_str(), unsafe { c_str!(true) });
        assert_eq!(CString::new("test").unwrap().as_c_str(), unsafe { c_str!("test") });
        assert_eq!(CString::new("42000").unwrap().as_c_str(), unsafe { c_str!(42_000u16) });
        assert_eq!(CString::new("147").unwrap().as_c_str(), unsafe { c_str!(0b_1001_0011_i32) });
    }
}
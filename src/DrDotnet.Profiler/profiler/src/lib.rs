mod profilers;
mod report;

#[no_mangle]
pub extern "C" fn string_from_rust() -> *const i8 {

    let s = std::ffi::CString::new("Hello World").unwrap();
    let p = s.as_ptr();
    std::mem::forget(s);

    println!("done!");

    return p
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_from_rust() {
        string_from_rust();
        assert!(true);
    }
}
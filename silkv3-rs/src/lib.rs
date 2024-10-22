
pub fn add(left: i32, right: i32) -> i32 {
    left + right
    // my_c_function(left, right)
}

pub fn get_silk_version() -> String {
    unsafe {
        let result = SKP_Silk_SDK_get_version();
        let c_str = std::ffi::CStr::from_ptr(result);
        let str_slice = c_str.to_str().unwrap();
        str_slice.to_string()
    }
}

pub fn silk_decoder(in_file: &str, out_file: &str) -> i32 {
    let in_file = std::ffi::CString::new(in_file).unwrap();
    let out_file = std::ffi::CString::new(out_file).unwrap();
    unsafe {
        silk_v3_decoder(in_file.as_ptr(), out_file.as_ptr())
    }
}


extern "C" {
    fn SKP_Silk_SDK_get_version() -> *const i8;
    // int silk_v3_decoder(char* in_file, char* out_file)
    fn silk_v3_decoder(in_file: *const i8, out_file: *const i8) -> i32;
}
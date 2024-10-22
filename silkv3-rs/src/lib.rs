
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

extern "C" {
    // pub fn my_c_function(a: i32, b: i32) -> i32;
    /**************************/
    /* Get the version number */
    /**************************/
    /* Return a pointer to string specifying the version */
    // const char *SKP_Silk_SDK_get_version()
    // {
    // static const char version[] = "1.0.9.6";
    // return version;
    // }
    fn SKP_Silk_SDK_get_version() -> *const i8;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

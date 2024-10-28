use silkv3_rs::bindings::SKP_Silk_SDK_get_version;

pub fn get_version() -> Result<String, std::str::Utf8Error>{
    unsafe {
        let result = SKP_Silk_SDK_get_version();
        let c_str = std::ffi::CStr::from_ptr(result);
        let str_slice = c_str.to_str()?;
        Ok(str_slice.to_string())
    }
}
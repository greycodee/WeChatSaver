
// use silkv3_rs::my_c_function;
use silkv3_rs::get_silk_version;
fn main() {
    println!("Hello, world!");
    let silk_version = get_silk_version();
    println!("Silk version: {}", silk_version);
    // unsafe {
    //     let result = SKP_Silk_SDK_get_version();
    //     let c_str = std::ffi::CStr::from_ptr(result);
    //     let str_slice = c_str.to_str().unwrap();
    //     println!("Result: {}", str_slice);
    // }
}

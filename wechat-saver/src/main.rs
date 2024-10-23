mod android_backup;
mod ffmpeg;

// use silkv3_rs::my_c_function;
use silkv3_rs::get_silk_version;
use silkv3_rs::silk_decoder;

fn main() {
    // file_util::test();
    println!("Hello, world!");
    let silk_version = get_silk_version();
    println!("Silk version: {}", silk_version);
    let res = silk_decoder("/tmp/msg_152059061922b0890a24269102.amr", "/tmp/msg_152059061922b0890a24269102.pcm");
    println!("Result: {}", res);
    // unsafe {
    //     let result = SKP_Silk_SDK_get_version();
    //     let c_str = std::ffi::CStr::from_ptr(result);
    //     let str_slice = c_str.to_str().unwrap();
    //     println!("Result: {}", str_slice);
    // }
}

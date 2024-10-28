mod android_backup;
mod ffmpeg;
mod wechat;
mod sqlite;

use silkv3_rs::silk_v3_decoder;



fn main() {
    // file_util::test();
    println!("Hello, world!");
    let res = silk_decoder("/tmp/msg_152059061922b0890a24269102.amr", "/tmp/msg_152059061922b0890a24269102.pcm");
    // println!("Result: {}", res);
    // unsafe {
    //     let result = SKP_Silk_SDK_get_version();
    //     let c_str = std::ffi::CStr::from_ptr(result);
    //     let str_slice = c_str.to_str().unwrap();
    //     println!("Result: {}", str_slice);
    // }
}

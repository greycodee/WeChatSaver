use wechat_saver_lib::voice_decode;

use wechat_saver_lib::wechat::utils;

fn main() {
    let res = utils::get_all_uin("/Users/zheng/Downloads/20241024_091952");
    println!("{:?}",res);
    // file_util::test();
    println!("Hello, world!");
    // let decode_version = voice_decode::get_version();
    // println!("decode_version: {}", decode_version.unwrap());
    //
    // // let res = silkv3_rs::silk_decoder("/tmp/msg_152059061922b0890a24269102.amr", "/tmp/msg_152059061922b0890a24269102.pcm");
    // // println!("Result: {}", res);
    //
    //
    // let res = voice_decode::silk_v3_decoder("/tmp/msg_152059061922b0890a24269102.amr", "/tmp/msg_152059061922b0890a24269102.pcm");
    // // println!("Result: {}", res);
    // match res {
    //     Ok(_) => {
    //         println!("Decoding success!");
    //     },
    //     Err(e) => {
    //         panic!("ERR: {}",e);
    //     }
    // }
    // unsafe {
    //     let result = SKP_Silk_SDK_get_version();
    //     let c_str = std::ffi::CStr::from_ptr(result);
    //     let str_slice = c_str.to_str().unwrap();
    //     println!("Result: {}", str_slice);
    // }
}

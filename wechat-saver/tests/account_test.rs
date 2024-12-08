mod common;

use wechat_saver_lib::wechat::get_all_account;


#[test]
fn test_dotenv(){
    common::setup();
    // get .env value
    let version = std::env::var("WECHAT_VERSION").unwrap();
    assert_eq!(version, "1.0.0");
}
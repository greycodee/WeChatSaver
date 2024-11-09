use quick_xml::events::Event;
use quick_xml::Reader;
mod account;
mod file_path;
mod database;
mod file_arch;
mod model;
mod utils;
pub mod voice_decode;
pub mod ffmpeg;

use account::AccountInfo;

pub fn get_all_account(base_path: &str) -> Vec<AccountInfo> {
    let uin_vec = get_all_uin(base_path);
    let mut account_vec = Vec::new();
    for uin in uin_vec {
        let account_info = AccountInfo::new(base_path, &uin);
        account_vec.push(account_info);
    }
    account_vec
}

pub fn get_all_uin(base_path: &str) -> Vec<String> {
    let mut uin_vec = Vec::new();
    let uin_file_path = format!("{}/apps/com.tencent.mm/sp/app_brand_global_sp.xml", base_path);
    let mut reader = Reader::from_file(uin_file_path).unwrap();
    reader.config_mut().trim_text(true);
    loop {
        match reader.read_event_into(&mut Vec::new()) {
            Ok(Event::Text(e)) => {
                match String::from_utf8(e.into_inner().into_owned()) {
                    Ok(uin) => {
                        uin_vec.push(uin);
                    },
                    Err(e) => {
                        panic!("Error: {:?}", e);
                    }
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (),
        }
    }
    uin_vec
}

#[cfg(test)]
mod test {

    const BASE_PATH: &str = "/Users/zheng/Downloads/20241024_091952";
    // use super::*;
    #[test]
    fn test_get_all_uin(){
        let res = crate::wechat::get_all_uin(BASE_PATH);
        println!("{:?}",res);
    }

    #[test]
    fn test_get_all_account(){
        let res = crate::wechat::get_all_account(BASE_PATH);
        // json serialization
        let json = serde_json::to_string(&res).unwrap();

        println!("{}",json);
    }
}

use std::path::Path;
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

pub fn get_all_account(base_path: &Path) -> std::io::Result<Vec<AccountInfo>> {
    let uin_vec = get_all_uin(base_path);
    let mut account_vec = Vec::new();
    for uin in uin_vec {
        let account_info = AccountInfo::new(base_path, &uin)?;
        account_vec.push(account_info);
    }
    Ok(account_vec)
}

pub fn get_all_uin(base_path: &Path) -> Vec<String> {
    let mut uin_vec = Vec::new();
    let uin_file_path = base_path.join("apps/com.tencent.mm/sp/app_brand_global_sp.xml");
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
    use std::path::Path;

    // use super::*;
    #[test]
    fn test_get_all_uin(){
        let base_path = Path::new("/sdcard/Android/data/com.tencent.mm");
        let res = crate::wechat::get_all_uin(base_path);
        println!("{:?}",res);
    }

    #[test]
    fn test_get_all_account(){
        let base_path = Path::new("/sdcard/Android/data/com.tencent.mm");

        let res = crate::wechat::get_all_account(base_path).unwrap();
        // json serialization
        let json = serde_json::to_string(&res).unwrap();

        println!("{}",json);
    }
}

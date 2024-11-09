use md5::{Md5, Digest};
use quick_xml::events::Event;
use quick_xml::Reader;

pub fn gen_db_private_key(uin: &str) -> String {
    let mut private_key = String::from("1234567890ABCDEF");
    private_key.push_str(uin);
    let md5_private_key = md5_encode(&private_key);
    md5_private_key[0..7].to_string()
}

pub fn md5_encode(input: &str) -> String {
    let mut hasher = Md5::new();
    hasher.update(input);
    let result = hasher.finalize();
    let result = hex::encode(result);
    result
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


mod test{
    use crate::wechat::utils::gen_db_private_key;

    const BASE_PATH: &str = "/Users/zheng/Downloads/20241024_091952";
    // use super::*;
    #[test]
    fn test_gen_db_private_key(){
        let uin = "1727242265";
        let key = gen_db_private_key(uin);
        println!("key: {}",key);
    }


    #[test]
    fn test_get_all_uin(){
        let uins = crate::wechat::utils::get_all_uin(BASE_PATH);
        println!("{:?}",uins);
    }

    #[test]
    fn test_md5_encode(){
        let input = "123123";
        let res = crate::wechat::utils::md5_encode(input);
        println!("{:?}",res);
    }

}
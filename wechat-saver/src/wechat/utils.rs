use md5::{Md5, Digest};
use quick_xml::events::Event;
use quick_xml::Reader;

pub fn gen_db_private_key(uin: &str) -> String {
    let mut private_key = String::from("1234567890ABCDEF");
    private_key.push_str(uin);
    let mut hasher = Md5::new();
    hasher.update(private_key);
    let result = hasher.finalize();

    let result = hex::encode(result);
    result
}

pub fn get_all_system_dir_path(base_path: &str) -> Vec<String> {
    let pre_path = format!("{}/apps/com.tencent.mm/r/MicroMsg", base_path);
    let mut system_file_paths = Vec::new();
    let uin_vec = get_all_uin(base_path);
    for uin in uin_vec {
        let system_file_name = get_system_file_name(&uin);
        let system_file_path = format!("{}/{}", pre_path, system_file_name);
        system_file_paths.push(system_file_path);
    }
    system_file_paths
}

pub fn get_system_file_name(uin: &str) -> String {
    let mut private_key = String::from("mm");
    private_key.push_str(uin);
    let mut hasher = Md5::new();
    hasher.update(private_key);
    let result = hasher.finalize();
    result.iter().map(|x| format!("{:02x}", x)).collect::<String>()
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
    fn test_get_system_file_name(){
        let uin = "1727242265";
        let key = crate::wechat::utils::get_system_file_name(uin);
        println!("key: {}",key);
    }

    #[test]
    fn test_get_all_uin(){
        let uins = crate::wechat::utils::get_all_uin(BASE_PATH);
        println!("{:?}",uins);
    }

    #[test]
    fn test_get_all_system_dir_path(){
        let system_file_paths = crate::wechat::utils::get_all_system_dir_path(BASE_PATH);
        println!("{:?}",system_file_paths);
    }

}
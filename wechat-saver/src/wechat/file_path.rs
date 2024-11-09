use std::io::Error;
use md5::{Digest, Md5};

pub fn get_system_file_name(uin: &str) -> String {
    let mut private_key = String::from("mm");
    private_key.push_str(uin);
    let mut hasher = Md5::new();
    hasher.update(private_key);
    let result = hasher.finalize();
    result
        .iter()
        .map(|x| format!("{:02x}", x))
        .collect::<String>()
}

pub fn get_sd_card_dir_name(base_path: &str, uin: &str) -> Result<String,Error> {
    let account_dir_name = get_system_file_name(uin);
    let account_mapping_file_path = format!(
        "{}/apps/com.tencent.mm/r/MicroMsg/{}/account.mapping",
        base_path, account_dir_name
    );
    let account_mapping_file = std::fs::read_to_string(account_mapping_file_path)?;
    Ok(account_mapping_file)
}



mod test {

    const BASE_PATH: &str = "/Users/zheng/Downloads/20241024_091952";
    // use super::*;

    #[test]
    fn test_get_system_file_name() {
        let uin = "1727242265";
        let key = crate::wechat::file_path::get_system_file_name(uin);
        println!("key: {}", key);
    }

    #[test]
    fn test_get_sd_card_dir_name() {
        let uin = "1727242265";
        let key = crate::wechat::file_path::get_sd_card_dir_name(BASE_PATH, uin);
        println!("key: {:?}", key);
    }
}

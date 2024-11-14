use std::io::Error;
use std::path::Path;
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

pub fn get_sd_card_dir_name(base_path: &Path, uin: &str) -> std::io::Result<String> {
    let account_dir_name = get_system_file_name(uin);
    let account_mapping_file_path = base_path.join("apps/com.tencent.mm/r/MicroMsg")
        .join(account_dir_name).join("account.mapping");

    let account_mapping_file = std::fs::read_to_string(account_mapping_file_path)?;

    Ok(account_mapping_file)
}



mod test {
    use std::path::Path;

    // const BASE_PATH: &Path = Path::new("/sdcard/Android/data/com.tencent.mm");
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
        let base_path = Path::new("/sdcard/Android/data/com.tencent.mm");
        let key = crate::wechat::file_path::get_sd_card_dir_name(base_path, uin);
        println!("key: {:?}", key);
    }
}

use super::utils::md5_encode;
use std::path::{Path, PathBuf};

pub fn get_system_file_name(uin: &str) -> String {
    let mut private_key = String::from("mm");
    private_key.push_str(uin);
    md5_encode(&private_key)
}

pub fn get_sd_card_dir_name(base_path: &Path, uin: &str) -> std::io::Result<String> {
    let account_dir_name = get_system_file_name(uin);
    let account_mapping_file_path = base_path
        .join("apps/com.tencent.mm/r/MicroMsg")
        .join(account_dir_name)
        .join("account.mapping");

    let account_mapping_file = std::fs::read_to_string(account_mapping_file_path)?;

    Ok(account_mapping_file)
}

pub fn get_avatar_path(wx_id: &str) -> PathBuf {
    let md5_wx_id = md5_encode(wx_id);
    let avatar_file_name = format!("user_{}.png", md5_wx_id);
    let avatar_pre_dir_path = format!("{}/{}", &md5_wx_id[0..2], &md5_wx_id[2..4]);
    let avatar_path = PathBuf::from(avatar_pre_dir_path).join(avatar_file_name);
    avatar_path
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

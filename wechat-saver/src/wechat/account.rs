use super::database::open_wechat_db;
use super::file_path::get_sd_card_dir_name;
use super::file_path::get_system_file_name;
use super::utils::gen_db_private_key;
use super::utils::md5_encode;
use rusqlite::params;
use serde::{Deserialize, Serialize};
use std::io::Error;
use std::path::{Path, PathBuf};

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct WXUserInfo {
    pub wx_id: String,
    pub wx_account_no: String,
    pub account_name: String,
    pub account_phone: String,
    pub account_avatar_path: Option<PathBuf>,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountInfo {
    pub wx_user_info: WXUserInfo,
    pub account_uin: String,
    pub video_path: PathBuf,
    pub voice_path: PathBuf,
    pub image_path: PathBuf,
    pub avatar_path: PathBuf,
    pub download_path: PathBuf,
    pub openapi_path: PathBuf,
    pub en_micro_msg_db_path: PathBuf,
    pub wx_file_index_db_path: PathBuf,
    pub db_private_key: String,
}

impl AccountInfo {
    pub fn new(base_path: &Path, uin: &str) -> std::io::Result<Self> {
        let account_dir_name = get_system_file_name(uin);
        let account_file_path = base_path
            .join("apps/com.tencent.mm/r/MicroMsg")
            .join(account_dir_name);

        let image_path = account_file_path.join("image2");
        let avatar_path = account_file_path.join("avatar");
        let en_micro_msg_db_path = account_file_path.join("EnMicroMsg.db");
        let wx_file_index_db_path = account_file_path.join("WxFileIndex.db");
        let account_sd_card_dir_name = get_sd_card_dir_name(base_path, uin)?;
        let account_sd_card_dir_path = base_path
            .join("Android/data/com.tencent.mm/MicroMsg")
            .join(account_sd_card_dir_name);

        let video_path = account_sd_card_dir_path.join("video");
        let voice_path = account_sd_card_dir_path.join("voice2");
        let download_path = account_sd_card_dir_path.join("Download");

        let db_private_key = gen_db_private_key(uin);
        match Self::get_wx_user_info(&en_micro_msg_db_path, &db_private_key) {
            Ok(wx_user_info) => Ok(AccountInfo {
                wx_user_info,
                account_uin: uin.to_string(),
                video_path,
                voice_path,
                image_path,
                avatar_path,
                download_path,
                openapi_path: Default::default(),
                en_micro_msg_db_path,
                wx_file_index_db_path,
                db_private_key,
            }),
            Err(err) => Err(Error::new(std::io::ErrorKind::Other, err)),
        }
    }

    fn get_wx_user_info(db_path: &Path, db_key: &str) -> rusqlite::Result<WXUserInfo> {
        let conn = open_wechat_db(db_path, db_key)?;
        let mut stmt = conn.prepare("SELECT id,value FROM userinfo where id in (2,4,6,42)")?;
        let persons = stmt.query_map(params![], |row| Ok((row.get(0)?, row.get(1)?)))?;

        let mut account_info = WXUserInfo {
            account_name: "".to_string(),
            account_phone: "".to_string(),
            account_avatar_path: None,
            wx_id: "".to_string(),
            wx_account_no: "".to_string(),
        };
        for p in persons {
            let (id, value): (i32, String) = p?;
            match id {
                2 => account_info.wx_id = value,
                4 => account_info.account_name = value,
                6 => account_info.account_phone = value,
                42 => account_info.wx_account_no = value,
                _ => {}
            }
        }

        account_info.account_avatar_path = Some(Self::get_avatar_path(&account_info.wx_id));

        Ok(account_info)
    }

    fn get_avatar_path(wx_id: &str) -> PathBuf {
        let md5_wx_id = md5_encode(wx_id);
        let avatar_file_name = format!("user_{}.png", md5_wx_id);
        let avatar_pre_dir_path = format!("{}/{}", &md5_wx_id[0..2], &md5_wx_id[2..4]);
        let avatar_path = PathBuf::from(avatar_pre_dir_path).join(avatar_file_name);
        avatar_path
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // const BASE_PATH: &Path = Path::

    #[test]
    fn test_account_info() {
        let uin = "1727242265";
        let base_path = Path::new("/Users/zheng/Downloads/20241024_091952");
        let account_info = AccountInfo::new(&base_path, uin);
        println!("{:?}", account_info);
    }

    #[test]
    fn test_get_avatar_path() {
        let wx_id = "wxid_123456";
        let avatar_path = AccountInfo::get_avatar_path(wx_id);
        println!("avatar_path: {:?}", avatar_path);
    }
}

use rusqlite::params;
use serde::{Deserialize, Serialize};
use super::file_path::get_system_file_name;
use super::file_path::get_sd_card_dir_name;
use super::utils::gen_db_private_key;
use super::utils::md5_encode;
use super::database::open_wechat_db;


#[allow(dead_code)]
#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct WXUserInfo {
    pub wx_id:String,
    pub wx_account_no:String,
    pub account_name:String,
    pub account_phone:String,
    pub account_avatar_path:String,
}

#[allow(dead_code)]
#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct AccountInfo{
    pub wx_user_info:Option<WXUserInfo>,
    pub account_uin:String,
    pub video_path:String,
    pub voice_path:String,
    pub image_path:String,
    pub avatar_path:String,
    pub download_path:String,
    pub en_micro_msg_db_path:String,
    pub wx_file_index_db_path:String,
    pub db_private_key:String,
}

impl AccountInfo {

    pub fn new(base_path: &str, uin: &str) -> Self {
        let account_dir_name = get_system_file_name(uin);
        let account_file_path = format!("{}/apps/com.tencent.mm/r/MicroMsg/{}", base_path,account_dir_name);

        let image_path = format!("{}/{}", account_file_path, "image2");
        let avatar_path = format!("{}/{}", account_file_path, "avatar");
        let en_micro_msg_db_path = format!("{}/{}", account_file_path, "EnMicroMsg.db");
        let wx_file_index_db_path = format!("{}/{}", account_file_path, "WxFileIndex.db");

        let account_sd_card_dir_name = get_sd_card_dir_name(base_path, uin).unwrap();
        let account_sd_card_dir_path = format!("{}/Android/data/com.tencent.mm/MicroMsg/{}", base_path, account_sd_card_dir_name);

        let video_path = format!("{}/{}", account_sd_card_dir_path, "video");
        let voice_path = format!("{}/{}", account_sd_card_dir_path, "voice2");
        let download_path = format!("{}/{}", account_sd_card_dir_path, "Download");

        let db_private_key = gen_db_private_key(uin);
        let wx_user_info = match Self::get_wx_user_info(&en_micro_msg_db_path, &db_private_key) {
            Ok(Some(info)) => info,
            _ => WXUserInfo {
                account_name: "".to_string(),
                account_phone: "".to_string(),
                account_avatar_path: "".to_string(),
                wx_id: "".to_string(),
                wx_account_no: "".to_string(),
            }
        };

        AccountInfo {
            wx_user_info: Some(wx_user_info),
            account_uin: uin.to_string(),
            video_path,
            voice_path,
            image_path,
            avatar_path,
            download_path,
            en_micro_msg_db_path,
            wx_file_index_db_path,
            db_private_key,
        }
    }

    fn get_wx_user_info(db_path:&str, db_key: &str) -> rusqlite::Result<Option<WXUserInfo>> {
        let conn = open_wechat_db(db_path, db_key)?;
        let mut stmt = conn.prepare("SELECT id,value FROM userinfo where id in (2,4,6,42)")?;
        let persons = stmt.query_map(params![], |row| {
            Ok((row.get(0)?, row.get(1)?))
        })?;

        let mut account_info = WXUserInfo {
            account_name: "".to_string(),
            account_phone: "".to_string(),
            account_avatar_path: "".to_string(),
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

        account_info.account_avatar_path = Self::get_avatar_path(&account_info.wx_id);

        Ok(Some(account_info))
    }

    fn get_avatar_path(wx_id: &str) -> String {
        let md5_wx_id = md5_encode(wx_id);
        let avatar_file_name = format!("user_{}.png", md5_wx_id);
        let avatar_pre_dir_path = format!("{}/{}",&md5_wx_id[0..2],&md5_wx_id[2..4]);
        let avatar_path = format!("{}/{}", avatar_pre_dir_path, avatar_file_name);
        avatar_path
    }

}

#[cfg(test)]
mod test {
    use super::*;

    const BASE_PATH: &str = "/Users/zheng/Downloads/20241024_091952";

    #[test]
    fn test_account_info() {
        let uin = "1727242265";
        let account_info = AccountInfo::new(BASE_PATH, uin);
        println!("{:?}", account_info);
    }

    #[test]
    fn test_get_avatar_path() {
        let wx_id = "wxid_123456";
        let avatar_path = AccountInfo::get_avatar_path(wx_id);
        println!("avatar_path: {}", avatar_path);
    }
}
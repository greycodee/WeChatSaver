use crate::wechat::account::WXUserInfo;
use crate::wechat::file_path;
use crate::wechat::model::{Message, RContact, UserInfo, WxFileIndex3};
use rusqlite::{params, Connection, OptionalExtension, Result};
use std::path::Path;

#[derive(Debug)]
pub struct WechatDB {
    en_micro_msg_conn: Connection,
    wx_file_index_conn: Connection,
}

impl WechatDB {
    pub fn new(
        en_micro_msg_db_path: &Path,
        wx_file_index_db_path: &Path,
        db_private_key: &str,
    ) -> Result<Self> {
        let en_micro_msg_conn = open_wechat_db(en_micro_msg_db_path, db_private_key)?;
        let wx_file_index_conn = open_wechat_db(wx_file_index_db_path, db_private_key)?;
        Ok(WechatDB {
            en_micro_msg_conn,
            wx_file_index_conn,
        })
    }

    pub fn select_message_with_limit(&self, start: u32, end: u32) -> Result<Vec<Message>> {
        let mut stmt = self
            .en_micro_msg_conn
            .prepare("SELECT * FROM message limit ?,?")?;
        let messages = stmt
            .query_map((start, end), |row| {
                Ok(Message {
                    msg_id: row.get(0)?,
                    msg_svr_id: row.get(1)?,
                    msg_type: row.get(2)?,
                    status: row.get(3)?,
                    is_send: row.get(4)?,
                    is_show_timer: row.get(5)?,
                    create_time: row.get(6)?,
                    talker: row.get(7)?,
                    content: row.get(8)?,
                    img_path: row.get(9)?,
                    reserved: row.get(10)?,
                    lvbuffer: row.get(11)?,
                    trans_content: row.get(12)?,
                    trans_brand_wording: row.get(13)?,
                    talker_id: row.get(14)?,
                    biz_client_msg_id: row.get(15)?,
                    biz_chat_id: row.get(16)?,
                    biz_chat_user_id: row.get(17)?,
                    msg_seq: row.get(18)?,
                    flag: row.get(19)?,
                    solitaire_fold_info: row.get(20)?,
                    history_id: row.get(21)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(messages)
    }

    pub fn select_r_contact_with_limit(&self,start:u32,end:u32) -> Result<Vec<RContact>>{
        let mut stmt = self.en_micro_msg_conn.prepare("SELECT * FROM rcontact LIMIT ?,?")?;
        let contacts = stmt.query_map((start, end), |row| {
            Ok(RContact {
                username: row.get(0)?,
                alias: row.get(1)?,
                con_remark: row.get(2)?,
                domain_list: row.get(3)?,
                nickname: row.get(4)?,
                py_initial: row.get(5)?,
                quan_pin: row.get(6)?,
                show_head: row.get(7)?,
                r#type: row.get(8)?,
                ui_type: row.get(9)?,
                weibo_flag: row.get(10)?,
                weibo_nickname: row.get(11)?,
                con_remark_py_full: row.get(12)?,
                con_remark_py_short: row.get(13)?,
                lvbuff: row.get(14)?,
                verify_flag: row.get(15)?,
                encrypt_username: row.get(16)?,
                chatroom_flag: row.get(17)?,
                delete_flag: row.get(18)?,
                contact_label_ids: row.get(19)?,
                desc_wording_id: row.get(20)?,
                open_im_appid: row.get(21)?,
                source_ext_info: row.get(22)?,
                ticket: row.get(23)?,
                username_flag: row.get(24)?,
                contact_extra: row.get(25)?,
                create_time: row.get(26)?,
            })
        })?.collect::<std::result::Result<Vec<_>, _>>()?;
        Ok(contacts)
    }

    pub fn select_user_info_with_limit(&self,start:u32,end:u32) -> Result<Vec<UserInfo>> {
        let mut stmt = self.en_micro_msg_conn.prepare("SELECT * FROM userinfo LIMIT ?,?")?;
        let persons = stmt.query_map((start, end), |row| {
            Ok(UserInfo {
                id: row.get(0)?,
                w_type: row.get(1)?,
                w_value: row.get(2)?,
            })
        })?.collect::<std::result::Result<Vec<_>, _>>()?;
        Ok(persons)
    }

    pub fn select_wx_file_index_by_msg_id(&self, msg_id: i64) -> Result<Option<WxFileIndex3>> {
        let mut stmt = self.wx_file_index_conn.prepare("SELECT * FROM WxFileIndex3 WHERE msgId = ?")?;
        let wx_file_index = stmt.query_row(params![msg_id], |row| {
            Ok(WxFileIndex3 {
                msg_id: row.get(0)?,
                username: row.get(1)?,
                msg_type: row.get(2)?,
                msg_sub_type: row.get(3)?,
                path: row.get(4)?,
                size: row.get(5)?,
                msg_time: row.get(6)?,
                hash: row.get(7).ok(),
                disk_space: row.get(8)?,
                link_uuid: row.get(9).ok(),
            })
        }).optional()?;
        Ok(wx_file_index)
    }


    pub fn get_wx_user_info(&self) -> Result<WXUserInfo> {
        let mut stmt = self
            .en_micro_msg_conn
            .prepare("SELECT id,value FROM userinfo where id in (2,4,6,42)")?;
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
        account_info.account_avatar_path = Some(file_path::get_avatar_path(&account_info.wx_id));
        Ok(account_info)
    }
}

fn open_wechat_db(db_path: &Path, pri_key: &str) -> Result<Connection> {
    let conn = Connection::open(db_path)?;
    conn.execute_batch(&format!("PRAGMA key = '{}';", pri_key))?;
    conn.execute_batch(&format!("PRAGMA cipher_use_hmac = {};", "off"))?;
    conn.execute_batch(&format!("PRAGMA kdf_iter = {};", 4000))?;
    conn.execute_batch(&format!("PRAGMA cipher_page_size = {};", 1024))?;
    conn.execute_batch(&format!("PRAGMA cipher_hmac_algorithm = {};", "HMAC_SHA1"))?;
    conn.execute_batch(&format!(
        "PRAGMA cipher_kdf_algorithm = {};",
        "PBKDF2_HMAC_SHA1"
    ))?;
    Ok(conn)
}

fn save_wechat_db_to_plan(db_path: &str, pri_key: &str) -> Result<String> {
    let conn = Connection::open(db_path)?;
    conn.execute_batch(&format!("PRAGMA key = '{}';", pri_key))?;
    conn.execute_batch(&format!("PRAGMA cipher_use_hmac = {};", "off"))?;
    conn.execute_batch(&format!("PRAGMA kdf_iter = {};", 4000))?;
    conn.execute_batch(&format!("PRAGMA cipher_page_size = {};", 1024))?;
    conn.execute_batch(&format!("PRAGMA cipher_hmac_algorithm = {};", "HMAC_SHA1"))?;
    conn.execute_batch(&format!(
        "PRAGMA cipher_kdf_algorithm = {};",
        "PBKDF2_HMAC_SHA1"
    ))?;
    conn.execute_batch("ATTACH DATABASE '/tmp/plan3.db' AS plan_db KEY '';")?;
    conn.execute_batch("SELECT sqlcipher_export('plan_db');")?;
    conn.execute_batch("DETACH DATABASE plan_db;")?;

    Ok("".to_string())
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_save_wechat_db_to_plan(){
        let db_path = "/Volumes/hkdisk/wechat-backup/20241117/84105892-ff2b-4377-914b-2b2e24b24661/apps/com.tencent.mm/r/MicroMsg/79b23ef49a3016d8c52a787fc4ab59e4/newuba.db";
        let db_private_key = "626d0bc";
        let result = save_wechat_db_to_plan(db_path, db_private_key);
        println!("{:?}", result);
    }

    #[test]
    fn test_select_message_with_limit() {
        let db_path = Path::new("/tmp/com.tencent.mm/2aa8c917-cab9-446e-85df-b777695ddcc8/apps/com.tencent.mm/r/MicroMsg/79b23ef49a3016d8c52a787fc4ab59e4/EnMicroMsg.db");
        let wx_file_index_db_path = Path::new("/tmp/com.tencent.mm/2aa8c917-cab9-446e-85df-b777695ddcc8/apps/com.tencent.mm/r/MicroMsg/79b23ef49a3016d8c52a787fc4ab59e4/WxFileIndex.db");
        let db_private_key = "626d0bc";
        let wechat_db = WechatDB::new(db_path, wx_file_index_db_path, db_private_key)
            .expect("TODO: panic message");
        let messages = wechat_db
            .select_message_with_limit(1000, 500)
            .expect("TODO: panic message");
        for message in messages {
            println!("{:?}", message);
        }
    }

    #[test]
    fn test_select_wx_file_index_by_msg_id(){
        let db_path = Path::new("/tmp/com.tencent.mm/2aa8c917-cab9-446e-85df-b777695ddcc8/apps/com.tencent.mm/r/MicroMsg/79b23ef49a3016d8c52a787fc4ab59e4/EnMicroMsg.db");
        let wx_file_index_db_path = Path::new("/tmp/com.tencent.mm/2aa8c917-cab9-446e-85df-b777695ddcc8/apps/com.tencent.mm/r/MicroMsg/79b23ef49a3016d8c52a787fc4ab59e4/WxFileIndex.db");
        let db_private_key = "626d0bc";
        let wechat_db = WechatDB::new(db_path, wx_file_index_db_path, db_private_key)
            .expect("TODO: panic message");

        if let Ok(f) = wechat_db.select_wx_file_index_by_msg_id(0){
            match f {
                Some(ff) => {
                    println!("wx: {:?}",ff);
                },
                None => {
                    println!("None");
                }
            }
        }
    }
}

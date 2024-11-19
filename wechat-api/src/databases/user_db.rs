use std::path::Path;
use anyhow::Result;
use rusqlite::Connection;
use wechat_saver_lib::wechat::model::Message;

struct UserDb {
    conn: Connection,
}


impl UserDb {

    pub fn new(db_path: &Path) -> Result<Self>{
        let conn = Connection::open(db_path)?;
        Ok(Self {
            conn
        })
    }

    pub fn select_group_talker_with_limit(&self,  start: u32, end: u32) -> Result<Vec<String>> {
        let mut stmt = self
            .conn
            .prepare("SELECT talker FROM message GROUP BY talker limit ?,?")?;
        let talkers = stmt
            .query_map((start, end), |row| {
                Ok(row.get(0)?)
            })?
            .collect::<rusqlite::Result<Vec<_>, _>>()?;
        Ok(talkers)
    }

    pub fn select_message_with_limit(&self, start: u32, end: u32) -> Result<Vec<Message>> {
        let mut stmt = self
            .conn
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
            .collect::<rusqlite::Result<Vec<_>, _>>()?;
        Ok(messages)
    }

}


#[cfg(test)]
mod test {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_user_db() {
        let db_path = PathBuf::from("/Volumes/hkdisk/wechat-backup/20241117/wxid_jafjkmbud9l912/wechat.db");
        let user_db = UserDb::new(&db_path).unwrap();
        let messages = user_db.select_message_with_limit(0, 10).unwrap();
        for message in messages {
            println!("{:?}", message);
        }
    }

    #[test]
    fn test_select_group_talker_with_limit() {
        let db_path = PathBuf::from("/Volumes/hkdisk/wechat-backup/20241117/wxid_jafjkmbud9l912/wechat.db");
        let user_db = UserDb::new(&db_path).unwrap();
        let talkers = user_db.select_group_talker_with_limit(0, 10).unwrap();
        for talker in talkers {
            println!("{:?}", talker);
        }
    }
}
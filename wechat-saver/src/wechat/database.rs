use rusqlite::{params, Connection, Result};
use crate::wechat::model::QuickAccountInfo;
use super::model;

#[allow(dead_code)]
#[derive(Debug)]
struct Person {
    id: i32,
    content: String,
}

fn open_wechat_db(db_path: &str,pri_key:&str) -> Result<Connection> {
    let conn = Connection::open(db_path)?;
    conn.execute_batch(&format!("PRAGMA key = '{}';", pri_key))?;
    conn.execute_batch(&format!("PRAGMA cipher_use_hmac = {};", "off"))?;
    conn.execute_batch(&format!("PRAGMA kdf_iter = {};", 4000))?;
    conn.execute_batch(&format!("PRAGMA cipher_page_size = {};", 1024))?;
    conn.execute_batch(&format!("PRAGMA cipher_hmac_algorithm = {};", "HMAC_SHA1"))?;
    conn.execute_batch(&format!("PRAGMA cipher_kdf_algorithm = {};", "PBKDF2_HMAC_SHA1"))?;
    Ok(conn)
}

pub fn quick_get_account_info(db_path:&str, db_key: &str) -> Result<Option<QuickAccountInfo>> {
    let conn = open_wechat_db(db_path, db_key)?;
    let mut stmt = conn.prepare("SELECT id,value FROM userinfo where id in (4,6)")?;
    let persons = stmt.query_map(params![], |row| {
        Ok((row.get(0)?, row.get(1)?))
    })?;
    let mut account_info = QuickAccountInfo {
        account_name: "".to_string(),
        account_uin: "".to_string(),
        account_phone: "".to_string(),
        account_avatar_path: "".to_string(),
    };
    for p in persons {
        let (id, value): (i32, String) = p?;
        match id {
            4 => account_info.account_name = value,
            6 => account_info.account_phone = value,
            _ => {}
        }
    }
    Ok(Some(account_info))
}


fn save_wechat_db_to_plan(db_path: &str,pri_key:&str) -> Result<String> {
    let conn = Connection::open(db_path)?;
    conn.execute_batch(&format!("PRAGMA key = '{}';", pri_key))?;
    conn.execute_batch(&format!("PRAGMA cipher_use_hmac = {};", "off"))?;
    conn.execute_batch(&format!("PRAGMA kdf_iter = {};", 4000))?;
    conn.execute_batch(&format!("PRAGMA cipher_page_size = {};", 1024))?;
    conn.execute_batch(&format!("PRAGMA cipher_hmac_algorithm = {};", "HMAC_SHA1"))?;
    conn.execute_batch(&format!("PRAGMA cipher_kdf_algorithm = {};", "PBKDF2_HMAC_SHA1"))?;
    conn.execute_batch("ATTACH DATABASE '/tmp/plan3.db' AS plan_db KEY '';")?;
    conn.execute_batch("SELECT sqlcipher_export('plan_db');")?;
    conn.execute_batch("DETACH DATABASE plan_db;")?;

    Ok("".to_string())
}


mod test{
    use rusqlite::params;
    use super::*;
    #[test]
    fn test_open_wechat_db(){
        let db_path = "/tmp/EnMicroMsg.db";
        let pri_key = "626d0bc";
        let db_conn = open_wechat_db(db_path, pri_key).expect("TODO: panic message");
        let mut stmt = db_conn.prepare("SELECT * FROM message limit 10").unwrap();
        let persons = stmt.query_map(params![], |row| {
            Ok(model::Message {
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
                lvbuffer: row.get(11 )?,

                trans_content: None,
                trans_brand_wording: None,
                talker_id: None,
                biz_client_msg_id: None,
                biz_chat_id: None,
                biz_chat_user_id: None,
                msg_seq: None,
                flag: 0,
                solitaire_fold_info: None,
                history_id: None,
            })
        }).unwrap();

        for p in persons {
            println!("Found person {:?}", p);
        }
    }

    #[test]
    fn test_save_wechat_db_to_plan(){
        let db_path = "/Users/zheng/Downloads/20241024_091952/apps/com.tencent.mm/r/MicroMsg/2db66c115dd15b04d5b022bd1dba5f50/EnMicroMsg.db";
        let pri_key = "c344e93";
        let res = save_wechat_db_to_plan(db_path, pri_key);
        println!("{:?}",res);
    }

    #[test]
    fn test_quick_get_account_info(){
        let db_path = "/Users/zheng/Downloads/20241024_091952/apps/com.tencent.mm/r/MicroMsg/2db66c115dd15b04d5b022bd1dba5f50/EnMicroMsg.db";
        let pri_key = "c344e93";
        let res = quick_get_account_info(db_path, pri_key);
        println!("{:?}",res);
    }
}



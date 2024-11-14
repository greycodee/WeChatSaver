use std::path::Path;
use rusqlite::{params, Connection,Result};
use crate::wechat::account::WXUserInfo;
use super::model;
// use std::io::{Error, Result};


#[allow(dead_code)]
#[derive(Debug)]
struct Person {
    id: i32,
    content: String,
}

fn init_db(wx_user_info: &WXUserInfo,dest_path: &Path) -> Result<Connection> {
    let db_path = dest_path.join(&wx_user_info.wx_id);
    let db_path = db_path.join("db");
    if !db_path.exists() {
        std::fs::create_dir_all(&db_path).expect("create db dir failed");
    }
    let db_path = db_path.join("wechat.db");
    let conn = Connection::open(db_path)?;
    // create database
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS message (
        msg_id INTEGER PRIMARY KEY,
        msg_svr_id INTEGER,
        msg_type INTEGER,
        status INTEGER,
        is_send INTEGER,
        is_show_timer INTEGER,
        create_time INTEGER,
        talker TEXT,
        content TEXT,
        img_path TEXT,
        reserved TEXT,
        lvbuffer BLOB
    )",
    )?;
    Ok(conn)

}

pub fn open_wechat_db(db_path: &Path,pri_key:&str) -> Result<Connection> {
    let conn = Connection::open(db_path)?;
    conn.execute_batch(&format!("PRAGMA key = '{}';", pri_key))?;
    conn.execute_batch(&format!("PRAGMA cipher_use_hmac = {};", "off"))?;
    conn.execute_batch(&format!("PRAGMA kdf_iter = {};", 4000))?;
    conn.execute_batch(&format!("PRAGMA cipher_page_size = {};", 1024))?;
    conn.execute_batch(&format!("PRAGMA cipher_hmac_algorithm = {};", "HMAC_SHA1"))?;
    conn.execute_batch(&format!("PRAGMA cipher_kdf_algorithm = {};", "PBKDF2_HMAC_SHA1"))?;
    Ok(conn)
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
        let db_path = Path::new("/tmp/EnMicroMsg.db");
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
        let db_path = "/Users/zheng/Downloads/20241024_091952/apps/com.tencent.mm/r/MicroMsg/79b23ef49a3016d8c52a787fc4ab59e4/EnMicroMsg.db";
        let pri_key = "626d0bc";
        let res = save_wechat_db_to_plan(db_path, pri_key);
        println!("{:?}",res);
    }

    #[test]
    fn test_init_db(){
        let wx_user_info = WXUserInfo {
            wx_id: "wxid_1sdas111".to_string(),
            wx_account_no: "".to_string(),
            account_name: "".to_string(),
            account_phone: "".to_string(),
            account_avatar_path: None,
        };
        let dest_path = Path::new("/tmp/testdb");
        let res = init_db(&wx_user_info, dest_path);
        println!("{:?}",res);
    }

}



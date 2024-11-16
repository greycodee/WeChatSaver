#[allow(dead_code)]
#[derive(Debug)]
pub struct Message {
    pub msg_id: i64,
    pub msg_svr_id: Option<i64>,
    pub msg_type: Option<i64>,
    pub status: Option<i64>,
    pub is_send: Option<i64>,
    pub is_show_timer: Option<i64>,
    pub create_time: i64,
    pub talker: String,
    pub content: Option<String>,
    pub img_path: Option<String>,
    pub reserved: Option<String>,
    pub lvbuffer: Option<Vec<u8>>, // BLOB as Vec<u8> in Rust
    pub trans_content: Option<String>,
    pub trans_brand_wording: Option<String>,
    pub talker_id: Option<i64>,
    pub biz_client_msg_id: Option<String>,
    pub biz_chat_id: Option<i64>, // Default value can be set at struct instantiation
    pub biz_chat_user_id: Option<String>,
    pub msg_seq: Option<i64>,
    pub flag: Option<i64>,
    pub solitaire_fold_info: Option<Vec<u8>>, // BLOB as Vec<u8> in Rust
    pub history_id: Option<String>,
}

#[derive(Debug)]
pub struct UserInfo {
    pub id: i64,       // INTEGER, primary key
    pub w_type: i64,   // INT (type is a keyword, use w_type )
    pub w_value: String, // TEXT
}

#[derive(Debug)]
pub struct RContact {
    pub username: String,                // TEXT default ''
    pub alias: String,                   // TEXT default ''
    pub con_remark: String,              // TEXT default ''
    pub domain_list: String,             // TEXT default ''
    pub nickname: String,                // TEXT default ''
    pub py_initial: String,              // TEXT default ''
    pub quan_pin: String,                // TEXT default ''
    pub show_head: i64,                  // INTEGER default '0'
    pub r#type: i64,                     // INTEGER default '0' (type is a keyword, use r#type)
    pub ui_type: i64,                    // LONG default '0'
    pub weibo_flag: i64,                 // INTEGER default '0'
    pub weibo_nickname: String,          // TEXT default ''
    pub con_remark_py_full: String,      // TEXT default ''
    pub con_remark_py_short: String,     // TEXT default ''
    pub lvbuff: Option<Vec<u8>>,         // BLOB
    pub verify_flag: i64,                // INTEGER default '0'
    pub encrypt_username: String,        // TEXT default ''
    pub chatroom_flag: Option<i64>,      // INTEGER
    pub delete_flag: i64,                // INTEGER default '0'
    pub contact_label_ids: String,       // TEXT default ''
    pub desc_wording_id: String,         // TEXT default ''
    pub open_im_appid: Option<String>,   // TEXT
    pub source_ext_info: Option<String>, // TEXT
    pub ticket: String,                  // TEXT default ''
    pub username_flag: i64,              // LONG default '0'
    pub contact_extra: Option<Vec<u8>>,  // BLOB
    pub create_time: i64,                // LONG default '0'
}

#[derive(Debug,Clone)]
pub struct WxFileIndex3 {
    pub msg_id: i64,                // LONG
    pub username: String,           // TEXT
    pub msg_type: i64,              // INTEGER
    pub msg_sub_type: i64,          // INTEGER
    pub path: String,               // TEXT
    pub size: i64,                  // LONG
    pub msg_time: i64,              // LONG
    pub hash: Option<Vec<u8>>,      // BLOB
    pub disk_space: i64,            // LONG
    pub link_uuid: Option<Vec<u8>>, // BLOB
}

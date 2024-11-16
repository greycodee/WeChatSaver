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
struct UserInfo {
    id: i64,       // INTEGER, primary key
    r#type: i64,   // INT (type is a keyword, use r#type)
    value: String, // TEXT
}

#[derive(Debug)]
struct RContact {
    username: String,          // TEXT default ''
    alias: String,             // TEXT default ''
    con_remark: String,        // TEXT default ''
    domain_list: String,       // TEXT default ''
    nickname: String,          // TEXT default ''
    py_initial: String,        // TEXT default ''
    quan_pin: String,          // TEXT default ''
    show_head: i64,            // INTEGER default '0'
    r#type: i64,               // INTEGER default '0' (type is a keyword, use r#type)
    ui_type: i64,              // LONG default '0'
    weibo_flag: i64,           // INTEGER default '0'
    weibo_nickname: String,    // TEXT default ''
    con_remark_py_full: String,// TEXT default ''
    con_remark_py_short: String, // TEXT default ''
    lvbuff: Option<Vec<u8>>,   // BLOB
    verify_flag: i64,          // INTEGER default '0'
    encrypt_username: String,  // TEXT default ''
    chatroom_flag: Option<i64>,// INTEGER
    delete_flag: i64,          // INTEGER default '0'
    contact_label_ids: String, // TEXT default ''
    desc_wording_id: String,   // TEXT default ''
    open_im_appid: Option<String>, // TEXT
    source_ext_info: Option<String>, // TEXT
    ticket: String,            // TEXT default ''
    username_flag: i64,        // LONG default '0'
    contact_extra: Option<Vec<u8>>, // BLOB
    create_time: i64,          // LONG default '0'
}


#[derive(Debug)]
struct WxFileIndex3 {
    msg_id: i64,        // LONG
    username: String,   // TEXT
    msg_type: i64,      // INTEGER
    msg_sub_type: i64,  // INTEGER
    path: String,       // TEXT
    size: i64,          // LONG
    msg_time: i64,      // LONG
    hash: Option<Vec<u8>>, // BLOB
    disk_space: i64,    // LONG
    link_uuid: Option<Vec<u8>>, // BLOB
}
#[allow(dead_code)]
#[derive(Debug)]
pub struct Message {
    pub msg_id: i32,
    pub msg_svr_id: Option<i32>,
    pub msg_type: i32,
    pub status: i32,
    pub is_send: i32,
    pub is_show_timer: Option<i32>,
    pub create_time: i32,
    pub talker: Option<String>,
    pub content: Option<String>,
    pub img_path: Option<String>,
    pub reserved: Option<String>,
    pub lvbuffer: Option<Vec<u8>>, // BLOB as Vec<u8> in Rust
    pub trans_content: Option<String>,
    pub trans_brand_wording: Option<String>,
    pub talker_id: Option<i32>,
    pub biz_client_msg_id: Option<String>,
    pub biz_chat_id: Option<i32>, // Default value can be set at struct instantiation
    pub biz_chat_user_id: Option<String>,
    pub msg_seq: Option<i32>,
    pub flag: i32,
    pub solitaire_fold_info: Option<Vec<u8>>, // BLOB as Vec<u8> in Rust
    pub history_id: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct QuickAccountInfo {
    pub account_name:String,
    pub account_uin:String,
    pub account_phone:String,
    pub account_avatar_path:String,
}
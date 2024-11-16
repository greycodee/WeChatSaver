use std::path::Path;
use rusqlite::{params, Connection};
use rusqlite::Result;
use super::super::model::Message;

#[derive(Debug)]
pub struct WeChatSaverDB {
    conn: Connection
}

impl WeChatSaverDB {

    /**
        @param base_path: userspace path
        @param wx_id: 微信id
    */
    pub fn new(base_path: &Path) -> Result<Self> {
        let conn = init_save_db(base_path)?;
        Ok(WeChatSaverDB {
            conn
        })
    }

    pub fn save_message(&self,message: &Message) -> Result<usize> {
        self.conn.execute(
                "INSERT INTO message (
                msgSvrId, type, status, isSend, isShowTimer, createTime, talker, content, imgPath, reserved, lvbuffer, transContent, transBrandWording, talkerId, bizClientMsgId, bizChatId, bizChatUserId, msgSeq, flag, solitaireFoldInfo, historyId
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21)",
                rusqlite::params![
                message.msg_svr_id,
                message.msg_type,
                message.status,
                message.is_send,
                message.is_show_timer,
                message.create_time,
                message.talker,
                message.content,
                message.img_path,
                message.reserved,
                message.lvbuffer,
                message.trans_content,
                message.trans_brand_wording,
                message.talker_id,
                message.biz_client_msg_id,
                message.biz_chat_id,
                message.biz_chat_user_id,
                message.msg_seq,
                message.flag,
                message.solitaire_fold_info,
                message.history_id,
            ],
            )

    }

    /**
    判断消息是否可以备份
    @param msg_svr_id: Option<i64> 为None时表示msgSvrId为空
    @param talker: &str
    @param create_time: i64 消息创建时间
    @return: 返回是否可以备份 true: 可以 false: 不可以
    */
    pub fn addition_flag(&self,msg_svr_id:Option<i64>, talker: &str, create_time: i64) -> Result<bool>{

            match msg_svr_id {
                None => {
                    let mut stmt = self.conn.prepare("SELECT count(*) FROM message WHERE msgSvrId IS NULL AND talker = ? AND createTime = ?")?;
                    let count: i64 = stmt.query_row(params![talker, create_time], |row| row.get(0))?;
                    Ok(count == 0)
                }
                Some(id) => {
                    let mut stmt = self.conn.prepare("SELECT count(*) FROM message WHERE msgSvrId = ? AND talker = ? AND createTime = ?")?;
                    let count: i64 = stmt.query_row(params![id, talker, create_time], |row| row.get(0))?;
                    Ok(count == 0)
                }
            }
    }
}


fn init_save_db(dest_path: &Path) -> Result<Connection> {

    let db_path = dest_path.join("wechat.db");
    let conn = Connection::open(db_path)?;
    // create database
    conn.execute_batch(
        "
create table IF NOT EXISTS message
(
    msgId             INTEGER
        primary key autoincrement ,
    msgSvrId          INTEGER,
    type              INT,
    status            INT,
    isSend            INT,
    isShowTimer       INTEGER,
    createTime        INTEGER,
    talker            TEXT,
    content           TEXT,
    imgPath           TEXT,
    reserved          TEXT,
    lvbuffer          BLOB,
    transContent      TEXT,
    transBrandWording TEXT,
    talkerId          INTEGER,
    bizClientMsgId    TEXT,
    bizChatId         INTEGER default -1,
    bizChatUserId     TEXT,
    msgSeq            INTEGER,
    flag              INT,
    solitaireFoldInfo BLOB,
    historyId         TEXT
);

create index IF NOT EXISTS messageCreateTaklerTimeIndex
    on message (talker, createTime);

create index IF NOT EXISTS messageCreateTaklerTypeTimeIndex
    on message (talker, type, createTime);

create index IF NOT EXISTS messageCreateTimeIndex
    on message (createTime);

create index IF NOT EXISTS messageIdIndex
    on message (msgId);

create index IF NOT EXISTS messageSendCreateTimeIndex
    on message (status, isSend, createTime);

create index IF NOT EXISTS messageSvrIdIndex
    on message (msgSvrId);

create index IF NOT EXISTS messageTalkerCreateTimeIsSendIndex
    on message (talker, isSend, createTime);

create index IF NOT EXISTS messageTalkerIdTypeIndex
    on message (talkerId, type);

create index IF NOT EXISTS messageTalkerStatusIndex
    on message (talker, status);

create index IF NOT EXISTS messageTalkerSvrIdIndex
    on message (talker, msgSvrId);

create index IF NOT EXISTS messageTalkerTypeIndex
    on message (talker, type);

create index IF NOT EXISTS messagemessageTalkerFlagMsgSeqIndex
    on message (talker, flag, msgSeq);

create index IF NOT EXISTS messagemessageTalkerMsgSeqIndex
    on message (talker, msgSeq);

"
    )?;

    conn.execute_batch(
        "
create table IF NOT EXISTS userinfo
(
    id    INTEGER
        primary key,
    type  INT,
    value TEXT
);
"
    )?;

    conn.execute_batch(
        "
create table IF NOT EXISTS rcontact
(
    username         TEXT    default ''
        primary key,
    alias            TEXT    default '',
    conRemark        TEXT    default '',
    domainList       TEXT    default '',
    nickname         TEXT    default '',
    pyInitial        TEXT    default '',
    quanPin          TEXT    default '',
    showHead         INTEGER default '0',
    type             INTEGER default '0',
    uiType           LONG    default '0',
    weiboFlag        INTEGER default '0',
    weiboNickname    TEXT    default '',
    conRemarkPYFull  TEXT    default '',
    conRemarkPYShort TEXT    default '',
    lvbuff           BLOB,
    verifyFlag       INTEGER default '0',
    encryptUsername  TEXT    default '',
    chatroomFlag     INTEGER,
    deleteFlag       INTEGER default '0',
    contactLabelIds  TEXT    default '',
    descWordingId    TEXT    default '',
    openImAppid      TEXT,
    sourceExtInfo    TEXT,
    ticket           TEXT    default '',
    usernameFlag     LONG    default '0',
    contactExtra     BLOB,
    createTime       LONG    default '0'
);

create index IF NOT EXISTS contact_alias_index
    on rcontact (alias);

create unique index IF NOT EXISTS contact_username_unique_index
    on rcontact (username);

create index IF NOT EXISTS contact_usernameflag_index
    on rcontact (usernameFlag);

create index IF NOT EXISTS en_username_unique_index
    on rcontact (encryptUsername);

create index IF NOT EXISTS type_verifyFlag_index
    on rcontact (type, verifyFlag);
",
    )?;

    conn.execute_batch(
        "
create table IF NOT EXISTS WxFileIndex3
(
    msgId      LONG,
    username   TEXT,
    msgType    INTEGER,
    msgSubType INTEGER,
    path       TEXT,
    size       LONG,
    msgtime    LONG,
    hash       BLOB,
    diskSpace  LONG,
    linkUUID   BLOB
);

create index IF NOT EXISTS WxFileIndex_uuid
    on WxFileIndex3 (linkUUID);

create index IF NOT EXISTS msgid_username_index
    on WxFileIndex3 (msgId, username, msgSubType);

create index IF NOT EXISTS username_type_index
    on WxFileIndex3 (username, msgtime, msgSubType);

",
    )?;
    Ok(conn)
}



mod test {
    use super::*;

    // #[test]
    // fn test_addition_flag(){
    //     let en_micro_msg_db_path = Path::new("/tmp/com.tencent.mm/2aa8c917-cab9-446e-85df-b777695ddcc8/apps/com.tencent.mm/r/MicroMsg/79b23ef49a3016d8c52a787fc4ab59e4/EnMicroMsg.db");
    //     let wx_file_index_db_path = Path::new("/tmp/com.tencent.mm/2aa8c917-cab9-446e-85df-b777695ddcc8/apps/com.tencent.mm/r/MicroMsg/79b23ef49a3016d8c52a787fc4ab59e4/WxFileIndex.db");
    //     let db_private_key = "626d0bc";
    //     let mut wechat_db = WechatDB::new(en_micro_msg_db_path, wx_file_index_db_path, db_private_key).expect("TODO: panic message");
    //
    //     let flag = wechat_db.addition_flag(Some(77380342827986082), "gh_aaf6483adb11", 1669089338000).expect("TODO: panic message");
    //     println!("{:?}",flag);
    // }
}

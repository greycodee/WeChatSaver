// create table WxFileIndex3
// (
// msgId      LONG,
// username   TEXT,
// msgType    INTEGER,
// msgSubType INTEGER,
// path       TEXT,
// size       LONG,
// msgtime    LONG,
// hash       BLOB,
// diskSpace  LONG,
// linkUUID   BLOB
// );
//
// create index WxFileIndex_uuid
// on WxFileIndex3 (linkUUID);
//
// create index msgid_username_index
// on WxFileIndex3 (msgId, username, msgSubType);
//
// create index username_type_index
// on WxFileIndex3 (username, msgtime, msgSubType);

// 1040187441 qq音乐封面/qq音乐分享
// 1090519089 文件消息
// 436207665：微信红包
// 1048625 表情包
// 822083633 是引用消息

enum FileType {
    Image,
    Video,
    Voice,
    ShareCard,
    RedPacket,

    File,
    Other,
}

struct WxFileIndex {
    msg_id: i64,
    username: String,
    msg_type: i32,
    msg_sub_type: i32,
    path: String,
    size: i64,
    msg_time: i64,
    hash: Vec<u8>,
    disk_space: i64,
    link_uuid: Vec<u8>,
}

impl WxFileIndex {
    // TODO 转移到新db
    // 解析文件类型

    fn get_first_value_after_double_slash(input: &str) -> Option<&str> {
        if let Some(start) = input.find("//") {
            let rest = &input[start + 2..];
            if let Some(end) = rest.find('/') {
                return Some(&rest[..end]);
            }
        }
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_first_value_after_double_slash() {
        let input = "wcf://attachment/clash_for_android.apk";
        let res = WxFileIndex::get_first_value_after_double_slash(input);
        assert_eq!(res, Some("attachment"));
    }
}

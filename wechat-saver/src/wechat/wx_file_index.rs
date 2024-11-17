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


#[derive(Debug, PartialEq)]
pub enum FileDirName{
    Download,
    Attachment,
}



pub fn get_first_value_after_double_slash(input: &str) -> Option<&str> {
    if let Some(start) = input.find("//") {
        let rest = &input[start + 2..];
        if let Some(end) = rest.find('/') {
            return Some(&rest[..end]);
        }
    }
    None
}

pub fn get_after_double_slash(input: &str) -> Option<&str> {
    if let Some(start) = input.find("//") {
        let rest = &input[start + 2..];
        return Some(rest);
    }
    None
}

pub fn get_file_dir_name(input: &str) -> Option<FileDirName> {
    if let Some(start) = input.find("//") {
        let rest = &input[start + 2..];
        if rest.starts_with("Download") {
            return Some(FileDirName::Download);
        } else if rest.starts_with("attachment") {
            return Some(FileDirName::Attachment);
        }
    }
    None
}

pub fn get_file_name(input: &str) -> Option<&str> {
    if let Some(start) = input.rfind('/') {
        return Some(&input[start + 1..]);
    }
    None
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_first_value_after_double_slash() {
        let input = "wcf://attachment/clash_for_android.apk";
        let res = get_first_value_after_double_slash(input);
        assert_eq!(res, Some("attachment"));
    }

    #[test]
    fn test_get_first_value_after_double_slash_1() {
        let input = "wcf://Download/test.docx";
        let res = get_first_value_after_double_slash(input);
        assert_eq!(res, Some("Download"));
    }

    #[test]
    fn test_get_after_double_slash(){
        let input = "wcf://Download/test.docx";
        let res = get_after_double_slash(input);
        assert_eq!(res, Some("Download/test.docx"));
    }

    #[test]
    fn test_get_after_double_slash_2(){
        let input = "wcf://image2/d7/0c/th_d70cd0752c8e5042c86de60349dd6b2b";
        let res = get_after_double_slash(input);
        assert_eq!(res, Some("image2/d7/0c/th_d70cd0752c8e5042c86de60349dd6b2b"));
    }

    #[test]
    fn test_get_file_dir_name(){
        let input = "wcf://Download/test.docx";
        let res = get_file_dir_name(input);
        assert_eq!(res, Some(FileDirName::Download));
    }

    #[test]
    fn test_get_file_dir_name_2(){
        let input = "wcf://attachment/test.docx";
        let res = get_file_dir_name(input);
        assert_eq!(res, Some(FileDirName::Attachment));
    }

    #[test]
    fn test_get_file_dir_name_none(){
        let input = "wcf://image2/d7/0c/th_d70cd0752c8e5042c86de60349dd6b2b";
        let res = get_file_dir_name(input);
        assert_eq!(res, None);
    }

    #[test]
    fn test_get_file_name(){
        let input = "wcf://Download/test.docx";
        let res = get_file_name(input);
        assert_eq!(res, Some("test.docx"));
    }
}

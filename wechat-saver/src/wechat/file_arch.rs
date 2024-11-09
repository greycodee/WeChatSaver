use crate::wechat::account::AccountInfo;

struct FileArch {
    account_info: AccountInfo,
    dest_path: String,
}

impl FileArch {
    fn new(account_info: AccountInfo, dest_path: String) -> Self {
        FileArch {
            account_info,
            dest_path,
        }
    }

    fn arch_voice(&self) {
        println!("voice_path: {}", self.account_info.voice_path);
    }

    fn arch_db(&self) {
        println!("en_micro_msg_db_path: {}", self.account_info.en_micro_msg_db_path);
    }

    fn arch_image(&self) {
        println!("image_path: {}", self.account_info.image_path);
    }

    fn arch_avatar(&self) {
        println!("avatar_path: {}", self.account_info.avatar_path);
    }

    fn arch_video(&self) {
        println!("video_path: {}", self.account_info.video_path);
    }

    fn arch_download(&self) {
        println!("download_path: {}", self.account_info.download_path);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::wechat::account::AccountInfo;

    #[test]
    fn test_file_arch() {
        let account_info = AccountInfo {
            account_uin: "1727242265".to_string(),
            wx_user_info: None,
            video_path: "video_path".to_string(),
            voice_path: "voice_path".to_string(),
            image_path: "image_path".to_string(),
            avatar_path: "avatar_path".to_string(),
            download_path: "download_path".to_string(),
            en_micro_msg_db_path: "en_micro_msg_db_path".to_string(),
            wx_file_index_db_path: "wx_file_index_db_path".to_string(),
            db_private_key: "db_private_key".to_string(),
        };
        let file_arch = FileArch::new(account_info, "dest_path".to_string());
        file_arch.arch_voice();
        file_arch.arch_db();
        file_arch.arch_image();
        file_arch.arch_avatar();
        file_arch.arch_video();
        file_arch.arch_download();
    }
}
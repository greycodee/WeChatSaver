use std::fs;
use std::path::{Path, PathBuf};
use crate::wechat::account::AccountInfo;
use std::io::Result;
#[derive(Debug)]
struct FileArch<'a> {
    account_info: &'a AccountInfo,
    dest_path: PathBuf,
}

impl<'a> FileArch<'a> {
    fn new(account_info: &'a AccountInfo, dest_path: &Path) -> Self {
        let user_space_path = dest_path.join(&account_info.wx_user_info.wx_id);
        FileArch {
            account_info,
            dest_path:user_space_path
        }
    }

    fn arch_all(&self) -> Result<()> {
        self.arch_voice()?;
        self.arch_db()?;
        self.arch_image()?;
        self.arch_avatar()?;
        self.arch_video()?;
        self.arch_download()?;
        Ok(())
    }

    fn arch_voice(&self)  -> Result<()>  {
        let src_path = Path::new(&self.account_info.voice_path);
        let dst_path = &self.dest_path.join("voice2");
        self.copy_dir_all(src_path, dst_path)?;
        Ok(())
    }

    fn arch_db(&self)  -> Result<()> {
        let db_path = &self.dest_path.join("db");
        if !db_path.exists() {
            fs::create_dir_all(db_path)?;
        }
        let en_micro_msg_db_path = Path::new(&self.account_info.en_micro_msg_db_path);
        let en_micro_msg_db_dst_path = db_path.join(en_micro_msg_db_path.file_name().unwrap());
        fs::copy(en_micro_msg_db_path, en_micro_msg_db_dst_path)?;
        let wx_file_index_db_path = Path::new(&self.account_info.wx_file_index_db_path);
        let wx_file_index_db_dst_path = db_path.join(wx_file_index_db_path.file_name().unwrap());
        fs::copy(wx_file_index_db_path, wx_file_index_db_dst_path)?;

        Ok(())
    }

    fn arch_image(&self) -> Result<()>  {
        let src_path = Path::new(&self.account_info.image_path);
        let dst_path = &self.dest_path.join("image2");
        self.copy_dir_all(src_path, dst_path)?;
        Ok(())
    }

    fn arch_avatar(&self) -> Result<()> {
        let src_path = Path::new(&self.account_info.avatar_path);
        let dst_path = &self.dest_path.join("avatar");
        self.copy_dir_all(src_path, dst_path)?;
        Ok(())
    }

    fn arch_video(&self)  -> Result<()>{
        let src_path = Path::new(&self.account_info.video_path);
        let dst_path = &self.dest_path.join("video");
        self.copy_dir_all(src_path, dst_path)?;
        Ok(())
    }

    fn arch_download(&self) -> Result<()> {
        let src_path = Path::new(&self.account_info.download_path);
        let dst_path = &self.dest_path.join("Download");
        self.copy_dir_all(src_path, dst_path)?;
        Ok(())
    }

    fn copy_dir_all(&self,src: &Path, dst: &Path) -> Result<()> {
        if !dst.exists() {
            fs::create_dir_all(dst)?;
        }
        if !src.exists() {
            return Ok(());
        }
        for entry in fs::read_dir(src)? {
            let entry = entry?;
            let file_type = entry.file_type()?;
            let src_path = entry.path();
            let dst_path = dst.join(entry.file_name());

            if file_type.is_dir() {
                self.copy_dir_all(&src_path, &dst_path)?;
            } else {
                fs::copy(&src_path, &dst_path)?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::wechat::account::{AccountInfo};


    // fn crate_account_info() -> AccountInfo {
    //     AccountInfo {
    //         account_uin: "1727242265".to_string(),
    //         wx_user_info: WXUserInfo {
    //             wx_id: "wxid_1sdas111".to_string(),
    //             wx_account_no: "".to_string(),
    //             account_name: "".to_string(),
    //             account_phone: "".to_string(),
    //             account_avatar_path: "".to_string(),
    //         },
    //         video_path: "video_path".to_string(),
    //         voice_path: "voice_path".to_string(),
    //         image_path: "image_path".to_string(),
    //         avatar_path: "avatar_path".to_string(),
    //         download_path: "download_path".to_string(),
    //         en_micro_msg_db_path: "en_micro_msg_db_path".to_string(),
    //         wx_file_index_db_path: "wx_file_index_db_path".to_string(),
    //         db_private_key: "db_private_key".to_string(),
    //     }
    // }

    // #[test]
    // fn test_file_arch() {
    //     let account_info = crate_account_info();
    //     let dest_path = Path::new("/tmp/test");
    //     let file_arch = FileArch::new(&account_info, dest_path);
    //     println!("dest_path: {:?}", file_arch.dest_path);
    //     println!("dest_path:{}",dest_path.display());
    // }

    #[test]
    fn test_arch_voice() {
        let uin = "1727242265";
        let base_path = Path::new("/Users/zheng/Downloads/20241024_091952");

        let account_info = AccountInfo::new(base_path, uin).unwrap();

        let dest_path = Path::new("/tmp/wechat");
        let file_arch = FileArch::new(&account_info, dest_path);
        file_arch.arch_voice().unwrap();
    }

    #[test]
    fn test_arch_db() {
        let uin = "1727242265";
        let base_path: &Path = Path::new("/sdcard/Android/data/com.tencent.mm");
        let account_info = AccountInfo::new(base_path, uin).unwrap();

        let dest_path = Path::new("/tmp/wechat");
        let file_arch = FileArch::new(&account_info, dest_path);
        file_arch.arch_db().unwrap();
    }

    #[test]
    fn test_arch_image() {
        let uin = "1727242265";
        let base_path: &Path = Path::new("/sdcard/Android/data/com.tencent.mm");

        let account_info = AccountInfo::new(base_path, uin).unwrap();

        let dest_path = Path::new("/tmp/wechat");
        let file_arch = FileArch::new(&account_info, dest_path);
        file_arch.arch_image().unwrap();
    }

    #[test]
    fn test_arch_avatar() {
        let uin = "1727242265";
        let base_path: &Path = Path::new("/sdcard/Android/data/com.tencent.mm");

        let account_info = AccountInfo::new(base_path, uin).unwrap();

        let dest_path = Path::new("/tmp/wechat");
        let file_arch = FileArch::new(&account_info, dest_path);
        file_arch.arch_avatar().unwrap();
    }

    #[test]
    fn test_arch_video() {
        let uin = "1727242265";
        let base_path: &Path = Path::new("/sdcard/Android/data/com.tencent.mm");

        let account_info = AccountInfo::new(base_path, uin).unwrap();

        let dest_path = Path::new("/tmp/wechat");
        let file_arch = FileArch::new(&account_info, dest_path);
        file_arch.arch_video().unwrap();
    }

    #[test]
    fn test_arch_download() {
        let uin = "1727242265";
        let base_path: &Path = Path::new("/sdcard/Android/data/com.tencent.mm");

        let account_info = AccountInfo::new(base_path, uin).unwrap();

        let dest_path = Path::new("/tmp/wechat");
        let file_arch = FileArch::new(&account_info, dest_path);
        file_arch.arch_download().unwrap();
    }

    #[test]
    fn test_arch_all() {
        let uin = "1727242265";
        let base_path: &Path = Path::new("/sdcard/Android/data/com.tencent.mm");

        let account_info = AccountInfo::new(base_path, uin).unwrap();

        let dest_path = Path::new("/tmp/wechat");
        let file_arch = FileArch::new(&account_info, dest_path);
        file_arch.arch_all().unwrap();
    }
}
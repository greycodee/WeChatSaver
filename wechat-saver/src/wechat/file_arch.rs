use crate::wechat::account::AccountInfo;
use crate::wechat::databases::wechat_saver_db::WeChatSaverDB;
use std::fs;
use std::io::{Error, Result};
use std::path::{Path, PathBuf};

#[derive(Debug)]
struct FileArch<'a> {
    account_info: &'a AccountInfo,
    dest_path: PathBuf,
    wechat_saver_db: WeChatSaverDB,
}

impl<'a> FileArch<'a> {
    /**
        @param base_path: workspace
    */
    fn new(base_path: &Path, account_info: &'a AccountInfo) -> std::io::Result<Self> {
        let user_space_path = base_path.join(&account_info.wx_user_info.wx_id);
        if !user_space_path.exists() {
            fs::create_dir_all(&user_space_path)?;
        }
        if let Ok(wechat_saver_db) = WeChatSaverDB::new(&user_space_path) {
            Ok(FileArch {
                account_info,
                dest_path: user_space_path,
                wechat_saver_db,
            })
        } else {
            Err(Error::new(
                std::io::ErrorKind::Other,
                "create wechat saver db error",
            ))
        }
    }

    fn arch_all(&mut self) -> Result<()> {
        self.arch_voice()?;
        self.arch_db()?;
        self.arch_image()?;
        self.arch_avatar()?;
        self.arch_video()?;
        self.arch_download()?;

        // TODO 删除临时文件夹
        // TODO 删除lock文件
        Ok(())
    }

    fn arch_voice(&self) -> Result<()> {
        let src_path = Path::new(&self.account_info.voice_path);
        let dst_path = &self.dest_path.join("voice2");
        self.copy_dir_all(src_path, dst_path)?;
        Ok(())
    }

    fn arch_db(&mut self) -> Result<()> {
        // self.account_info.db_conn.init_save_db()
        // TODO 考虑增量备份的情况
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

    fn arch_db_message_table(&self) -> Result<()> {
        // 每次查询 500 条数据
        let mut offset = 0;
        let limit = 500;
        loop {
            let message_list = self
                .account_info
                .db_conn
                .select_message_with_limit(offset, limit);
            match message_list {
                Ok(list) => {
                    if list.is_empty() {
                        break;
                    }
                    for message in list {
                        if let Ok(true) = self.wechat_saver_db.addition_flag(
                            message.msg_svr_id,
                            &message.talker,
                            message.create_time,
                        ) {
                            if let Err(e) = self.wechat_saver_db.save_message(&message) {
                                println!("save message error: {:?}", e);
                            }
                        }
                    }
                }
                Err(e) => {
                    break;
                }
            }
            offset += limit;
        }
        Ok(())
    }

    fn arch_image(&self) -> Result<()> {
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

    fn arch_video(&self) -> Result<()> {
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

    fn copy_dir_all(&self, src: &Path, dst: &Path) -> Result<()> {
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
    use crate::wechat::account::AccountInfo;

    #[test]
    fn test_arch_db_message_table() {
        let uin = "1727242265";
        let base_path: &Path =
            Path::new("/tmp/com.tencent.mm/2aa8c917-cab9-446e-85df-b777695ddcc8");

        let account_info = AccountInfo::new(base_path, uin).unwrap();

        let dest_path = Path::new("/tmp/com.tencent.mm");
        let file_arch = FileArch::new(dest_path, &account_info).unwrap();
        file_arch.arch_db_message_table().unwrap();
    }
}

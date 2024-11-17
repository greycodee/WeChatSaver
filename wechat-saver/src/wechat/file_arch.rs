use crate::wechat::account::AccountInfo;
use crate::wechat::databases::wechat_saver_db::WeChatSaverDB;
use std::fs;
use std::io::{Error, Result};
use std::path::{Path, PathBuf};
use crate::wechat::utils::change_file_extension;
use crate::wechat::voice_decode::{convert_all_voice_to_mp3, wechat_voice_decode};
use crate::wechat::wx_file_index::{get_after_double_slash, get_file_dir_name, get_file_name, FileDirName};

#[derive(Debug)]
pub struct FileArch<'a> {
    account_info: &'a AccountInfo,
    dest_path: PathBuf,
    wechat_saver_db: WeChatSaverDB,
}

impl<'a> FileArch<'a> {
    /**
        @param base_path: workspace
    */
    pub fn new(base_path: &Path, account_info: &'a AccountInfo) -> std::io::Result<Self> {
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

    pub fn arch_all(&mut self) -> Result<()> {
        self.arch_db()?;
        self.arch_voice()?;
        self.arch_image()?;
        self.arch_avatar()?;
        self.arch_video()?;
        self.arch_openapi()?;
        self.arch_attachment()?;
        // TODO 删除临时文件夹
        // TODO 删除lock文件
        Ok(())
    }

    fn arch_db(&mut self) -> Result<()> {
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

        self.arch_db_message_table()?;
        self.arch_db_r_contact_table()?;
        self.arch_db_user_info_table()?;

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
                        if let Ok(true) = self.wechat_saver_db.addition_message_flag(
                            message.msg_svr_id,
                            &message.talker,
                            message.create_time,
                        ) {
                            if let Ok(count) = self.wechat_saver_db.save_message(&message) {
                                // TODO 如果是多线程，注意获取可能不准确，后续上多线程的话，msg_id 进行手动维护
                                let latest_rows_id = self.wechat_saver_db.get_last_insert_row_id();
                                println!("latest_rows_id: {}", latest_rows_id);
                                // TODO process WXFileIndex3
                                // TODO get msg_id of new insert message
                                self.arch_db_wx_file_index_by_msg_id(message.msg_id,latest_rows_id)?;
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

    fn arch_db_r_contact_table(&self) -> Result<()> {
        // TODO 考虑返回冲突的联系人，选择是否更新
        // TODO 标记删除的联系人
        let mut offset = 0;
        let limit = 500;
        loop {
            let contact_list = self.account_info.db_conn.select_r_contact_with_limit(offset, limit);
            match contact_list {
                Ok(list) => {
                    if list.is_empty() {
                        break;
                    }
                    for contact in list {
                        if let Err(e) = self.wechat_saver_db.save_r_contact(&contact) {
                            println!("save r contact error: {:?}", e);
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

    fn arch_db_user_info_table(&self) -> Result<()> {
        let mut offset = 0;
        let limit = 500;
        loop {
            let user_info_list = self.account_info.db_conn.select_user_info_with_limit(offset, limit);
            match user_info_list {
                Ok(list) => {
                    if list.is_empty() {
                        break;
                    }
                    for user_info in list {
                        if let Err(e) = self.wechat_saver_db.save_user_info(&user_info) {
                            println!("save user info error: {:?}", e);
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

    fn arch_db_wx_file_index_by_msg_id(&self,old_msg_id:i64,new_msg_id:i64) -> Result<()> {
        if let Ok(old_wx_file_index_opt) = self.account_info.db_conn.select_wx_file_index_by_msg_id(old_msg_id){
            if let Some(old_wx_file_index) = old_wx_file_index_opt {
                let mut new_wx_file_index = old_wx_file_index.clone();
                new_wx_file_index.msg_id = new_msg_id;

                match get_file_dir_name(&old_wx_file_index.path) {
                    None => {}
                    Some(name) => {
                        match name {
                            FileDirName::Download => {
                                self.arch_download(get_file_name(&old_wx_file_index.path).unwrap())?;
                                new_wx_file_index.path = get_after_double_slash(&old_wx_file_index.path).unwrap().to_string();
                            }
                            _ => {
                                new_wx_file_index.path = get_after_double_slash(&old_wx_file_index.path).unwrap().to_string();
                            }
                        }
                    }
                }
                if let Err(e) = self.wechat_saver_db.save_wx_file_index(&new_wx_file_index){
                    println!("save wx file index error: {:?}", e);
                }
            }
        }
        Ok(())
    }

    fn arch_single_voice(&self,voice_file_path:&str) -> Result<PathBuf> {
        let amr_file_path = &self.account_info.voice_path.parent().unwrap().join(voice_file_path);
        if !amr_file_path.exists(){
            return Err(Error::new(std::io::ErrorKind::NotFound,format!("amr file not found: {:?}",amr_file_path)));
        }
        let mp3_file_path = wechat_voice_decode(amr_file_path)?;
        let dst_mp3_relative_path = change_file_extension(voice_file_path.as_ref(), "mp3");
        let dst_path = &self.dest_path.join(&dst_mp3_relative_path);
        if !dst_path.parent().unwrap().exists(){
            fs::create_dir_all(dst_path.parent().unwrap())?;
        }
        if mp3_file_path.exists() {
            fs::copy(mp3_file_path, dst_path)?;
        }
        Ok(dst_mp3_relative_path)
    }

    fn arch_voice(&self) -> Result<()> {
        let src_path = Path::new(&self.account_info.voice_path);
        let dst_path = &self.dest_path.join("voice2");
        self.copy_dir_all(src_path, dst_path)?;

        let sd_card_voice_path = Path::new(&self.account_info.sd_card_voice_path);
        self.copy_dir_all(sd_card_voice_path, dst_path)?;

        convert_all_voice_to_mp3(dst_path)?;
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

        let sd_card_video_path = Path::new(&self.account_info.sd_card_video_path);
        self.copy_dir_all(sd_card_video_path, dst_path)?;
        Ok(())
    }
    fn arch_download(&self,file_name: &str) -> Result<()> {
        let file_path = &self.account_info.download_path.join(file_name);
        let dst_path = &self.dest_path.join("Download");
        if !dst_path.exists(){
            fs::create_dir_all(dst_path)?;
        }
        let dst_path = dst_path.join(file_name);
        if file_path.exists() {
            fs::copy(file_path, dst_path)?;
        }
        Ok(())
    }

    fn arch_openapi(&self) -> Result<()> {
        let src_path = Path::new(&self.account_info.openapi_path);
        let dst_path = &self.dest_path.join("openapi");
        self.copy_dir_all(src_path, dst_path)?;
        Ok(())
    }

    fn arch_attachment(&self) -> Result<()> {
        let src_path = Path::new(&self.account_info.attachment_path);
        let dst_path = &self.dest_path.join("attachment");
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

    #[test]
    fn test_arch_db_r_contact_table(){
        let uin = "1727242265";
        let base_path: &Path =
            Path::new("/tmp/com.tencent.mm/2aa8c917-cab9-446e-85df-b777695ddcc8");

        let account_info = AccountInfo::new(base_path, uin).unwrap();

        let dest_path = Path::new("/tmp/com.tencent.mm");
        let file_arch = FileArch::new(dest_path, &account_info).unwrap();
        file_arch.arch_db_r_contact_table().unwrap();
    }

    #[test]
    fn test_arch_db_user_info_table(){
        let uin = "1727242265";
        let base_path: &Path =
            Path::new("/tmp/com.tencent.mm/2aa8c917-cab9-446e-85df-b777695ddcc8");

        let account_info = AccountInfo::new(base_path, uin).unwrap();

        let dest_path = Path::new("/tmp/com.tencent.mm");
        let file_arch = FileArch::new(dest_path, &account_info).unwrap();
        file_arch.arch_db_user_info_table().unwrap();
    }

    #[test]
    fn test_path(){
        let p = Path::new("/asd/bbb/ccc/aa.d");
        let parent = p.parent().unwrap();
        println!("{:?}",parent);
    }
}

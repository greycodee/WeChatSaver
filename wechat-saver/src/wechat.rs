use quick_xml::events::Event;
use quick_xml::Reader;
use std::path::{Path, PathBuf};
mod account;
mod android_backup;
mod databases;
mod ffmpeg;
mod file_arch;
mod file_path;
pub mod model;
mod utils;
mod voice_decode;
mod wx_file_index;
mod message_parse;

use crate::wechat::android_backup::file::{unpack_android_backup, unpack_zip_file};
use account::AccountInfo;

/**
    @param work_space: 这个项目的工作空间
    @param android_backup_file: 微信备份文件的路径，一般以 .bak 或者 .db 为后缀的文件
    @param android_sdcard_file: 媒体数据的备份压缩包，一般是一个 zip 文件
    @return: 返回一个临时文件夹的路径（临时工作空间）
*/
pub fn process_backup_file(
    work_space: &Path,
    android_backup_file: &Path,
    android_sdcard_file: &Path,
) -> std::io::Result<PathBuf> {
    // 判断 android_backup_file 和 android_sdcard 是否存在
    if !android_backup_file.exists() {
        panic!("android_backup_file not exists");
    }
    if !android_sdcard_file.exists() {
        panic!("android_sdcard not exists");
    }
    // 判断work_space是否存在，不存在则创建
    if !work_space.exists() {
        std::fs::create_dir_all(work_space)?;
    }

    // 判断work_space 是否存在lock文件，存在则退出
    let lock_file = work_space.join("lock");
    if lock_file.exists() {
        let temp_dir_name = std::fs::read_to_string(&lock_file)?;
        let temp_dir = work_space.join(&temp_dir_name);
        return Ok(temp_dir);
    }

    let temp_dir_name = uuid::Uuid::new_v4().to_string();
    let temp_dir = work_space.join(&temp_dir_name);
    if !temp_dir.exists() {
        std::fs::create_dir_all(&temp_dir)?;
    }

    // 创建lock文件
    std::fs::File::create(&lock_file)?;
    std::fs::write(&lock_file, &temp_dir_name)?;

    unpack_android_backup(android_backup_file, &temp_dir)?;
    // 解压android_sdcard到临时文件夹
    unpack_zip_file(android_sdcard_file, &temp_dir)?;
    Ok(temp_dir)
}

pub fn get_all_account(base_path: &Path) -> std::io::Result<Vec<AccountInfo>> {
    let uin_vec = get_all_uin(base_path);
    let mut account_vec = Vec::new();
    for uin in uin_vec {
        let account_info = AccountInfo::new(base_path, &uin)?;
        account_vec.push(account_info);
    }
    Ok(account_vec)
}

fn get_all_uin(base_path: &Path) -> Vec<String> {
    let mut uin_vec = Vec::new();
    let uin_file_path = base_path.join("apps/com.tencent.mm/sp/app_brand_global_sp.xml");
    let mut reader = Reader::from_file(uin_file_path).unwrap();
    reader.config_mut().trim_text(true);
    loop {
        match reader.read_event_into(&mut Vec::new()) {
            Ok(Event::Text(e)) => match String::from_utf8(e.into_inner().into_owned()) {
                Ok(uin) => {
                    uin_vec.push(uin);
                }
                Err(e) => {
                    panic!("Error: {:?}", e);
                }
            },
            Ok(Event::Eof) => break,
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (),
        }
    }
    uin_vec
}


pub fn quick_run(work_space: &Path, android_backup_file: &Path, android_sdcard_file: &Path) -> std::io::Result<()> {
    let temp_dir = process_backup_file(work_space, android_backup_file, android_sdcard_file)?;
    let account_vec = get_all_account(&temp_dir)?;
    for account in account_vec {
        let file_arch = file_arch::FileArch::new(work_space, &account);
        file_arch.unwrap().arch_all()?;
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use std::path::Path;

    use super::*;



    #[test]
    fn test_run() {
        let work_space = Path::new("/Volumes/hkdisk/wechat-backup/20241117");
        let android_backup_file = Path::new("/Volumes/hkdisk/wechat-backup/20241024_091952/wechat.bak");
        let android_sdcard = Path::new("/Volumes/hkdisk/wechat-backup/20241024_091952/backup_wechat.zip");
        match process_backup_file(work_space, android_backup_file, android_sdcard) {
            Ok(temp_dir) => {
                println!("temp_dir: {:?}", temp_dir);
            }
            Err(e) => {
                panic!("{}", e);
            }
        }
    }

    #[test]
    fn test_get_all_uin() {
        let base_path = Path::new("/Volumes/hkdisk/wechat-backup/20241117/84105892-ff2b-4377-914b-2b2e24b24661");
        let res = get_all_uin(base_path);
        println!("{:?}", res);
    }

    #[test]
    fn test_get_all_account() {
        let base_path = Path::new("/Volumes/hkdisk/wechat-backup/20241117/84105892-ff2b-4377-914b-2b2e24b24661");
        let res = get_all_account(base_path);
        println!("{:?}", res);
    }

    #[test]
    fn test_arch_all(){
        let base_path = Path::new("/Volumes/hkdisk/wechat-backup/20241117/84105892-ff2b-4377-914b-2b2e24b24661");
        let res = get_all_account(base_path);
        let account_vec = res.unwrap();
        let account = &account_vec[2];

        println!("account: {:?}",account);

        let workspace = Path::new("/Volumes/hkdisk/wechat-backup/20241117");

        let file_arch = file_arch::FileArch::new(workspace,account);
        let res = file_arch.unwrap().arch_all();
        println!("res: {:?}",res);

        // for account in res.unwrap(){
        //     let file_arch = file_arch::FileArch::new(workspace,&account);
        //     let res = file_arch.unwrap().arch_all();
        //     println!("res: {:?}",res);
        // }
    }
}

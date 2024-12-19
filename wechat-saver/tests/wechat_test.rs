mod common;

use std::path::Path;
use std::env;
use wechat_saver_lib::wechat::{get_all_account, process_backup_file, quick_run};


#[test]
fn test_dotenv(){
    common::setup();
    let version = std::env::var("WECHAT_VERSION").unwrap();
    assert_eq!(version, "1.0.0");
}

#[test]
fn test_process_backup_file() {
    common::setup();
    let work_space_path = env::var("WORK_SPACE_PATH").unwrap();
    let android_backup_file = env::var("ANDROID_BACKUP_FILE").unwrap();
    let android_sdcard = env::var("ANDROID_SDCARD_ZIP_FILE").unwrap();

    let work_space = Path::new(work_space_path.as_str());
    let android_backup_file = Path::new(android_backup_file.as_str());
    let android_sdcard = Path::new(android_sdcard.as_str());
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
fn test_get_all_account() {
    common::setup();
    let temp_path = env::var("TEMP_PATH").unwrap();
    println!("temp_path: {:?}", temp_path);
    let base_path = Path::new(temp_path.as_str());
    let res = get_all_account(base_path);
    println!("{:?}", res);
}

#[test]
fn test_quick_run(){
    common::setup();

    let work_space = env::var("WORK_SPACE_PATH").unwrap();
    let android_backup_file = env::var("ANDROID_BACKUP_FILE").unwrap();
    let android_sdcard = env::var("ANDROID_SDCARD_ZIP_FILE").unwrap();

    let work_space = Path::new(work_space.as_str());
    let android_backup_file = Path::new(android_backup_file.as_str());
    let android_sdcard = Path::new(android_sdcard.as_str());
    match quick_run(work_space, android_backup_file, android_sdcard) {
        Ok(_) => {
            println!("run success!");
        }
        Err(e) => {
            panic!("{}", e);
        }
    }
}
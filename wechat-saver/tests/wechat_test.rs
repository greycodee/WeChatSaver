mod common;

use std::path::Path;
use std::env;
use wechat_saver_lib::wechat::{get_all_account, process_backup_file, quick_run};


#[test]
fn test_dotenv(){
    common::setup();
    // get .env value
    let version = std::env::var("WECHAT_VERSION").unwrap();
    assert_eq!(version, "1.0.0");
}

#[test]
fn test_process_backup_file() {
    common::setup();
    let work_space = Path::new(env::var("WORK_SPACE_PATH").unwrap().as_str());
    let android_backup_file = Path::new(env::var("ANDROID_BACKUP_FILE").unwrap().as_str());
    let android_sdcard = Path::new(env::var("ANDROID_SDCARD_ZIP_FILE").unwrap().as_str());
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
    let base_path = Path::new(env::var("TEMP_PATH").unwrap().as_str());
    let res = get_all_account(base_path);
    println!("{:?}", res);
}

#[test]
fn test_quick_run(){
    let work_space = Path::new(env::var("WORK_SPACE_PATH").unwrap().as_str());
    let android_backup_file = Path::new(env::var("ANDROID_BACKUP_FILE").unwrap().as_str());
    let android_sdcard = Path::new(env::var("ANDROID_SDCARD_ZIP_FILE").unwrap().as_str());
    match quick_run(work_space, android_backup_file, android_sdcard) {
        Ok(_) => {
            println!("run success!");
        }
        Err(e) => {
            panic!("{}", e);
        }
    }
}
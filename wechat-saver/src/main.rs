use wechat_saver_lib::wechat::quick_run;

fn main() {
    let work_space = std::path::Path::new("/Volumes/hkdisk/wechat-backup/20241201/workspace2");
    let android_backup_file = std::path::Path::new("/Volumes/hkdisk/wechat-backup/20241201/wechat.bak");
    let android_sdcard = std::path::Path::new("/Volumes/hkdisk/wechat-backup/20241201/backup_wechat.zip");
    match quick_run(work_space, android_backup_file, android_sdcard) {
        Ok(_) => {
            println!("run success!");
        }
        Err(e) => {
            panic!("{}", e);
        }
    }
}

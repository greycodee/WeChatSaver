use crate::wechat::account::AccountInfo;

pub fn arch_voice(account_info: &AccountInfo) {
    println!("voice_path: {}", account_info.voice_path);
}

pub fn arch_db(account_info: &AccountInfo) {
    println!("en_micro_msg_db_path: {}", account_info.en_micro_msg_db_path);
}

pub fn arch_image(account_info: &AccountInfo) {
    println!("image_path: {}", account_info.image_path);
}

pub fn arch_avatar(account_info: &AccountInfo) {
    println!("avatar_path: {}", account_info.avatar_path);
}

pub fn arch_video(account_info: &AccountInfo) {
    println!("video_path: {}", account_info.video_path);
}

pub fn arch_download(account_info: &AccountInfo) {
    println!("download_path: {}", account_info.download_path);
}
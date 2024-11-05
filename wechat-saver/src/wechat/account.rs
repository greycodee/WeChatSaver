
use super::file_path::get_system_file_name;
use super::file_path::get_sd_card_dir_name;
use super::utils::gen_db_private_key;
#[allow(dead_code)]
#[derive(Debug)]
pub struct AccountInfo{
    video_path:String,
    voice_path:String,
    image_path:String,
    avatar_path:String,
    download_path:String,
    en_micro_msg_db_path:String,
    wx_file_index_db_path:String,
    db_private_key:String,
}

impl AccountInfo {

    pub fn new(base_path: &str, uin: &str) -> Self {
        let account_dir_name = get_system_file_name(uin);
        let account_file_path = format!("{}/apps/com.tencent.mm/r/MicroMsg/{}", base_path,account_dir_name);

        let image_path = format!("{}/{}", account_file_path, "image2");
        let avatar_path = format!("{}/{}", account_file_path, "avatar");
        let en_micro_msg_db_path = format!("{}/{}", account_file_path, "EnMicroMsg.db");
        let wx_file_index_db_path = format!("{}/{}", account_file_path, "WxFileIndex.db");

        let account_sd_card_dir_name = get_sd_card_dir_name(base_path, uin).unwrap();
        let account_sd_card_dir_path = format!("{}/Android/data/com.tencent.mm/MicroMsg/{}", base_path, account_sd_card_dir_name);

        let video_path = format!("{}/{}", account_sd_card_dir_path, "video");
        let voice_path = format!("{}/{}", account_sd_card_dir_path, "voice2");
        let download_path = format!("{}/{}", account_sd_card_dir_path, "Download");

        let db_private_key = gen_db_private_key(uin);

        AccountInfo {
            video_path,
            voice_path,
            image_path,
            avatar_path,
            download_path,
            en_micro_msg_db_path,
            wx_file_index_db_path,
            db_private_key,
        }
    }

}

#[cfg(test)]
mod test {
    use super::*;

    const BASE_PATH: &str = "/Users/zheng/Downloads/20241024_091952";

    #[test]
    fn test_account_info() {
        let uin = "-215593504";
        let account_info = AccountInfo::new(BASE_PATH, uin);
        println!("{:?}", account_info);
    }
}
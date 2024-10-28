pub mod model;

pub mod utils{
    use md5::{Md5, Digest};
    pub fn gen_db_private_key(uin: &str) -> String {
        let mut private_key = String::from("1234567890ABCDEF");
        private_key.push_str(uin);
        let mut hasher = Md5::new();
        hasher.update(private_key);
        let result = hasher.finalize();

        let result = hex::encode(result);
        result
        // println!("{:?}",result);
        // result[0..7].to_vec().iter().map(|x| format!("{:02x}", x)).collect::<String>()
    }

    pub fn gen_system_file_ptah_name(uin: &str) -> String {
        let mut private_key = String::from("mm");
        private_key.push_str(uin);
        let mut hasher = Md5::new();
        hasher.update(private_key);
        let result = hasher.finalize();
        result.iter().map(|x| format!("{:02x}", x)).collect::<String>()
    }
}

mod test{
    use crate::wechat::utils::gen_db_private_key;
    // use super::*;
    #[test]
    fn test_gen_db_private_key(){
        let uin = "1727242265";
        let key = gen_db_private_key(uin);
        println!("key: {}",key);
    }

    #[test]
    fn test_gen_system_file_ptah_name(){
        let uin = "1727242265";
        let key = crate::wechat::utils::gen_system_file_ptah_name(uin);
        println!("key: {}",key);
    }
}
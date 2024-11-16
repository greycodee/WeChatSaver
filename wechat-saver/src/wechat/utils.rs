use md5::{Digest, Md5};

pub fn gen_db_private_key(uin: &str) -> String {
    let mut private_key = String::from("1234567890ABCDEF");
    private_key.push_str(uin);
    let md5_private_key = md5_encode(&private_key);
    md5_private_key[0..7].to_string()
}

pub fn md5_encode(input: &str) -> String {
    let mut hasher = Md5::new();
    hasher.update(input);
    let result = hasher.finalize();
    let result = hex::encode(result);
    result
}

mod test {
    use crate::wechat::utils::gen_db_private_key;

    #[test]
    fn test_gen_db_private_key() {
        let uin = "1727242265";
        let key = gen_db_private_key(uin);
        println!("key: {}", key);
    }

    #[test]
    fn test_md5_encode() {
        let input = "123123";
        let res = crate::wechat::utils::md5_encode(input);
        println!("{:?}", res);
    }
}

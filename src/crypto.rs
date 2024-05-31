use crate::trait_crypto::Crypto;
use md5::compute;
pub struct Md5;

impl Crypto for Md5 {
    fn hash(&self, password: &String) -> String {
        let password = format!("{:x}", compute(password));
        return password;
    }
}


pub trait Crypto {
    fn hash(&self, password: &String) -> String;
}

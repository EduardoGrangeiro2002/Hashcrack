// Resposabilidade =>
// Ele será responsável por fazer um loop de tentativas chamando a função de hash e verificando se a hash foi encontrada ou não
// Ele terá que informar se conseguiu encontrar a hash, se ja testou todas as combinações do seu spectro e se está livre para pegar mais trabalho.
pub mod worker {
    use crate::{crypto::Md5, trait_crypto::Crypto};

    pub struct Worker;
    
    impl Worker {        
        pub fn worker(password: &String, hash: String, algorithim: String) -> bool {
            match algorithim.as_str() {
                "md5" => {
                    let compare_hash = Md5.hash(&password);
                    hash.eq(&compare_hash)
                }
                _ => {
                    let compare_hash = Md5.hash(&password);
                    hash.eq(&compare_hash)
                }
            }
            
        }
    }
}
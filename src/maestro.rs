// Responsabilidades =>
// Será responsável por workestrar grupos de threads onde se comunicará via canais para enviar informações
// As informações são para parar, novas senhas, worker livre e quando encontrar a senha
// Quando encontrar a senha ele envia por canal uma ordem de pausa e para todos os workers.
// Caso ele receba a informação de que tem um worker livre ele enviar mais senhas se tiver, se não tiver ele manda uma ordem de pausa.

pub mod maestro {
    use std::sync::mpsc::Receiver;
    use crate::worker::worker::Worker;
    use std::thread;

    pub struct Maestro;
    impl Maestro {
        pub fn start_thread(receiver: Receiver<(String, String, String)>) {
            thread::spawn(move || {
                while let Ok((password, hash, crypto_hash)) = receiver.recv() {
                    let found_password = Worker::worker(&password, hash, crypto_hash);
                    if found_password == true {
                        println!("Encontrou a senha! {}", password);
                        break;
                    }
                }
            });
        }
    }
}
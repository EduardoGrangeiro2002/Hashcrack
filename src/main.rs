mod crypto;
mod trait_crypto;
mod worker;
mod maestro;
use encoding_rs_io::DecodeReaderBytesBuilder;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::sync::mpsc::sync_channel;
use crate::maestro::maestro::Maestro;

fn main() -> io::Result<()> {
    let hash = "bebe5df1ad650edd0c017aaad908fb14";
    let md5 = "md5";
    let num_core = 1;
    let mut actual_core = 0;
    let file = File::open("./file/rockyou.txt").expect("Arquivo não encontrado!");
    let reader = BufReader::new(DecodeReaderBytesBuilder::new().build(file));
    let mut senders = Vec::new();
    for _ in 0..num_core {
        let ( sender, receiver ) = sync_channel::<(String, String, String)>(1);
        senders.push(sender);
        Maestro::start_thread(receiver);
    }
    for line in reader.lines() {
        let line = line.expect("Erro ao ler a linha");
        let _= senders[actual_core].send((line, hash.to_string(), md5.to_string()));
        if actual_core == num_core - 1 {
            actual_core = 0;
        } else {
            actual_core+= 1;
        }
    }
    Ok(())
}
use std::io::prelude::*;
use std::net::TcpStream;
use recoverSecret::{RecoverSecretInput, validateRecoverSecretSentence};

mod recoverSecret;

fn main() {
    // let mut stream = TcpStream::connect("127.0.0.1:7878").unwrap();
    // let message = "I'm trying to speak with you 😄";
    //
    // println!("{}", message.len());
    // let len = message.len() as u32;
    // stream.write(&len.to_be_bytes()).unwrap(); // on écrit le préfixe (taille du prochain message)
    // stream.write(message.as_bytes()).unwrap(); // puis le message en tant que tel


    let test_word = "C'est chou";
    let recoverSecretInput = RecoverSecretInput{
        word_count: 2,
        letters: "t cCehuCethoCeschouC'schout h".to_string(),
        tuple_sizes: Vec::from([3, 4, 5, 7, 7, 3])
    };
    validateRecoverSecretSentence(test_word, recoverSecretInput);
}

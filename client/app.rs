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


    // Correct
    let test_word = "C'est chou";
    let recoverSecretInput = RecoverSecretInput{
        word_count: 2,
        letters: "t cCehuCethoCeschouC'schout h".to_string(),
        tuple_sizes: Vec::from([3, 4, 5, 7, 7, 3])
    };

    // Not correct
    let test_word2 = "xWRvraj4fonTUmzyO25wA3lBeiM9H";
    let recoverSecretInput2 = RecoverSecretInput{
        word_count: 1,
        letters: "WvyOAlxafUzleiSOl9xayBeHTmy9xWTU5lMW4nUO5lMWRajn2BiHSRUzy5afnUz5wlexWrm5wlBWr4mAlBrUmzHxTUzwlHrfTwBeSRmzlMSRfoUOAe9S4oUiraOiramzM5w3l".to_string(),
        tuple_sizes: Vec::from([6, 8, 4, 6, 4, 7, 8, 9, 6, 9, 8, 7, 5, 7, 6, 6, 9, 5, 4, 5, 4])
    };
    validateRecoverSecretSentence(test_word, recoverSecretInput);
}

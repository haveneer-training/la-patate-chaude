extern crate random_string;

use std::io::prelude::*;
use std::net::TcpStream;
use recoverSecret::{getValueIndex, RecoverSecretInput, validateRecoverSecretSentence};

mod recoverSecret;

fn main() {
    // let mut stream = TcpStream::connect("127.0.0.1:7878").unwrap();
    // let message = "I'm trying to speak with you 😄";
    //
    // println!("{}", message.len());
    // let len = message.len() as u32;
    // stream.write(&len.to_be_bytes()).unwrap(); // on écrit le préfixe (taille du prochain message)
    // stream.write(message.as_bytes()).unwrap(); // puis le message en tant que tel
    // generateRecoverSecretSentence();
}

#[cfg(test)]
mod tests {
    use ::{RecoverSecretInput, validateRecoverSecretSentence};
    use getValueIndex;

    #[test]
    fn should_validate_secret_sentence() {
        // test 1
        // let test_word = "C'est chou";
        let test_word = "Cet 'schou";

        let recoverSecretInput = RecoverSecretInput {
            word_count: 2,
            letters: "t cCehuCethoCeschouC'schout h".to_string(),
            tuple_sizes: Vec::from([3, 4, 5, 7, 7, 3])
            // Cet 'schou
            // si existe pas: ajouter à la fin
            // si existe: le déplacer juste après notre dernière lettre trouvé ou ajouté
        };
        assert!(validateRecoverSecretSentence(test_word, recoverSecretInput));

        // test 2
        let test_word2 = "xWRvraj4fonTUmzyO25wA3lBeiM9H";
        // let test_word2 = "WvAxayfUziSOl9BeH";
        let recoverSecretInput2 = RecoverSecretInput {
            word_count: 1,
            letters: "WvyOAlxafUzleiSOl9xayBeHTmy9xWTU5lMW4nUO5lMWRajn2BiHSRUzy5afnUz5wlexWrm5wlBWr4mAlBrUmzHxTUzwlHrfTwBeSRmzlMSRfoUOAe9S4oUiraOiramzM5w3l".to_string(),
            tuple_sizes: Vec::from([6, 8, 4, 6, 4, 7, 8, 9, 6, 9, 8, 7, 5, 7, 6, 6, 9, 5, 4, 5, 4])
        };
        assert!(validateRecoverSecretSentence(test_word2, recoverSecretInput2));
    }

    #[test]
    fn should_not_validate_secret_sentence() {
        // test 1
        let test_word = "C'set chou";
        let recoverSecretInput = RecoverSecretInput {
            word_count: 2,
            letters: "t cCehuCethoCeschouC'schout h".to_string(),
            tuple_sizes: Vec::from([3, 4, 5, 7, 7, 3])
        };
        assert!(!validateRecoverSecretSentence(test_word, recoverSecretInput));

        // test 2
        let test_word2 = "xWRarvj4fonTUmzyO25wA3BleiM9H";
        let recoverSecretInput2 = RecoverSecretInput {
            word_count: 1,
            letters: "WvyOAlxafUzleiSOl9xayBeHTmy9xWTU5lMW4nUO5lMWRajn2BiHSRUzy5afnUz5wlexWrm5wlBWr4mAlBrUmzHxTUzwlHrfTwBeSRmzlMSRfoUOAe9S4oUiraOiramzM5w3l".to_string(),
            tuple_sizes: Vec::from([6, 8, 4, 6, 4, 7, 8, 9, 6, 9, 8, 7, 5, 7, 6, 6, 9, 5, 4, 5, 4])
        };
        assert!(!validateRecoverSecretSentence(test_word2, recoverSecretInput2));
    }

    #[test]
    fn should_return_good_index_position(){
        assert_eq!(getValueIndex("Hello!".to_string(), "l".to_string()), 2);
        assert_eq!(getValueIndex("Hello!".to_string(), "H".to_string()), 0);
        assert_eq!(getValueIndex("Hello!".to_string(), "o".to_string()), 4);
    }

    #[test]
    fn should_not_return_good_index_position(){
        assert_eq!(getValueIndex("Hello!".to_string(), "j".to_string()), -1);
        assert_eq!(getValueIndex("Hello!".to_string(), "1".to_string()), -1);
    }
}

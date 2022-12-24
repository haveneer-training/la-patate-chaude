extern crate random_string;

use std::borrow::Borrow;
use std::io::prelude::*;
use std::net::TcpStream;
use recoverSecret::{generateRecoverSecretSentence, getValueIndex, RecoverSecretInput, validateRecoverSecretSentence};

mod recoverSecret;

fn main() {
    // let mut stream = TcpStream::connect("127.0.0.1:7878").unwrap();
    // let message = "I'm trying to speak with you 😄";
    //
    // println!("{}", message.len());
    // let len = message.len() as u32;
    // stream.write(&len.to_be_bytes()).unwrap(); // on écrit le préfixe (taille du prochain message)
    // stream.write(message.as_bytes()).unwrap(); // puis le message en tant que tel

    let test2 = "xWvSRra4fTjnoUmzyO5w3Al2BeiM9H";
    let recoverSecretInput2 = RecoverSecretInput {
        word_count: 1,
        letters: "WvyOAlxafUzleiSOl9xayBeHTmy9xWTU5lMW4nUO5lMWRajn2BiHSRUzy5afnUz5wlexWrm5wlBWr4mAlBrUmzHxTUzwlHrfTwBeSRmzlMSRfoUOAe9S4oUiraOiramzM5w3l".to_string(),
        tuple_sizes: Vec::from([6, 8, 4, 6, 4, 7, 8, 9, 6, 9, 8, 7, 5, 7, 6, 6, 9, 5, 4, 5, 4])
    };
    println!("Validated :{}", validateRecoverSecretSentence(test2.to_string(), recoverSecretInput2.tuple_sizes.clone(), recoverSecretInput2.letters.clone()));
}

#[cfg(test)]
mod tests {
    use ::{RecoverSecretInput, validateRecoverSecretSentence};
    use ::{generateRecoverSecretSentence, getValueIndex};

    #[test]
    fn should_validate_secret_sentence() {
        // test 1
        let test_word = "C'est chou";

        let recoverSecretInput = RecoverSecretInput {
            word_count: 2,
            letters: "t cCehuCethoCeschouC'schout h".to_string(),
            tuple_sizes: Vec::from([3, 4, 5, 7, 7, 3])
        };
        assert!(validateRecoverSecretSentence(test_word.to_string(), recoverSecretInput.tuple_sizes.clone(), recoverSecretInput.letters.clone()));

        // test 2
        let test_word2 = "xWRvraj4fonTUmzyO25wA3lBeiM9H";
        let recoverSecretInput2 = RecoverSecretInput {
            word_count: 1,
            letters: "WvyOAlxafUzleiSOl9xayBeHTmy9xWTU5lMW4nUO5lMWRajn2BiHSRUzy5afnUz5wlexWrm5wlBWr4mAlBrUmzHxTUzwlHrfTwBeSRmzlMSRfoUOAe9S4oUiraOiramzM5w3l".to_string(),
            tuple_sizes: Vec::from([6, 8, 4, 6, 4, 7, 8, 9, 6, 9, 8, 7, 5, 7, 6, 6, 9, 5, 4, 5, 4])
        };
        assert!(validateRecoverSecretSentence(test_word2.to_string(), recoverSecretInput2.tuple_sizes.clone(), recoverSecretInput2.letters.clone()));
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
        assert!(!validateRecoverSecretSentence(test_word.to_string(), recoverSecretInput.tuple_sizes.clone(), recoverSecretInput.letters.clone()));

        // test 2
        let test_word2 = "xWRarvj4fonTUmzyO25wA3BleiM9H";
        let recoverSecretInput2 = RecoverSecretInput {
            word_count: 1,
            letters: "WvyOAlxafUzleiSOl9xayBeHTmy9xWTU5lMW4nUO5lMWRajn2BiHSRUzy5afnUz5wlexWrm5wlBWr4mAlBrUmzHxTUzwlHrfTwBeSRmzlMSRfoUOAe9S4oUiraOiramzM5w3l".to_string(),
            tuple_sizes: Vec::from([6, 8, 4, 6, 4, 7, 8, 9, 6, 9, 8, 7, 5, 7, 6, 6, 9, 5, 4, 5, 4])
        };
        assert!(!validateRecoverSecretSentence(test_word2.to_string(), recoverSecretInput.tuple_sizes.clone(), recoverSecretInput.letters.clone()));
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

    #[test]
    fn should_generate_good_output1(){
        let recoverSecretInput1 = RecoverSecretInput {
            word_count: 2,
            letters: "t cCehuCethoCeschouC'schout h".to_string(),
            tuple_sizes: Vec::from([3, 4, 5, 7, 7, 3])
        };
        let mut recoverSecretOutput: String = generateRecoverSecretSentence(recoverSecretInput1.tuple_sizes.clone(), recoverSecretInput1.letters.clone()).secret_sentence.to_string();

        assert_eq!(recoverSecretOutput, "Cet 'schou");

        assert!(validateRecoverSecretSentence(recoverSecretOutput.to_string(), recoverSecretInput1.tuple_sizes.clone(), recoverSecretInput1.letters.clone()));
    }

    #[test]
    fn should_generate_good_output2(){
        let recoverSecretInput2 = RecoverSecretInput {
            word_count: 1,
            letters: "WvyOAlxafUzleiSOl9xayBeHTmy9xWTU5lMW4nUO5lMWRajn2BiHSRUzy5afnUz5wlexWrm5wlBWr4mAlBrUmzHxTUzwlHrfTwBeSRmzlMSRfoUOAe9S4oUiraOiramzM5w3l".to_string(),
            tuple_sizes: Vec::from([6, 8, 4, 6, 4, 7, 8, 9, 6, 9, 8, 7, 5, 7, 6, 6, 9, 5, 4, 5, 4])
        };

        let mut recoverSecretOutput = generateRecoverSecretSentence(recoverSecretInput2.tuple_sizes.clone(), recoverSecretInput2.letters.clone()).secret_sentence.to_string();

        assert_eq!(recoverSecretOutput, "xWvSRra4fTjnoUmzyO5w3Al2BeiM9H");

        assert!(validateRecoverSecretSentence(recoverSecretOutput.to_string(), recoverSecretInput2.tuple_sizes.clone(), recoverSecretInput2.letters.clone()));
    }
}

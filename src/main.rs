extern crate random_string;

use std::borrow::Borrow;
use std::io::prelude::*;
use std::net::TcpStream;

mod recover_secret;

use recover_secret::data_structures::{RecoverSecretInput, RecoverSecretOutput};
use recover_secret::recover_secret_challenge::{Challenge, RecoverSecretChallenge};

fn main() {
    let first_test_word = "C'est chou";
    let recover_secret_output = RecoverSecretOutput{
        secret_sentence: first_test_word.to_string()
    };

    let recover_secret_input = RecoverSecretInput {
        word_count: 2,
        letters: "t cCehuCethoCeschouC'schout h".to_string(),
        tuple_sizes: Vec::from([3, 4, 5, 7, 7, 3])
    };

    let challenge_recover_secret = RecoverSecretChallenge::new(recover_secret_input);

    let status = challenge_recover_secret.verify(&recover_secret_output);
    let out = challenge_recover_secret.solve();
    // println!("{}", out.secret_sentence);
    println!("{}", status);

}

#[cfg(test)]
mod tests {
    use ::{RecoverSecretInput, RecoverSecretOutput};
    use ::{Challenge, RecoverSecretChallenge};
    use recover_secret::utils::get_value_index;

    #[test]
    fn should_validate_secret_sentence() {
        // First test
        let first_test_output = RecoverSecretOutput{
            secret_sentence: "C'est chou".to_string()
        };

        let first_test_input = RecoverSecretInput {
            word_count: 2,
            letters: "t cCehuCethoCeschouC'schout h".to_string(),
            tuple_sizes: Vec::from([3, 4, 5, 7, 7, 3])
        };

        let challenge_recover_secret = RecoverSecretChallenge::new(first_test_input);
        assert!(challenge_recover_secret.verify(&first_test_output));

        // Second test
        let second_test_output = RecoverSecretOutput{
            secret_sentence: "xWRvraj4fonTUmzyO25wA3lBeiM9H".to_string()
        };
        let second_test_input = RecoverSecretInput {
            word_count: 1,
            letters: "WvyOAlxafUzleiSOl9xayBeHTmy9xWTU5lMW4nUO5lMWRajn2BiHSRUzy5afnUz5wlexWrm5wlBWr4mAlBrUmzHxTUzwlHrfTwBeSRmzlMSRfoUOAe9S4oUiraOiramzM5w3l".to_string(),
            tuple_sizes: Vec::from([6, 8, 4, 6, 4, 7, 8, 9, 6, 9, 8, 7, 5, 7, 6, 6, 9, 5, 4, 5, 4])
        };
        let challenge_recover_secret = RecoverSecretChallenge::new(second_test_input);
        assert!(challenge_recover_secret.verify(&second_test_output));
    }

    #[test]
    fn should_not_validate_secret_sentence() {
        // First test
        let first_test_output = RecoverSecretOutput{
            secret_sentence: "C'est chou".to_string()
        };
        let first_test_input = RecoverSecretInput {
            word_count: 2,
            letters: "t cCehuCethoCeschouC'schout h".to_string(),
            tuple_sizes: Vec::from([3, 4, 5, 7, 7, 3])
        };

        let challenge_recover_secret = RecoverSecretChallenge::new(first_test_input);
        assert!(challenge_recover_secret.verify(&first_test_output));

        // Second test
        let second_test_output = RecoverSecretOutput{
            secret_sentence: "xWRvraj4fonTUmzyO25wA3lBeiM9H".to_string()
        };
        let second_test_input = RecoverSecretInput {
            word_count: 1,
            letters: "WvyOAlxafUzleiSOl9xayBeHTmy9xWTU5lMW4nUO5lMWRajn2BiHSRUzy5afnUz5wlexWrm5wlBWr4mAlBrUmzHxTUzwlHrfTwBeSRmzlMSRfoUOAe9S4oUiraOiramzM5w3l".to_string(),
            tuple_sizes: Vec::from([6, 8, 4, 6, 4, 7, 8, 9, 6, 9, 8, 7, 5, 7, 6, 6, 9, 5, 4, 5, 4])
        };
        let challenge_recover_secret = RecoverSecretChallenge::new(second_test_input);
        assert!(challenge_recover_secret.verify(&second_test_output));
    }

    #[test]
    fn should_return_good_index_position(){
        assert_eq!(get_value_index("Hello!".to_string(), "l".to_string()), 2);
        assert_eq!(get_value_index("Hello!".to_string(), "H".to_string()), 0);
        assert_eq!(get_value_index("Hello!".to_string(), "o".to_string()), 4);
    }

    #[test]
    fn should_not_return_good_index_position(){
        assert_eq!(get_value_index("Hello!".to_string(), "j".to_string()), -1);
        assert_eq!(get_value_index("Hello!".to_string(), "1".to_string()), -1);
    }

    #[test]
    fn should_generate_good_output1(){
        let test_input = RecoverSecretInput {
            word_count: 2,
            letters: "t cCehuCethoCeschouC'schout h".to_string(),
            tuple_sizes: Vec::from([3, 4, 5, 7, 7, 3])
        };
        let challenge_recover_secret = RecoverSecretChallenge::new(test_input);
        let mut test_output = challenge_recover_secret.solve();

        assert_eq!(test_output.secret_sentence, "Cet 'schou");
        assert!(challenge_recover_secret.verify(&test_output));
    }

    #[test]
    fn should_generate_good_output2(){
        let test_input = RecoverSecretInput {
            word_count: 1,
            letters: "WvyOAlxafUzleiSOl9xayBeHTmy9xWTU5lMW4nUO5lMWRajn2BiHSRUzy5afnUz5wlexWrm5wlBWr4mAlBrUmzHxTUzwlHrfTwBeSRmzlMSRfoUOAe9S4oUiraOiramzM5w3l".to_string(),
            tuple_sizes: Vec::from([6, 8, 4, 6, 4, 7, 8, 9, 6, 9, 8, 7, 5, 7, 6, 6, 9, 5, 4, 5, 4])
        };

        let challenge_recover_secret = RecoverSecretChallenge::new(test_input);
        let mut test_output = challenge_recover_secret.solve();

        assert_eq!(test_output.secret_sentence, "xWvSRra4fTjnoUmzyO5w3Al2BeiM9H");
        assert!(challenge_recover_secret.verify(&test_output));
    }
}
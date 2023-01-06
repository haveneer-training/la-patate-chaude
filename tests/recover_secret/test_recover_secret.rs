mod recover_secret;
// TODO To fix
#[cfg(test)]
mod tests {

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
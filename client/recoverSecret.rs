pub struct RecoverSecretInput {
    pub word_count: usize,
    pub letters: String,
    pub tuple_sizes: Vec<usize>,
}

pub struct RecoverSecretOutput {
    pub secret_sentence: String,
}

pub fn validateRecoverSecretSentence(test_word: &str, recoverSecretInput: RecoverSecretInput) -> bool {
    // Validate a sentence test_word with letters and tuple_sizes
    for i in 0..recoverSecretInput.tuple_sizes.len(){
        let mut startInterval = 0;
        let mut endInterval = 0;
        for j in 0..i{
            startInterval = startInterval + recoverSecretInput.tuple_sizes[j];
            endInterval = endInterval + recoverSecretInput.tuple_sizes[j];
        }
        endInterval = endInterval + recoverSecretInput.tuple_sizes[i];

        let mut word = &recoverSecretInput.letters[startInterval..endInterval];
        // println!("{}", word);
        //println!("{}", patternWord(word, test_word));
        let status = patternWord(word, test_word);
        if (!status){
            println!("Status of the sentence '{}' : {status}", test_word);
            return false;
        }
    }
    println!("Status of the sentence '{}' is Good", test_word);
    return true;
}

fn is_sorted<T>(data: Vec<i32>) -> bool
    where
        T: Ord,
{
    data.windows(2).all(|w| w[0] <= w[1])
}

fn patternWord(pattern:&str, word:&str) -> bool {
    // Check contains all need characters
    let mut result: Vec<i32> = Vec::new();
    for i in 0..pattern.len(){
        // Add indexes of found letters in result variable
        for j in 0..word.len(){
            if word.chars().nth(j).unwrap() == pattern.chars().nth(i).unwrap(){
                result.push(j as i32);
            }
        }
    }
    // Check we have the letters in the correct order
    is_sorted::<u8>(result)
}
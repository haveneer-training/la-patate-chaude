use std::ptr::null;
use random_string::generate;
use RecoverSecretStruct::RecoverSecretOutput;

// Verify if a word or sentence respecting the tuple_sizes and letters parameters rules
pub fn validateRecoverSecretSentence(test_word: String, tuple_sizes: Vec<usize>, letters: String) -> bool {

    // Validate a sentence test_word with letters and tuple_sizes
    for i in 0..tuple_sizes.len(){
        let mut startInterval = 0;
        let mut endInterval = 0;
        for j in 0..i{
            startInterval = startInterval + tuple_sizes[j];
            endInterval = endInterval + tuple_sizes[j];
        }
        endInterval = endInterval + tuple_sizes[i];

        let mut word = &letters[startInterval..endInterval];
        // println!("{}", patternWord(word, test_word));
        let status = patternWord(word.to_string(), test_word.to_string());
        if (!status){
            return false;
        }
    }
    return true;
}

// Get the index of letter parameter in word parameter as parameters
pub fn getValueIndex(word: String, letter: String) -> i32 {
    for j in 0..word.len(){
        if word.chars().nth(j).unwrap() == letter.chars().nth(0).unwrap(){
            return j as i32;
        }
    }
    return -1;
}

// Generate a correct Recover Secret Sentence respecting the tuple_sizes and letters parameters rules
pub fn generateRecoverSecretSentence(tuple_sizes: Vec<usize>, letters: String) -> RecoverSecretOutput{
    let mut result: String = "".to_string();
    let mut status = false;
    while !status{
        for i in 0..tuple_sizes.len(){

            // Get the word pattern
            let mut startInterval = 0;
            let mut endInterval = 0;
            for j in 0..i{
                startInterval = startInterval + tuple_sizes[j];
                endInterval = endInterval + tuple_sizes[j];
            }
            endInterval = endInterval + tuple_sizes[i];
            let mut word = &letters[startInterval..endInterval];

            let mut prec_letter:i32;
            for i in 0..word.len(){

                let mut tmp_index = getValueIndex(result.to_string(), word.chars().nth(i).unwrap().to_string());
                // If letter of pattern do not exist, add to the end of result
                if tmp_index==-1 {
                    result = result.to_owned() + &*word.chars().nth(i).unwrap().to_string();
                }
                // If letter of pattern do exist
                else if i!=0 && tmp_index<getValueIndex(result.to_string(), word.chars().nth(i-1).unwrap().to_string()){
                    // Get precedent letter index of word
                    prec_letter = getValueIndex(result.to_string(), word.chars().nth(i-1).unwrap().to_string());

                    // Calculate the nex letter index of word
                    let mut prec_plus = prec_letter + 1;

                    // Get found letter index in result
                    let mut first_word_letter = getValueIndex(result.to_string(), word.chars().nth(0).unwrap().to_string());

                    // Same in the two conditions but in different case
                    // - Cut from current index of result to the first letter of word
                    // - Put it just after the precedent letter index of word
                    if tmp_index < first_word_letter {
                        let mut cutted = &result[tmp_index as usize..first_word_letter as usize].to_string();
                        let new_resutlt = result[..tmp_index as usize].to_string() + &result[first_word_letter as usize..prec_plus as usize].to_string() + &*cutted.to_string() + &result[prec_plus as usize..].to_string();
                        result = new_resutlt;
                    }
                    else{
                        let mut cutted = &result[tmp_index as usize..prec_letter as usize].to_string();
                        let new_resutlt = result[0..tmp_index as usize].to_string() + &result[prec_letter as usize..].to_string() + &*cutted.to_string();
                        result = new_resutlt;
                    }
                }
                prec_letter = i as i32;
            }
        }
        status = validateRecoverSecretSentence(result.clone().to_string(), tuple_sizes.clone(), letters.clone())
    }

    let mut output: RecoverSecretOutput = RecoverSecretOutput{
        secret_sentence: result
    };
    return output;
}

// Verify if a Vector of i32 is sorted
fn is_sorted<T>(data: Vec<i32>) -> bool
    where
        T: Ord,
{
    data.windows(2).all(|w| w[0] <= w[1])
}

// Verify if word parameter repects pattern variable
fn patternWord(pattern:String, word: String) -> bool {
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

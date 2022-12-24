use std::ptr::null;
use random_string::generate;

pub struct RecoverSecretInput {
    pub word_count: usize,
    pub letters: String,
    pub tuple_sizes: Vec<usize>,
}

pub struct RecoverSecretOutput {
    pub secret_sentence: String,
}

// pub fn generateRecoverSecretSentence() -> RecoverSecretOutput{
//     // Your custom charset
//     // let charset = "abcdefghijklmnopqrstuvwxyz";
//     // println!("{}", generate(6, charset));
//
//     // let test_word = "Cet 'schou";
//     let recoverSecretInput = RecoverSecretInput {
//         word_count: 2,
//         letters: "t cCehuCethoCeschouC'schout h".to_string(),
//         tuple_sizes: Vec::from([3, 4, 5, 7, 7, 3])
//         // Cet 'schou
//         // si existe pas: ajouter à la fin
//         // si existe:
//         //    si avant la lettre courante, select à partir de cette lettre jusqu'au précedente; puis la mettre après la précédente
//         //    si après: do noting
//     };
//
//     let mut result = "";
//     for i in 0..recoverSecretInput.tuple_sizes.len(){
//         let mut startInterval = 0;
//         let mut endInterval = 0;
//         for j in 0..i{
//             startInterval = startInterval + recoverSecretInput.tuple_sizes[j];
//             endInterval = endInterval + recoverSecretInput.tuple_sizes[j];
//         }
//         endInterval = endInterval + recoverSecretInput.tuple_sizes[i];
//         let mut word = &recoverSecretInput.letters[startInterval..endInterval];
//         println!("{}", word);
//
//         let mut prec_letter = 0;
//         for i in 0..word.len(){
//             // si existe pas: ajouter à la fin
//             if getValueIndex(result, word.chars().nth(i).unwrap() as &str)==-1 {
//                 result = &*(result.to_owned() + word.chars().nth(i).unwrap() as &str);
//             }
//             // si existe: le déplacer juste après notre dernière lettre trouvé ou ajouté
//             // Get found value index in result
//             let mut tmp_index = getValueIndex(result, word.chars().nth(i).unwrap() as &str);
//
//             if i!=0 && tmp_index<prec_letter{
//                 let mut first_word_letter = getValueIndex(result, word.chars().nth(0).unwrap() as &str);
//                 if tmp_index < first_word_letter {
//                     let mut cutted = word = result[tmp_index..first_word_letter];
//                     let new_resutlt = result[first_word_letter..prec_letter] + cutted + result[prec_letter..];
//                     result = new_resutlt;
//                 }
//                 else{
//                     let mut cutted = word = result[tmp_index..prec_letter];
//                     let new_resutlt = result[0..prec_letter] + cutted + result[prec_letter..];
//                     result = new_resutlt;
//                 }
//             }
//             prec_letter = i as i32;
//         }
//         println!("- {}", result);
//     }
//     println!("Final :     {}   ", result);
//
//     let mut output: RecoverSecretOutput = RecoverSecretOutput{
//         secret_sentence: "C'est chou".to_string()
//     };
//     return output;
// }

pub fn getValueIndex(word: String, letter: String) -> i32 {
    for j in 0..word.len(){
        if word.chars().nth(j).unwrap() == letter.chars().nth(0).unwrap(){
            return j as i32;
        }
    }
    return -1;
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
        println!("{}", word);
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
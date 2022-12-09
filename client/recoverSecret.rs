pub fn validateRecoverSecret() {
    let test_word = "C'est chou";
    let word_count = 2;
    let letters = "t cCehuCethoCeschouC'schout h".to_string();
    let tuple_sizes = [3, 4, 5, 7, 7, 3];

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
        // println!("{}", word);
        //println!("{}", patternWord(word, test_word));
        let status = patternWord(word, test_word);
        if (!status){
            println!("Status of the sentence '{}' : {status}", test_word)
        }
    }
    println!("Status of the sentence '{}' is Good", test_word)

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
        for j in 0..word.len(){
            if word.chars().nth(j).unwrap() == pattern.chars().nth(i).unwrap(){
                result.push(j as i32);
            }
        }
    }
    // Check we have got all the letters
    if (result.len() != pattern.len()){
        return false;
    }
    // Check we have the letters in the correct order
    is_sorted::<u8>(result)
}
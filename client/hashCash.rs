// Données en entrée du challenge
struct MD5HashCashInput {
    // complexity in bits
    complexity: u32,
    // message to sign
    message: String,
}

// Données en sortie du challenge
struct MD5HashCashOutput {
    // Seed used to solve the challenge
    seed: u64,
    // hashcode found using seed + message
    hashcode: String,
}

// Generate a correct hashcode
/*pub fn getHashMd5(input: MD5HashCashInput) -> MD5HashCashOutput {

}*/


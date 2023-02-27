#[test]
fn format_binary(c: char){
    if c == '0' {
        return "0000".to_string();
    } else if c == '1' {
        return "0001".to_string();
    } else if c == '2' {
        return "0010".to_string();
    } else if c == '3' {
        return "0011".to_string();
    } else if c == '4' {
        return "0100".to_string();
    } else if c == '5' {
        return "0101".to_string();
    } else if c == '6' {
        return "0110".to_string();
    } else if c == '7' {
        return "0111".to_string();
    } else if c == '8' {
        return "1000".to_string();
    } else if c == '9' {
        return "1001".to_string();
    } else if c == 'A' {
        return "1010".to_string();
    } else if c == 'B' {
        return "1011".to_string();
    } else if c == 'C' {
        return "1100".to_string();
    } else if c == 'D' {
        return "1101".to_string();
    } else if c == 'E' {
        return "1110".to_string();
    } else if c == 'F' {
        return "1111".to_string();
    };
    return "1".to_string();
}

#[test]
fn convert_hex_to_binary(hex: &String) -> String {
    let mut binary = "".to_string();
    for i in hex.chars() {
        binary = binary.to_owned() + &format_binary(i);
    }
    binary
}

#[test]
fn verify_bit_zero(number: u32, binary: String) -> bool {
    for i in 0..number {
        if binary.chars().nth(i as usize) != Some('0') {
            return false;
        }
    }
    return true;
}


#[test]
fn solve_md5_hash_cash(input: MD5HashCashInput) -> MD5HashCashOutput {
// Map qui stocke les résultats déjà calculés pour éviter de refaire les calculs
    let mut cache: HashMap<MD5HashCashInput, MD5HashCashOutput> = HashMap::new();
    let mut output: MD5HashCashOutput = MD5HashCashOutput{seed: 1, hashcode: "".to_string()};

// Génère une valeur de graine aléatoire
    let mut seed: u64 = 0;

// Tant qu'on n'a pas trouvé une valeur de graine qui résout le challenge
    loop {
        let mut seed_binary = format!("{:X}", seed);
        if seed_binary.len() < 16{
            let zero_to_add = 16-seed_binary.len();
            for i in 0..zero_to_add{
                seed_binary = "0".to_string() + &seed_binary;
            }
        }
        /*for i in seed.to_le_bytes(){
        seed_binary = seed_binary + &*i.to_string();
    }*/
        // Calcul du hashcode en utilisant la graine + le message
        let mut binary = md5::Md5::new();
        binary.update(seed_binary.to_owned() + &input.message.to_string());
        let binary_bin = binary.finalize();
        //let bin = format!("{:X}", binary_bin);
        //println!("{bin}");
        let hashcode = format!("{:X}", binary_bin);
        //println!("{hashcode}");
        //let hashcode = format!("{:X}", md5::compute(format!("{:X}{}", seed, input.message)));

        // Convertit le hashcode en binaire
        let hashcode_bin = convert_hex_to_binary(&hashcode);
        //let hashcode_bin = format!("{:b}", hashcode.into_bytes());

        //regarder si les complexity bits sont egaux à 0 convertir hashcode en bits
        let hashcode_clone = hashcode.clone();
        // Vérifie si le hashcode comprend au moins "complexity" bits égaux à 0
        if verify_bit_zero(input.complexity, hashcode_bin) {
            //Stocke le résultat dans la map pour éviter de refaire les calculs
            //println!("{}", seed_binary);
            let seed_binary_64 = u64::from_str_radix(&seed_binary, 16).unwrap();
            output.seed = seed_binary_64;
            output.hashcode = hashcode_clone;
            //println!("{}", seed_binary_64);
            //cache.insert(input.clone(), MD5HashCashOutput { seed: seed_binary_64, hashcode: hashcode_clone });
            break;
        }

        // Génère une nouvelle valeur de graine aléatoire
        seed = seed+1;
    }

// Retourne le résultat du challenge
    println!("{:?}", output);
    output
}

// Implémentation du trait Challenge pour la structure MD5HashCash
pub struct MD5HashCashChallenge {
    md5HashCashInput: MD5HashCashInput,
    md5HashCashOuput: MD5HashCashOutput,
}

impl Challenge for MD5HashCashChallenge {
    type Input = MD5HashCashInput;
    type Output = MD5HashCashOutput;
    // Le nom du challenge est "MD5 Hash Cash"
    fn name() -> String {
        "MD5 Hash Cash".to_string()
    }

    // Crée un nouveau challenge à partir des données en entrée
    fn new(hashInput: Self::Input) -> Self {
        let hashInput_clone = hashInput.clone();
        MD5HashCashChallenge {
            md5HashCashInput: hashInput,
            md5HashCashOuput: solve_md5_hash_cash(hashInput_clone),
        }
    }

    // Résout le challenge
    fn solve(&self) -> Self::Output {
        self.md5HashCashOuput.clone()
    }

    // Vérifie qu'une sortie est valide pour le challenge
    fn verify(&self, output: &Self::Output) -> bool {
        // Vérifie si les données en sortie sont égales à celles calculées lors de la création du challenge
        self.md5HashCashOuput == *output
    }
}
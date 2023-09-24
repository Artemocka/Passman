use rand::prelude::SliceRandom;
use rand::random;

pub fn generate_password(len: usize) -> Result<String, String> {
    let mut password = vec![];
    if len < 4 {
        return Err("Password shorter than 4 symbols".to_string());
    }

    let digits = "0123456789".chars().collect::<Vec<char>>();
    let special_sybmols = "!@#$%^&*".chars().collect::<Vec<char>>();
    let lower_alphabet = "abcdefghijklmnopqrstuvwxyz".chars().collect::<Vec<char>>();
    let upper_alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect::<Vec<char>>();

    let mut pointer = 0;
    for _ in 0..len {
        pointer += 1;

        let random_symbol = match pointer % 4 {
            0 => { digits[random::<usize>() % digits.len()] }
            1 => { special_sybmols[random::<usize>() % special_sybmols.len()] }
            2 => { lower_alphabet[random::<usize>() % lower_alphabet.len()] }
            _ => { upper_alphabet[random::<usize>() % upper_alphabet.len()] }
        };
        password.push(random_symbol);
    }

    password.shuffle(&mut rand::thread_rng());

    Ok(password.iter().collect::<String>())
}
// src/password_generator.rs

use crate::utils::{LOWERCASE_LETTERS, UPPERCASE_LETTERS, NUMERALS, SPECIAL_CHARACTERS};
use rand::Rng;

pub fn generate_password(length: usize, use_special_chars: bool, use_uppercase: bool) -> Result<String, &'static str> {
    if length < 8 {
        return Err("Password length must be at least 8 characters.");
    }

    let mut character_pool = String::from(LOWERCASE_LETTERS);
    if use_uppercase {
        character_pool.push_str(UPPERCASE_LETTERS);
    }
    if use_special_chars {
        character_pool.push_str(SPECIAL_CHARACTERS);
    }
    character_pool.push_str(NUMERALS);

    let mut rng = rand::thread_rng();
    let password: String = (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..character_pool.len());
            character_pool.chars().nth(idx).unwrap()
        })
        .collect();

    Ok(password)
}

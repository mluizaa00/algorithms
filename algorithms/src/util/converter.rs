use std::ops::{Add, Mul};

pub fn char_to_int<T: Default + Mul<Output = T> + Add<Output = T> + From<u8> + Copy>(chars: &[T]) -> T {
    chars.iter().fold(T::default(), |acc, &digit| acc * T::from(10u8) + digit)
}

pub fn to_algarisms<T: From<u8> + ToString>(number: T) -> Vec<i32> {
    number.to_string()
        .chars()
        .map(|char| char.to_digit(10).unwrap_or(0) as i32)
        .collect()
}

pub fn letter_to_code<T: From<u8>>(letter: char) -> Option<T> {
    if letter.is_ascii_alphabetic() {
        let code = letter.to_ascii_uppercase() as u8 - b'A' + 1;
        return Some(T::from(code));
    } 
    
    None
}
pub fn kaprekar() {
    let mut number = 923;

    let mut tries = 0; 
    loop {
        let result = result(number);
        println!("Try: {}, Number: {}", tries, result);

        tries += 1;
        if result == number {
            println!("Found krapekar number in {} tries. Final number: {}", tries, result);
            break;
        }

        number = result;
    }
}

fn result(number: i32) -> i32 {
    let algarisms: Vec<u32> = number.to_string()
        .chars()
        .map(|char| char.to_digit(10).unwrap_or(0))
        .collect();

    let mut major = algarisms.clone();
    major.sort_by(|a, b| b.cmp(a));
    
    let mut minor = algarisms.clone();
    minor.sort();

    char_to_int(major) - char_to_int(minor)
}

fn char_to_int(algarisms: Vec<u32>) -> i32 {
    algarisms.iter()
        .fold(0, |acc, &digit| acc * 10 + digit as i32)
}
use crate::util::integer::IntegerVec;
use crate::util::integer::Integer;

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
    let algarisms: Vec<i32> = number.to_algarisms();

    let mut major = algarisms.clone();
    major.sort_by_descending();
    
    let mut minor = algarisms.clone();
    minor.sort();

    major.char_to_int() - minor.char_to_int()
}
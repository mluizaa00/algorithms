use crate::util::integer::IntegerVec;
use crate::util::integer::Integer;

pub fn kaprekar(initial_number: i32) -> Krapekar {
    let mut number = initial_number;

    let mut tries = 0; 
    let mut number_list = vec!(number; 10);
    loop {
        let result = result(number);
        number_list.push(result);
        println!("Try: {}, Number: {}", tries, result);

        tries += 1;
        if result == number {
            println!("Found krapekar number in {} tries. Final number: {}", tries, result);
            break;
        }

        number = result;
    }

    Krapekar {
        tries: tries,
        numbers: number_list
    }
}

pub struct Krapekar {
    tries: i32,
    numbers: Vec<i32>
}

fn result(number: i32) -> i32 {
    let algarisms: Vec<i32> = number.to_algarisms();

    let mut major = algarisms.clone();
    major.sort_by_descending();
    
    let mut minor = algarisms.clone();
    minor.sort();

    major.char_to_int() - minor.char_to_int()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_sequence() {
        let kaprekar = kaprekar(932);
        assert_eq!(kaprekar.tries, 4);
    }
    
}
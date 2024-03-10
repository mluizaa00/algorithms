use crate::util::sort;
use crate::util::converter;

/*
Explain the kaprekar sequence here in the comment
The kaprekar sequence is generated through a initial number of 3 algarisms
and the result of the subtraction of the major and minor number of the algarisms.

Example: 939, major = 993, minor = 399, result = 594

And for the result, the step above is repeated, until the result equals the number provided.
*/
pub fn kaprekar(initial_number: i32) -> Kaprekar {
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

    Kaprekar {
        tries: tries,
        numbers: number_list
    }
}

pub struct Kaprekar {
    tries: i32,
    numbers: Vec<i32>
}

fn result(number: i32) -> i32 {
    let algarisms: Vec<i32> = converter::to_algarisms(number);

    let mut major = algarisms.clone();
    major.sort_by(sort::descending);
    
    let mut minor = algarisms.clone();
    minor.sort();

    converter::char_to_int(&major) - converter::char_to_int(&minor)
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
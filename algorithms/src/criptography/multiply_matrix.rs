use crate::util::matrix;

pub fn criptography() {
    let mut codifying = matrix::create(2, 2);
    codifying[0][0] = 3;
    codifying[0][1] = 2;
    codifying[1][0] = 1;
    codifying[1][1] = 1;

    println!("The codifying matrix is the following one:");
    matrix::print(&codifying);

    let mut decodifying = matrix::create(2, 2);
    decodifying[0][0] = 1;
    decodifying[0][1] = -2;
    decodifying[1][0] = -1;
    decodifying[1][1] = 3;

    println!("The decodifying matrix is the following one:");
    matrix::print(&decodifying);

    let message = vec![
        vec!['F', 'U', 'V'],
        vec!['E', 'S', 'T'],
    ];

    
    let message_with_code: Vec<Vec<u32>> = message
        .iter()
        .map(|row| {
            row.iter()
                .map(|&letter| letter_to_code(letter).unwrap_or(0))
                .collect::<Vec<u32>>()
        })
        .collect();

    println!("The message with code is:");
    matrix::print(&message_with_code);

    let codified_message: Vec<Vec<u32>> = matrix::multiply(message_with_code, codifying);

    println!("The codified message is:");
    matrix::print(&codified_message);

}

fn letter_to_code(letter: char) -> Option<u32> {
    if letter.is_ascii_alphabetic() {
        return Some(letter.to_ascii_uppercase() as u32 - 'A' as u32 + 1);
    } 
    
    None
}
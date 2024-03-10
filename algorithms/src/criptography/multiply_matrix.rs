use crate::util::matrix;
use crate::util::converter;

pub fn criptography() {
    let mut codifying: Vec<Vec<i32>> = matrix::create(2, 2);
    codifying[0][0] = 3;
    codifying[0][1] = 2;
    codifying[1][0] = 1;
    codifying[1][1] = 1;

    println!("The codifying matrix is the following one:");
    matrix::print(&codifying);

    let mut decodifying: Vec<Vec<i32>> = matrix::create(2, 2);
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

    
    let message_with_code: Vec<Vec<i32>> = message
        .iter()
        .map(|row| {
            row.iter()
                .map(|&letter| converter::letter_to_code(letter).unwrap_or(1))
                .collect::<Vec<i32>>()
        })
        .collect();

    println!("The message with code is:");
    matrix::print(&message_with_code);

    let codified_message = matrix::multiply(message_with_code, codifying);

    println!("The codified message is:");
    matrix::print(&codified_message);

    let decodified_message = matrix::multiply(codified_message, decodifying);

    println!("The decodified message is:");
    matrix::print(&decodified_message);

}
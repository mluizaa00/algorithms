use std::fmt::Debug;

pub fn print<T: Debug>(matrix: &mut Vec<Vec<T>>) {
    println!("The matrix in operation is the following one:");
    for row in matrix {
        for element in row {
            print!("{:4?} ", element);
        }
        println!()
    }
}
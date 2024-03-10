use std::fmt::Debug;

pub fn print<T: Debug>(matrix: &Vec<Vec<T>>) {
    for row in matrix {
        for element in row {
            print!("{:4?} ", element);
        }
        println!()
    }
}